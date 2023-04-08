use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufWriter, Cursor, Write},
    path::{Path, PathBuf},
};

use clap::{Arg, ArgAction, Command};
use rayon::prelude::*;
use serde::Serialize;
use ssbh_data::{prelude::*, shdr_data::MetaData};
use ssbh_lib::formats::shdr::ShaderType;
use xmb_lib::XmbFile;
use xmltree::EmitterConfig;

const VEC4_SIZE: i32 = 16;

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
                .arg(output_arg.clone())
                .arg(
                    Arg::new("code")
                        .action(ArgAction::SetTrue)
                        .long("code")
                        .help("Only output the compiled program code"),
                ),
        )
        .subcommand(
            Command::new("nushdb_metadata")
                .about("Export nushdb shader metadata JSON")
                .arg(
                    Arg::new("nushdb_folder")
                        .index(1)
                        .help("The folder containing the nushdb files")
                        .required(true)
                        .takes_value(true),
                )
                .arg(output_arg.clone()),
        )
        .subcommand(
            Command::new("annotate_decompiled_shaders")
                .about("Annotate parameter names in decompiled shader code")
                .arg(
                    Arg::new("source_folder")
                        .index(1)
                        .help("The folder of decompiled GLSL shaders")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::new("nushdb_folder")
                        .index(2)
                        .help("The folder containing the nushdb files")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::new("output")
                        .index(3)
                        .help("The output folder")
                        .required(true)
                        .takes_value(true),
                ),
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
        ("shader_binaries", sub_m) => export_shader_binaries(
            sub_m.value_of("input").unwrap(),
            sub_m.value_of("output").unwrap(),
            sub_m.get_flag("code"),
        ),
        ("shader_info", sub_m) => export_shader_info(
            sub_m.value_of("nufxlb").unwrap(),
            sub_m.value_of("binary_folder").unwrap(),
            sub_m.value_of("source_folder").unwrap(),
            sub_m.value_of("output_json").unwrap(),
        ),
        ("nushdb_metadata", sub_m) => export_nushdb_metadata(
            sub_m.value_of("nushdb_folder").unwrap(),
            sub_m.value_of("output").unwrap(),
        ),
        ("annotate_decompiled_shaders", sub_m) => annotate_decompiled_shaders(
            sub_m.value_of("source_folder").unwrap(),
            sub_m.value_of("nushdb_folder").unwrap(),
            sub_m.value_of("output").unwrap(),
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
    receives_shadow: bool,
    sh: bool,
    lighting: bool,
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
                            &vertex_binary_data,
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

                        // Texture15 is always the shadow map texture.
                        // Shaders with Texture15 also have IN_ShadowMap.
                        // Just check if the shadow map is present for now.
                        // Checking shadow map usage requires mapping decompiled texture handles to uniforms.
                        let receives_shadow = pixel_binary_data
                            .as_ref()
                            .map(|p| p.uniforms.iter().any(|u| u.name == "Texture15"))
                            .unwrap_or_default();

                        // Spherical harmonic ambient lighting is passed from the vertex shader.
                        let sh = pixel_binary_data
                            .as_ref()
                            .map(|p| p.inputs.iter().any(|i| i.name == "IN_shLighting"))
                            .unwrap_or_default();

                        // Some models with baked lighting don't use the light set.
                        // A negative offset means that the buffer doesn't contain the uniform.
                        let lighting = pixel_binary_data
                            .as_ref()
                            .map(|p| {
                                p.uniforms.iter().any(|u| {
                                    u.name == "lightDirColor1" && u.uniform_buffer_offset != -1
                                })
                            })
                            .unwrap_or_default();

                        ShaderProgram {
                            name: program.name.to_string_lossy(),
                            discard,
                            premultiplied,
                            receives_shadow,
                            sh,
                            lighting,
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
) -> Result<MetaData, Box<dyn std::error::Error>> {
    let file = Path::new(binary_folder).join(&shader).with_extension("bin");
    MetaData::from_file(&file)
}

fn shader_source(source_folder: &str, shader: &String) -> Result<String, std::io::Error> {
    let file = Path::new(source_folder).join(shader).with_extension("glsl");
    std::fs::read_to_string(&file)
}

fn vertex_attributes(
    program: &ssbh_lib::formats::nufx::ShaderProgramV1,
    vertex_binary_data: Result<MetaData, Box<dyn std::error::Error>>,
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
    vertex_binary_data: &Result<MetaData, Box<dyn std::error::Error>>,
    pixel_binary_data: &Result<MetaData, Box<dyn std::error::Error>>,
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
                // Check what Vector4 color channels are used.
                let pixel_channels =
                    vector4_color_channels(&name, "fp_c9_data", pixel_binary_data, pixel_source)
                        .unwrap_or_default();
                let vertex_channels =
                    vector4_color_channels(&name, "vp_c9_data", vertex_binary_data, vertex_source)
                        .unwrap_or_default();

                // Channels may be accessed in either shader.
                let channels: String = "xyzw"
                    .chars()
                    .into_iter()
                    .enumerate()
                    .filter(|(i, _)| pixel_channels[*i] || vertex_channels[*i])
                    .map(|(_, c)| c)
                    .collect();

                if !channels.is_empty() {
                    name = format!("{name}.{channels}")
                }
            }

            name
        })
        .collect()
}

