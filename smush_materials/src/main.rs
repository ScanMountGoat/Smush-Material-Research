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
                    Arg::new("output")
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
    attrs: Vec<String>,
    params: Vec<String>,
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
            let database = ShaderDatabase {
                shaders: nufx
                    .programs
                    .elements
                    .into_iter()
                    .filter(|program| program.render_pass.to_str() == Some("nu::Final"))
                    .map(|program| {
                        // We can infer information from the shader source using some basic heurstics.
                        let pixel_shader = program.shaders.pixel_shader.to_string_lossy();
                        let pixel_shader_file = Path::new(source_folder)
                            .join(&pixel_shader)
                            .with_extension("glsl");
                        let pixel_source = std::fs::read_to_string(&pixel_shader_file);

                        let vertex_shader = program.shaders.vertex_shader.to_string_lossy();
                        let vertex_shader_file = Path::new(source_folder)
                            .join(&vertex_shader)
                            .with_extension("glsl");
                        let vertex_source = std::fs::read_to_string(&vertex_shader_file);

                        // Alpha testing in Smash Ultimate is done in shader, so check for discard.
                        // There may be false positives if the discard code path is unused.
                        let discard = pixel_source
                            .as_ref()
                            .map(|source| source.contains("discard;"))
                            .unwrap_or_default();

                        let binary_file = Path::new(binary_folder)
                            .join(&pixel_shader)
                            .with_extension("bin");
                        let binary_data = BinaryData::from_file(&binary_file);

                        let params: Vec<_> = program
                            .material_parameters
                            .elements
                            .iter()
                            .map(|p| {
                                let mut name = p.parameter_name.to_string_lossy();

                                // TODO: Clean this up.
                                if name.contains("Texture") {
                                    // TODO: Check what color channels are used from textures (requires nushdb).
                                } else if name.contains("CustomVector") {
                                    if let Some(uniform) =
                                        binary_data.as_ref().ok().and_then(|data| {
                                            data.uniforms.iter().find(|u| u.name == name)
                                        })
                                    {
                                        // Check what Vector4 color channels are used.
                                        if let (Ok(vertex), Ok(pixel)) =
                                            (&vertex_source, &pixel_source)
                                        {
                                            let channels =
                                                vector4_color_channels(uniform, &vertex, &pixel);

                                            if channels != "" {
                                                name = format!("{name}.{channels}")
                                            }
                                        }
                                    }
                                }

                                name
                            })
                            .collect();

                        ShaderProgram {
                            name: program.name.to_string_lossy(),
                            discard,
                            attrs: program
                                .vertex_attributes
                                .elements
                                .iter()
                                .map(|a| a.attribute_name.to_string_lossy())
                                .collect(),
                            params,
                        }
                    })
                    .collect(),
            };

            // TODO: Make pretty printing optional.
            let json = serde_json::to_string_pretty(&database).unwrap();
            std::fs::write(output_file, json).unwrap();
        }
        Ok(_) => eprintln!("Only NUFX version 1.1 is supported"),
        Err(e) => eprintln!("Error reading {:?}: {:?}", nufx_file, e),
    }
    0
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
            let element = xmb_file.to_xml();

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
