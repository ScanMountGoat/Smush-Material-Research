use std::{
    fs::File,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

use clap::{Arg, Command};
use rayon::prelude::*;
use serde::Serialize;
use ssbh_data::{prelude::*, shdr_data::BinaryData};
use xmb_lib::XmbFile;
use xmltree::EmitterConfig;

fn main() {
    let input_arg = Arg::new("input")
        .index(1)
        .help("The source folder to search recursively for files")
        .required(true)
        .takes_value(true);
    let output_arg = Arg::new("output")
        .index(2)
        .help("The output folder")
        .required(true)
        .takes_value(true);

    let matches = Command::new("smush_materials")
        .version("0.1")
        .author("SMG")
        .about("Rendering data dumps for Smash Ultimate")
        .subcommand(
            Command::new("xmb")
                .about("Batch convert XMB to XML")
                .arg(input_arg.clone())
                .arg(output_arg.clone()),
        )
        .subcommand(
            Command::new("stage_lighting")
                .about("Batch convert stage NUANMB lighting to JSON")
                .arg(input_arg.clone())
                .arg(output_arg.clone()),
        )
        .subcommand(
            Command::new("shader_info")
                .about("Export shader info JSON")
                .arg(
                    Arg::new("nufxlb")
                        .index(1)
                        .help("The input nufxlb file")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::new("binary_folder")
                        .index(2)
                        .help("The folder of shader binaries")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::new("source_folder")
                        .index(3)
                        .help("The folder of decompiled GLSL shaders")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::new("output_json")
                        .index(4)
                        .help("The output shader info JSON")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            Command::new("shader_binaries")
                .about("Export shader binaries")
                .arg(input_arg.clone())
                .arg(output_arg.clone()),
        )
        .get_matches();

    let start = std::time::Instant::now();

    // TODO: Decompiled shaders?
    let count = match matches.subcommand().unwrap() {
        ("xmb", sub_m) => batch_convert(
            sub_m.value_of("input").unwrap(),
            sub_m.value_of("output").unwrap(),
            "*.xmb",
            "xml",
            xmb_to_xml,
        ),
        ("stage_lighting", sub_m) => batch_convert(
            sub_m.value_of("input").unwrap(),
            sub_m.value_of("output").unwrap(),
            "{light,light_}[0-9]*.nuanmb",
            "json",
            anim_data_to_json,
        ),
        ("shader_binaries", sub_m) => batch_convert(
            sub_m.value_of("input").unwrap(),
            sub_m.value_of("output").unwrap(),
            "*.nushdb",
            "bin",
            shdrs_to_bin,
        ),
        ("shader_info", sub_m) => export_shader_info(
            sub_m.value_of("nufxlb").unwrap(),
            sub_m.value_of("binary_folder").unwrap(),
            sub_m.value_of("source_folder").unwrap(),
            sub_m.value_of("output_json").unwrap(),
        ),
        _ => 0,
    };

    println!("Converted {:?} files in {:?}", count, start.elapsed());
}

#[derive(Debug, Serialize)]
struct ShaderDatabase {
    shaders: Vec<ShaderProgram>,
}

#[derive(Debug, Serialize)]
struct ShaderProgram {
    name: String,
    discard: bool,
    premultiplied: bool,
    attrs: Vec<String>,
    params: Vec<String>,
    complexity: f64,
}

fn export_shader_info(
    nufx_file: &str,
    binary_folder: &str,
    source_folder: &str,
    output_file: &str,
) -> usize {
    // Generate the shader info JSON for ssbh_wgpu.
    match ssbh_lib::formats::nufx::Nufx::from_file(nufx_file) {
        Ok(ssbh_lib::formats::nufx::Nufx::V1(nufx)) => {
            // TODO: Make excluding duplicate render pass entries optional?
            // All "SFX_PBS..." programs support all render passes.
            // Only consider one render pass per program since the entries are identical.
            let mut database = ShaderDatabase {
                shaders: nufx
                    .programs
                    .elements
                    .into_iter()
                    .filter(|program| program.render_pass.to_str() == Some("nu::Final"))
                    .map(|program| {
                        // We can infer information from the shader source using some basic heurstics.
                        let pixel_shader = program.shaders.pixel_shader.to_string_lossy();
                        let pixel_source = shader_source(source_folder, &pixel_shader);

                        let vertex_shader = program.shaders.vertex_shader.to_string_lossy();
                        let vertex_source = shader_source(source_folder, &vertex_shader);

                        // Alpha testing in Smash Ultimate is done in shader, so check for discard.
                        // There may be false positives if the discard code path is unused.
                        let discard = pixel_source
                            .as_ref()
                            .map(|source| source.contains("discard;"))
                            .unwrap_or_default();

                        let premultiplied = pixel_source
                            .as_ref()
                            .map(|source| is_premultiplied_alpha(source).unwrap_or_default())
                            .unwrap_or_default();

                        let pixel_binary_data = shader_binary_data(binary_folder, pixel_shader);
                        let vertex_binary_data = shader_binary_data(binary_folder, vertex_shader);

                        let params = material_parameters(
                            &program,
                            &pixel_binary_data,
                            &vertex_source,
                            &pixel_source,
                        );

                        let attrs = vertex_attributes(&program, vertex_binary_data, &vertex_source);

                        // TODO: Don't count comment lines?
                        // This assumes each line of code takes has the same cost.
                        // Some lines will cost more in practice like texture loads.
                        let lines_of_code =
                            pixel_source.map(|s| s.lines().count()).unwrap_or_default()
                                + vertex_source.map(|s| s.lines().count()).unwrap_or_default();

                        ShaderProgram {
                            name: program.name.to_string_lossy(),
                            discard,
                            premultiplied,
                            attrs,
                            params,
                            complexity: lines_of_code as f64,
                        }
                    })
                    .collect(),
            };

            // Normalize shader complexity so the highest complexity is 1.0.
            // Prevent a potential division by zero.
            let total_lines_of_code = database
                .shaders
                .iter()
                .map(|s| s.complexity)
                .reduce(f64::max)
                .unwrap_or_default()
                .max(1.0);

            for s in &mut database.shaders {
                s.complexity /= total_lines_of_code;
            }

            // TODO: Make pretty printing optional.
            let json = serde_json::to_string_pretty(&database).unwrap();
            std::fs::write(output_file, json).unwrap();
        }
        Ok(_) => eprintln!("Only NUFX version 1.1 is supported"),
        Err(e) => eprintln!("Error reading {:?}: {:?}", nufx_file, e),
    }
    0
}

fn shader_binary_data(
    binary_folder: &str,
    shader: String,
) -> Result<BinaryData, Box<dyn std::error::Error>> {
    let file = Path::new(binary_folder).join(&shader).with_extension("bin");
    BinaryData::from_file(&file)
}

fn shader_source(source_folder: &str, shader: &String) -> Result<String, std::io::Error> {
    let file = Path::new(source_folder).join(shader).with_extension("glsl");
    std::fs::read_to_string(&file)
}

fn vertex_attributes(
    program: &ssbh_lib::formats::nufx::ShaderProgramV1,
    vertex_binary_data: Result<BinaryData, Box<dyn std::error::Error>>,
    vertex_source: &Result<String, std::io::Error>,
) -> Vec<String> {
    program
        .vertex_attributes
        .elements
        .iter()
        .map(|a| {
            let mut name = a.attribute_name.to_string_lossy();

            // Check the vertex shader since it uses the same naming conventions.
            // TODO: This is overestimates used channels since we don't include the pixel shader.
            // Some attributes are combined before passing to the pixel shader.
            // TODO: Figure out what the pixel shader naming conventions are?
            let input_name = format!("IN_{name}");
            if let Some(location) = vertex_binary_data.as_ref().ok().and_then(|data| {
                data.inputs
                    .iter()
                    .find(|i| i.name == input_name)
                    .map(|i| i.location)
            }) {
                if let Ok(vertex) = &vertex_source {
                    let channels = input_attribute_color_channels(location, vertex);
                    if !channels.is_empty() {
                        name = format!("{name}.{channels}")
                    }
                }
            }
            name
        })
        .collect()
}

fn input_attribute_color_channels(location: i32, source: &str) -> String {
    // Assume the name is the location like "layout (location = 1) in vec4 in_attr1;"
    let mut channels = String::new();
    for component in "xyzw".chars() {
        let access = format!("in_attr{location}.{component}");
        if source.contains(&access) {
            channels.push(component);
        }
    }

    channels
}

fn material_parameters(
    program: &ssbh_lib::formats::nufx::ShaderProgramV1,
    pixel_binary_data: &Result<BinaryData, Box<dyn std::error::Error>>,
    vertex_source: &Result<String, std::io::Error>,
    pixel_source: &Result<String, std::io::Error>,
) -> Vec<String> {
    program
        .material_parameters
        .elements
        .iter()
        .map(|p| {
            let mut name = p.parameter_name.to_string_lossy();

            // TODO: Clean this up.
            if name.contains("Texture") {
                // TODO: Check what color channels are used from textures (requires nushdb).
            } else if name.contains("CustomVector") {
                if let Some(uniform) = pixel_binary_data
                    .as_ref()
                    .ok()
                    .and_then(|data| data.uniforms.iter().find(|u| u.name == name))
                {
                    // Check what Vector4 color channels are used.
                    if let (Ok(vertex), Ok(pixel)) = (vertex_source, pixel_source) {
                        let channels = vector4_color_channels(uniform, vertex, pixel);

                        if !channels.is_empty() {
                            name = format!("{name}.{channels}")
                        }
                    }
                }
            }

            name
        })
        .collect()
}

fn vector4_color_channels(
    uniform: &ssbh_data::shdr_data::Uniform,
    vertex_source: &str,
    pixel_source: &str,
) -> String {
    let mut channels = String::new();
    // The material uniform buffer is always "vec4 fp_c9_data[0x1000]".
    let vec4_index = uniform.uniform_buffer_offset / 16;
    for component in "xyzw".chars() {
        let vertex_access = format!("vp_c9_data[{vec4_index}].{component}");
        let pixel_access = format!("fp_c9_data[{vec4_index}].{component}");
        if vertex_source.contains(&vertex_access) || pixel_source.contains(&pixel_access) {
            channels.push(component);
        }
    }

    channels
}

fn batch_convert<F: Fn(&Path, PathBuf) + Send + Sync>(
    source_folder: &str,
    destination_folder: &str,
    input_pattern: &str,
    output_extension: &str,
    convert: F,
) -> usize {
    // Make sure the output directory exists.
    if !Path::new(destination_folder).exists() {
        std::fs::create_dir(destination_folder).unwrap();
    }

    let paths: Vec<_> = globwalk::GlobWalkerBuilder::from_patterns(source_folder, &[input_pattern])
        .build()
        .unwrap()
        .into_iter()
        .filter_map(Result::ok)
        .collect();

    paths.par_iter().for_each(|path| {
        let output_full_path = flattened_output_path(
            path.path(),
            source_folder,
            destination_folder,
            output_extension,
        );

        convert(path.path(), output_full_path);
    });

    // Assume all files converted successfully.
    paths.len()
}

fn xmb_to_xml(path: &Path, output_full_path: PathBuf) {
    match XmbFile::from_file(path) {
        Ok(xmb_file) => {
            let element = xmb_file.to_xml().unwrap();

            // Match the output of the original Python script where possible.
            let config = EmitterConfig::new()
                .perform_indent(true)
                .indent_string("    ")
                .pad_self_closing(false);

            let mut writer = BufWriter::new(File::create(output_full_path).unwrap());
            element.write_with_config(&mut writer, config).unwrap();
        }
        Err(e) => eprintln!("Error reading {:?}: {:?}", path, e),
    }
}

fn anim_data_to_json(path: &Path, output_full_path: PathBuf) {
    match ssbh_data::anim_data::AnimData::from_file(path) {
        Ok(anim) => {
            let mut writer = std::fs::File::create(output_full_path).unwrap();
            writer
                .write_all(serde_json::to_string_pretty(&anim).unwrap().as_bytes())
                .unwrap();
        }
        Err(e) => eprintln!("Error reading {:?}: {:?}", path, e),
    }
}

fn shdrs_to_bin(path: &Path, output: PathBuf) {
    match ssbh_lib::formats::shdr::Shdr::from_file(path) {
        Ok(ssbh_lib::formats::shdr::Shdr::V12 { shaders }) => {
            // TODO: Add a flag to just output the code portion?
            // This allows using an unmodified version of ryujinx shadertools.
            for shader in shaders.elements {
                let output = output
                    .with_file_name(shader.name.to_string_lossy())
                    .with_extension("bin");
                let mut writer = std::fs::File::create(output).unwrap();
                writer.write_all(&shader.shader_binary.elements).unwrap();
            }
        }
        Err(e) => eprintln!("Error reading {:?}: {:?}", path, e),
    }
}

fn flattened_output_path(
    path: &Path,
    source_folder: &str,
    destination_folder: &str,
    extension: &str,
) -> PathBuf {
    // Flatten directory structures by converting "source/a/b/c.in" to "destination/a_b_c.out".
    let output_file = path
        .strip_prefix(source_folder)
        .unwrap()
        .components()
        .into_iter()
        .map(|c| c.as_os_str().to_string_lossy().into_owned())
        .collect::<Vec<String>>()
        .join("_");
    Path::new(destination_folder)
        .join(output_file)
        .with_extension(extension)
}

fn is_premultiplied_alpha(source: &str) -> Option<bool> {
    // Identical shader code may have different variable names or whitespace.
    // This is known in the literature as a "type-2 code clone".
    // A proper solution would perform a structual match with a reference.
    // This is possible using an AST graph with normalized identifiers.
    // Replacing all variables with "var" allows for different variable names.
    // The argument edges should be labeled so a*b and b*a are equivalent.
    // The code matches the premultiplied reference if the graphs are isomorphic.

    // Use a simple heuristic for now based on known premultiplied shaders.
    // This basically hardcodes the graph traversal and isomorphism check.
    // Each assignment or input to an operation is an edge in the graph.
    // Checking all the shaders manually to validate this is infeasible.
    // TODO: Is it worth trying to implement a better heuristic?

    // Find the variable used to set the alpha output.
    // Assume the relevant code is in the last lines.
    let alpha_var = most_recent_assignment(source, "out_attr0.w")?;

    // The multiplied alpha should be used to initialize the var above.
    // Find the variable assigned to the alpha output.
    let alpha_assignment = most_recent_assignment(source, alpha_var)?;

    // Find the variable used for the premultiplied alpha.
    let multiplied_alpha_var = alpha_assignment.get(
        alpha_assignment.chars().position(|c| c == '(')? + 1
            ..alpha_assignment.chars().position(|c| c == ',')?,
    )?;

    // Find the variables assigned to the RGB outputs.
    // TODO: This doesn't correctly handle the BGRA condition.
    let var_r = most_recent_assignment(source, "out_attr0.x")?;
    let var_g = most_recent_assignment(source, "out_attr0.y")?;
    let var_b = most_recent_assignment(source, "out_attr0.z")?;

    // Check if the RGB outputs are multiplied by alpha.
    Some(
        is_multiplied_by_alpha(source, var_r, multiplied_alpha_var)
            && is_multiplied_by_alpha(source, var_g, multiplied_alpha_var)
            && is_multiplied_by_alpha(source, var_b, multiplied_alpha_var),
    )
}

fn is_multiplied_by_alpha(source: &str, var: &str, alpha_var: &str) -> bool {
    if let Some(assignment) = most_recent_assignment(source, var) {
        assignment.contains(&format!("* {alpha_var}"))
            || assignment.contains(&format!("{alpha_var} *"))
    } else {
        false
    }
}

fn most_recent_assignment<'a>(source: &'a str, var: &str) -> Option<&'a str> {
    source
        .lines()
        .rfind(|l| l.contains(&format!("{var} = ")))
        .and_then(|s| s.split_once("="))
        .map(|(_, s)| s.trim().trim_end_matches(';'))
}

