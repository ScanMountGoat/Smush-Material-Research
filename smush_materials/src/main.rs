use std::{
    fs::File,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

use clap::{App, Arg};
use rayon::prelude::*;
use ssbh_data::SsbhData;
use xmb_lib::XmbFile;
use xmltree::EmitterConfig;

fn main() {
    // TODO: Handle args with clap?
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

    let matches = App::new("smush_materials")
        .version("0.1")
        .author("SMG")
        .about("Rendering data dumps for Smash Ultimate")
        .subcommand(
            App::new("xmb")
                .about("Batch convert XMB to XML")
                .arg(input_arg.clone())
                .arg(output_arg.clone()),
        )
        .subcommand(
            App::new("stage_lighting")
                .about("Batch convert stage NUANMB lighting to JSON")
                .arg(input_arg.clone())
                .arg(output_arg.clone()),
        )
        .subcommand(
            App::new("shader_info")
                .about("Export shader input information")
                .arg(
                    Arg::new("input")
                        .index(1)
                        .help("The input nufxlb file")
                        .required(true)
                        .takes_value(true),
                )
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
        ("shader_info", sub_m) => export_nufxlb_shader_info(
            sub_m.value_of("input").unwrap(),
            sub_m.value_of("output").unwrap(),
        ),
        _ => 0,
    };

    println!("Converted {:?} files in {:?}", count, start.elapsed());
}

fn export_nufxlb_shader_info(input_file: &str, destination_folder: &str) -> usize {
    // Make sure the output directory exists.
    if !Path::new(destination_folder).exists() {
        std::fs::create_dir(destination_folder).unwrap();
    }

    match ssbh_lib::formats::nufx::Nufx::from_file(input_file) {
        Ok(nufx) => match nufx.programs {
            ssbh_lib::formats::nufx::ShaderPrograms::ProgramsV0(programs) => {
                for program in programs.elements {
                    println!("{}", program.name.to_string_lossy());
                    let parameters = program
                        .material_parameters
                        .elements
                        .iter()
                        .map(|p| p.parameter_name.to_string_lossy())
                        .collect::<Vec<String>>()
                        .join("\n");
                    println!("=== Material Parameters ===\n{}", parameters);
                }
            }
            ssbh_lib::formats::nufx::ShaderPrograms::ProgramsV1(programs) => {
                for program in programs.elements {
                    let parameters = program
                        .material_parameters
                        .elements
                        .iter()
                        .map(|p| p.parameter_name.to_string_lossy())
                        .collect::<Vec<String>>()
                        .join("\n");
                    let attributes = program
                        .vertex_attributes
                        .elements
                        .iter()
                        .map(|a| a.attribute_name.to_string_lossy())
                        .collect::<Vec<String>>()
                        .join("\n");

                    let output_path = Path::new(destination_folder)
                        .join(program.name.to_string_lossy() + "_info")
                        .with_extension("txt");
                    let mut writer = std::fs::File::create(output_path).unwrap();
                    writeln!(
                        &mut writer,
                        "=== Material Parameters ===\n{}\n\n=== Vertex Attributes ===\n{}",
                        parameters, attributes
                    )
                    .unwrap();
                }
            }
        },
        Err(e) => eprintln!("Error reading {:?}: {:?}", input_file, e),
    }
    0
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