fn vector4_color_channels(
    name: &str,
    buffer_name: &str,
    binary_data: &Result<MetaData, Box<dyn Error>>,
    source: &Result<String, std::io::Error>,
) -> Option<[bool; 4]> {
    let uniform = binary_data
        .as_ref()
        .ok()?
        .uniforms
        .iter()
        .find(|u| u.name == name)?;

    // Check what Vector4 color channels are used.
    Some(vector4_color_channels_from_source(
        uniform,
        source.as_ref().ok()?,
        buffer_name,
    ))
}

fn vector4_color_channels_from_source(
    uniform: &ssbh_data::shdr_data::Uniform,
    source: &str,
    buffer_name: &str,
) -> [bool; 4] {
    let mut channels = [false; 4];
    let vec4_index = uniform.uniform_buffer_offset / VEC4_SIZE;
    for (channel, component) in channels.iter_mut().zip("xyzw".chars()) {
        let access = format!("{buffer_name}[{vec4_index}].{component}");
        if source.contains(&access) {
            *channel = true;
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

fn export_nushdb_metadata(nushdb_folder: &str, output_folder: &str) -> usize {
    // Make sure the output directory exists.
    let output_folder = Path::new(output_folder);
    if !output_folder.exists() {
        std::fs::create_dir(output_folder).unwrap();
    }

    let paths: Vec<_> = globwalk::GlobWalkerBuilder::from_patterns(nushdb_folder, &["*.nushdb"])
        .build()
        .unwrap()
        .into_iter()
        .filter_map(Result::ok)
        .collect();

    // Each nushdb file can contain multiple shader entry.
    // The shader entry contains the compiled code and metadata.
    // Split into separate files to match the binary/decompiled dumps.
    paths
        .par_iter()
        .filter_map(|path| ShdrData::from_file(path.path()).ok())
        .flat_map(|data| data.shaders)
        .map(|shader| {
            let output_path = output_folder.join(&shader.name).with_extension("json");
            if let Ok(json) = serde_json::to_string_pretty(&shader) {
                if let Err(e) = std::fs::write(&output_path, json) {
                    println!("Error writing to {output_path:?}: {e}");
                }
            }
        })
        .count()
}

fn annotate_decompiled_shaders(
    source_folder: &str,
    nushdb_folder: &str,
    output_folder: &str,
) -> usize {
    // Make sure the output directory exists.
    let output_folder = Path::new(output_folder);
    if !output_folder.exists() {
        std::fs::create_dir(output_folder).unwrap();
    }

    let nushdb_paths: Vec<_> =
        globwalk::GlobWalkerBuilder::from_patterns(nushdb_folder, &["*.nushdb"])
            .build()
            .unwrap()
            .into_iter()
            .filter_map(Result::ok)
            .collect();

    // Collect the nushdb metadata for each shader program.
    let metadata_by_name: HashMap<String, (_, _)> = nushdb_paths
        .par_iter()
        .filter_map(|path| ShdrData::from_file(path.path()).ok())
        .flat_map(|data| data.shaders)
        .map(|shader| (shader.name, (shader.shader_type, shader.meta_data)))
        .collect();

    let shader_paths: Vec<_> =
        globwalk::GlobWalkerBuilder::from_patterns(source_folder, &["*.glsl"])
            .build()
            .unwrap()
            .into_iter()
            .filter_map(Result::ok)
            .collect();

    // Don't count decompiled shaders that can't be annotated.
    shader_paths
        .par_iter()
        .filter_map(|e| annotate_and_write_glsl(e.path(), &metadata_by_name, output_folder))
        .count()
}

fn annotate_and_write_glsl(
    glsl_path: &Path,
    metadata_by_name: &HashMap<String, (ShaderType, MetaData)>,
    output_folder: &Path,
) -> Option<()> {
    let glsl = std::fs::read_to_string(glsl_path).ok()?;
    let shader_name = glsl_path.with_extension("");
    let shader_name = shader_name.file_name()?.to_str()?;

    let (shader_type, metadata) = metadata_by_name.get(shader_name)?;
    let annotated_glsl = annotate_glsl(glsl, shader_type, &metadata)?;

    // TODO: Move this out of the function?
    let output_path = output_folder.join(&shader_name).with_extension("glsl");
    if let Err(e) = std::fs::write(&output_path, annotated_glsl) {
        println!("Error writing to {output_path:?}: {e}");
    }

    Some(())
}

fn annotate_glsl(glsl: String, shader_type: &ShaderType, metadata: &MetaData) -> Option<String> {
    // TODO: The goal is to eventually annotate texture names as well.
    let mut annotated_glsl = glsl;

    annotate_input_outputs(&mut annotated_glsl, shader_type, metadata);
    annotate_uniforms(&mut annotated_glsl, metadata, shader_type)?;

    Some(annotated_glsl)
}

fn annotate_input_outputs(
    annotated_glsl: &mut String,
    shader_type: &ShaderType,
    metadata: &MetaData,
) {
    match shader_type {
        ShaderType::Vertex => {
            // Vertex inputs have explicit locations.
            for input in &metadata.inputs {
                let glsl_name = format!("in_attr{}", input.location);
                *annotated_glsl = annotated_glsl.replace(&glsl_name, &input.name);
            }
            // Vertex outputs appear in order.
            // TODO: Skip builtins like gl_Position?
            for (i, output) in metadata.outputs.iter().enumerate() {
                let glsl_name = format!("out_attr{i}");
                *annotated_glsl = annotated_glsl.replace(&glsl_name, &output.name);
            }
        }
        ShaderType::Fragment => {
            // Fragment inputs appear in order.
            for (i, input) in metadata.inputs.iter().enumerate() {
                let glsl_name = format!("in_attr{i}");
                *annotated_glsl = annotated_glsl.replace(&glsl_name, &input.name);
            }
            // Fragment outputs have explicit locations.
            for output in &metadata.outputs {
                let glsl_name = format!("out_attr{}", output.location);
                *annotated_glsl = annotated_glsl.replace(&glsl_name, &output.name);
            }
        }
        _ => (),
    }
}

fn annotate_uniforms(
    annotated_glsl: &mut String,
    metadata: &MetaData,
    shader_type: &ShaderType,
) -> Option<()> {
    // TODO: is there a way to determine the binding index for a uniform buffer?
    // Just use known slots for buffer names for now.
    let per_material_slot = metadata
        .buffers
        .iter()
        .position(|b| b.name == "nuPerMaterial")?;

    let buffer = match shader_type {
        ShaderType::Vertex => Some("vp_c9_data"),
        ShaderType::Geometry => None,
        ShaderType::Fragment => Some("fp_c9_data"),
        ShaderType::Compute => None,
    }?;

    let mut assignments = Vec::new();

    for u in metadata
        .uniforms
        .iter()
        .filter(|u| u.buffer_slot == per_material_slot as i32)
    {
        if u.uniform_buffer_offset != -1 {
            // Assume all uniform buffers are of the form "vec4 data[0x1000];".
            let vec4_index = u.uniform_buffer_offset / VEC4_SIZE;
            // TODO: Also convert bools and floats?
            match u.data_type {
                ssbh_data::shdr_data::DataType::Boolean => {
                    // Booleans offsets are accessed as floats and converted to booleans.
                    let assignment = annotate_float(annotated_glsl, u, buffer, vec4_index);
                    assignments.push(assignment);
                }
                ssbh_data::shdr_data::DataType::Float => {
                    // Float offsets point to one of the vec4 components.
                    let assignment = annotate_float(annotated_glsl, u, buffer, vec4_index);
                    assignments.push(assignment);
                }
                ssbh_data::shdr_data::DataType::Vector4 => {
                    let assignment = annotate_vector4(annotated_glsl, u, buffer, vec4_index);
                    assignments.push(assignment);
                }
                _ => (),
            }
        }
    }

    add_variable_assignments(annotated_glsl, assignments);

    Some(())
}

fn annotate_vector4(
    annotated_glsl: &mut String,
    u: &ssbh_data::shdr_data::Uniform,
    buffer: &str,
    vec4_index: i32,
) -> String {
    let pattern = format!("{buffer}[{vec4_index}]");
    *annotated_glsl = annotated_glsl.replace(&pattern, &u.name);

    format!("vec4 {} = {pattern};", &u.name)
}

fn annotate_float(
    annotated_glsl: &mut String,
    u: &ssbh_data::shdr_data::Uniform,
    buffer: &str,
    vec4_index: i32,
) -> String {
    let component_offset = u.uniform_buffer_offset - vec4_index * VEC4_SIZE;
    let component = match component_offset {
        0 => "x",
        4 => "y",
        8 => "z",
        12 => "w",
        _ => todo!(),
    };
    let pattern = format!("{buffer}[{vec4_index}].{component}");
    *annotated_glsl = annotated_glsl.replace(&pattern, &u.name);

    format!("float {} = {pattern};", &u.name)
}

fn add_variable_assignments(annotated_glsl: &mut String, assignments: Vec<String>) {
    // TODO: Find a more robust way to add the variable assignments to the start of main.
    let mut lines: Vec<String> = annotated_glsl.lines().map(Into::into).collect();
    let assignment_index = lines
        .iter()
        .position(|l| l.contains("void main()"))
        .unwrap_or_default()
        + 2;
    for assignment in &assignments {
        let indented = format!("    {assignment}");
        lines.insert(assignment_index, indented);
    }
    *annotated_glsl = lines.join("\n");
}

fn export_shader_binaries(source_folder: &str, destination_folder: &str, just_code: bool) -> usize {
    // Make sure the output directory exists.
    if !Path::new(destination_folder).exists() {
        std::fs::create_dir(destination_folder).unwrap();
    }

    let paths: Vec<_> = globwalk::GlobWalkerBuilder::from_patterns(source_folder, &["*.nushdb"])
        .build()
        .unwrap()
        .into_iter()
        .filter_map(Result::ok)
        .collect();

    paths.par_iter().for_each(|path| {
        let output_full_path =
            flattened_output_path(path.path(), source_folder, destination_folder, "bin");

        shdrs_to_bin(path.path(), output_full_path, just_code);
    });

    // Assume all files converted successfully.
    paths.len()
}

fn shdrs_to_bin(path: &Path, output: PathBuf, just_code: bool) {
    // Use the lower level API from ssbh_lib to access the actual code.
    // The ssbh_data implementation is still a heavy WIP.
    match ssbh_lib::formats::shdr::Shdr::from_file(path) {
        Ok(ssbh_lib::formats::shdr::Shdr::V12 { shaders }) => {
            for shader in shaders.elements {
                let output = output
                    .with_file_name(shader.name.to_string_lossy())
                    .with_extension("bin");
                let mut writer = std::fs::File::create(output).unwrap();

                if just_code {
                    // The actual assembly code starts after the header.
                    // This allows using an unmodified Ryujinx.ShaderTools or a dissassembler.
                    let mut reader = Cursor::new(&shader.shader_binary.elements);
                    let binary = ssbh_data::shdr_data::ShaderBinary::read(&mut reader).unwrap();
                    // TODO: Strip 0x50 bytes of header at the beginning for easier dissassembly.
                    // TODO: This also helps with alignment requirements in certain tools.
                    writer.write_all(&binary.program_code).unwrap();
                } else {
                    writer.write_all(&shader.shader_binary.elements).unwrap();
                };
            }
        }
        Err(e) => eprintln!("Error reading {:?}: {:?}", path, e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;
    use ssbh_data::shdr_data::{Attribute, Buffer, DataType, Uniform};

    #[test]
    fn is_premultiplied() {
        let source = indoc! {"
            temp_743 = fma(temp_736, temp_742, temp_736);
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
            out_attr0.w = temp_743;"
        };

        assert!(is_premultiplied_alpha(source).unwrap_or_default());
    }

    #[test]
    fn pixel_source_not_premultiplied() {
        let source = indoc! {"
            temp_733 = temp_730 * temp_680;
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
            out_attr0.w = temp_733;
        "};

        assert!(!is_premultiplied_alpha(source).unwrap_or_default());
    }

    #[test]
    fn pixel_source_not_premultiplied_empty() {
        assert!(!is_premultiplied_alpha("").unwrap_or_default());
    }

    #[test]
    fn annotate_glsl_fragment() {
        let glsl = indoc! {"
            layout (location = 0) in vec4 in_attr0;
            layout (location = 1) in vec4 in_attr1;
            layout (location = 0) out vec4 out_attr0;

            void main() 
            {
                vec4 varVec4 = fp_c9_data[0];
                float varFloat = fp_c9_data[5].y;
                float varBool = 0 != floatBitsToInt(fp_c9_data[9].z);
                out_attr0 = in_attr0 + in_attr1;
            }
        "}
        .to_string();

        let metadata = MetaData {
            buffers: vec![Buffer {
                name: "nuPerMaterial".to_string(),
                used_size_in_bytes: 0,
                uniform_count: 0,
                unk4: 0,
                unk5: 0,
            }],
            uniforms: vec![
                Uniform {
                    name: "CustomVector0".to_string(),
                    data_type: DataType::Vector4,
                    buffer_slot: 0,
                    uniform_buffer_offset: 0,
                    unk11: -1,
                },
                Uniform {
                    name: "CustomFloat0".to_string(),
                    data_type: DataType::Float,
                    buffer_slot: 0,
                    uniform_buffer_offset: 5 * VEC4_SIZE + 4,
                    unk11: -1,
                },
                Uniform {
                    name: "CustomBoolean0".to_string(),
                    data_type: DataType::Boolean,
                    buffer_slot: 0,
                    uniform_buffer_offset: 9 * VEC4_SIZE + 8,
                    unk11: -1,
                },
            ],
            inputs: vec![
                Attribute {
                    name: "attribute0".to_string(),
                    data_type: DataType::Vector4,
                    location: -1,
                },
                Attribute {
                    name: "attribute1".to_string(),
                    data_type: DataType::Vector4,
                    location: -1,
                },
            ],
            outputs: vec![Attribute {
                name: "outAttribute0".to_string(),
                data_type: DataType::Vector4,
                location: 0,
            }],
        };

        pretty_assertions::assert_eq!(
            indoc! {"
                layout (location = 0) in vec4 attribute0;
                layout (location = 1) in vec4 attribute1;
                layout (location = 0) out vec4 outAttribute0;

                void main() 
                {
                    float CustomBoolean0 = fp_c9_data[9].z;
                    float CustomFloat0 = fp_c9_data[5].y;
                    vec4 CustomVector0 = fp_c9_data[0];
                    vec4 varVec4 = CustomVector0;
                    float varFloat = CustomFloat0;
                    float varBool = 0 != floatBitsToInt(CustomBoolean0);
                    outAttribute0 = attribute0 + attribute1;
                }"
            },
            annotate_glsl(glsl, &ShaderType::Fragment, &metadata).unwrap()
        );
    }

    #[test]
    fn annotate_glsl_vertex() {
        let glsl = indoc! {"
            layout (location = 2) in vec4 in_attr2;
            layout (location = 7) in vec4 in_attr7;
            layout (location = 0) out vec4 out_attr0;
            layout (location = 1) out vec4 out_attr1;

            void main() 
            {
                vec4 varVec4 = vp_c9_data[0];
                float varFloat = vp_c9_data[5].y;
                float varBool = 0 != floatBitsToInt(vp_c9_data[9].z);
                out_attr0 = in_attr2;
                out_attr1 = in_attr7;
            }
        "}
        .to_string();

        let metadata = MetaData {
            buffers: vec![Buffer {
                name: "nuPerMaterial".to_string(),
                used_size_in_bytes: 0,
                uniform_count: 0,
                unk4: 0,
                unk5: 0,
            }],
            uniforms: vec![
                Uniform {
                    name: "CustomVector0".to_string(),
                    data_type: DataType::Vector4,
                    buffer_slot: 0,
                    uniform_buffer_offset: 0,
                    unk11: -1,
                },
                Uniform {
                    name: "CustomFloat0".to_string(),
                    data_type: DataType::Float,
                    buffer_slot: 0,
                    uniform_buffer_offset: 5 * VEC4_SIZE + 4,
                    unk11: -1,
                },
                Uniform {
                    name: "CustomBoolean0".to_string(),
                    data_type: DataType::Boolean,
                    buffer_slot: 0,
                    uniform_buffer_offset: 9 * VEC4_SIZE + 8,
                    unk11: -1,
                },
            ],
            inputs: vec![
                Attribute {
                    name: "attribute2".to_string(),
                    data_type: DataType::Vector4,
                    location: 2,
                },
                Attribute {
                    name: "attribute7".to_string(),
                    data_type: DataType::Vector4,
                    location: 7,
                },
            ],
            outputs: vec![
                Attribute {
                    name: "outAttribute0".to_string(),
                    data_type: DataType::Vector4,
                    location: 0,
                },
                Attribute {
                    name: "outAttribute1".to_string(),
                    data_type: DataType::Vector4,
                    location: 1,
                },
            ],
        };

        pretty_assertions::assert_eq!(
            indoc! {"
                layout (location = 2) in vec4 attribute2;
                layout (location = 7) in vec4 attribute7;
                layout (location = 0) out vec4 outAttribute0;
                layout (location = 1) out vec4 outAttribute1;

                void main() 
                {
                    float CustomBoolean0 = vp_c9_data[9].z;
                    float CustomFloat0 = vp_c9_data[5].y;
                    vec4 CustomVector0 = vp_c9_data[0];
                    vec4 varVec4 = CustomVector0;
                    float varFloat = CustomFloat0;
                    float varBool = 0 != floatBitsToInt(CustomBoolean0);
                    outAttribute0 = attribute2;
                    outAttribute1 = attribute7;
                }"
            },
            annotate_glsl(glsl, &ShaderType::Vertex, &metadata).unwrap()
        );
    }
}