#[cfg(test)]
mod tests {
    use crate::is_premultiplied_alpha;

    #[test]
    fn is_premultiplied() {
        let source = "temp_743 = fma(temp_736, temp_742, temp_736);
            // 0x001850: 0x5C68100000570201 Fmul
            temp_744 = temp_739 * temp_736;
            // 0x001858: 0x5C68100000570000 Fmul
            temp_745 = temp_740 * temp_736;
            // 0x001868: 0x5C68100000570402 Fmul
            temp_746 = temp_741 * temp_736;
            // 0x001870: 0xE30000000007000F Exit
            if (!s_is_bgra[0])
            {
                out_attr0.x = temp_745;
                temp_747 = true;
            }
            else
            {
                out_attr0.z = temp_745;
            }
            temp_747 = false;
            out_attr0.y = temp_744;
            if (!s_is_bgra[0])
            {
                out_attr0.z = temp_746;
                temp_748 = true;
            }
            else
            {
                out_attr0.x = temp_746;
            }
            temp_748 = false;
            out_attr0.w = temp_743;";

        assert!(is_premultiplied_alpha(source).unwrap_or_default());
    }

    #[test]
    fn pixel_source_not_premultiplied() {
        let source = "temp_733 = temp_730 * temp_680;
            // 0x0017F8: 0x49A002AC06870000 Ffma
            temp_734 = fma(temp_722, fp_c11_data[26].x, temp_732);
            // 0x001808: 0x49A002AC06870401 Ffma
            temp_735 = fma(temp_724, fp_c11_data[26].x, temp_732);
            // 0x001810: 0x49A002AC06870202 Ffma
            temp_736 = fma(temp_726, fp_c11_data[26].x, temp_732);
            // 0x001818: 0xE30000000007000F Exit
            if (!s_is_bgra[0])
            {
                out_attr0.x = temp_734;
                temp_737 = true;
            }
            else
            {
                out_attr0.z = temp_734;
            }
            temp_737 = false;
            out_attr0.y = temp_735;
            if (!s_is_bgra[0])
            {
                out_attr0.z = temp_736;
                temp_738 = true;
            }
            else
            {
                out_attr0.x = temp_736;
            }
            temp_738 = false;
            out_attr0.w = temp_733;";

        assert!(!is_premultiplied_alpha(source).unwrap_or_default());
    }

    #[test]
    fn pixel_source_not_premultiplied_empty() {
        assert!(!is_premultiplied_alpha("").unwrap_or_default());
    }
}
