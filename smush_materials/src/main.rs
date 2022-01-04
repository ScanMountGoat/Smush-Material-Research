use std::{
    fs::File,
    io::BufWriter,
    path::{Path, PathBuf},
};

use clap::{App, Arg};
use rayon::prelude::*;
use xmb_lib::XmbFile;
use xmltree::EmitterConfig;

fn main() {
    // TODO: Handle args with clap?
    let input_arg = Arg::new("input")
        .index(1)
        .short('i')
        .long("input")
        .help("The source folder to search recursively for files")
        .required(true)
        .takes_value(true);
    let output_arg = Arg::new("output")
        .index(2)
        .short('o')
        .long("output")
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
                .arg(input_arg)
                .arg(output_arg),
        )
        .get_matches();

    let start = std::time::Instant::now();

    // TODO: Decompiled shaders?
    let count = match matches.subcommand().unwrap() {
        ("xmb", sub_m) => batch_convert(
            sub_m.value_of("input").unwrap(),
            sub_m.value_of("output").unwrap(),
            "xmb",
            "xml",
        ),
        _ => 0,
    };

    println!("Converted {:?} files in {:?}", count, start.elapsed());
}

fn batch_convert(
    source_folder: &str,
    destination_folder: &str,
    input_extension: &str,
    output_extension: &str,
) -> usize {
    // Make sure the output directory exists.
    if !Path::new(destination_folder).exists() {
        std::fs::create_dir(destination_folder).unwrap();
    }

    let paths: Vec<_> = globwalk::GlobWalkerBuilder::from_patterns(
        source_folder,
        &[format!("*.{}", input_extension)],
    )
    .build()
    .unwrap()
    .into_iter()
    .filter_map(Result::ok)
    .collect();

    paths.par_iter().for_each(|path| {
        match XmbFile::from_file(path.path()) {
            Ok(xmb_file) => {
                let element = xmb_file.to_xml();

                // Match the output of the original Python script where possible.
                let config = EmitterConfig::new()
                    .perform_indent(true)
                    .indent_string("    ")
                    .pad_self_closing(false);

                let output_full_path = flattened_output_path(
                    path.path(),
                    source_folder,
                    destination_folder,
                    output_extension,
                );

                let mut writer = BufWriter::new(File::create(output_full_path).unwrap());
                element.write_with_config(&mut writer, config).unwrap();
            }
            Err(e) => eprintln!("Error reading {:?}: {:?}", path.path(), e),
        }
    });

    // Assume all files converted successfully.
    paths.len()
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
