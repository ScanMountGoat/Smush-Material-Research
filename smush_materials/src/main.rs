use std::{
    collections::HashMap,
    fmt::Write,
    fs::File,
    io::{BufWriter, Cursor},
    path::{Path, PathBuf},
};

use annotation::annotate_glsl;
use anyhow::anyhow;
use clap::{Parser, Subcommand};
use database::export_shader_database;
use dependencies::source_dependencies;
use rayon::prelude::*;
use smush_shader::ShaderExprs;
use ssbh_data::{prelude::*, shdr_data::Metadata};
use ssbh_lib::formats::shdr::ShaderStage;
use xc3_shader::graph::glsl::GlslGraph;
use xmb_lib::XmbFile;
use xmltree::EmitterConfig;

use crate::database::shader_from_glsl;

mod annotation;
mod database;
mod dependencies;

// Faster than the default hash implementation.
type IndexSet<T> = indexmap::IndexSet<T, ahash::RandomState>;

/// Rendering data dumps for Smash Ultimate
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Batch convert XMB to XML
    Xmb {
        /// The source folder to search recursively for files
        input: String,
        /// The output folder
        output: String,
    },
    /// Batch convert stage NUANMB lighting to JSON
    StageLighting {
        /// The source folder to search recursively for files
        input: String,
        /// The output folder
        output: String,
    },
    /// Export decompiled shader info database as JSON.
    ShaderDatabase {
        /// The input nufxlb file
        nufxlb: String,
        ///The folder of shader binaries
        binary_folder: String,
        ///The folder of decompiled GLSL shaders
        source_folder: String,
        ///The output shader info JSON
        output_json: String,
    },
    /// Export shader binaries
    ShaderBinaries {
        /// The source folder to search recursively for files
        input: String,
        /// The output folder
        output: String,
    },
    /// Decompile shader binaries
    DecompileShaders {
        /// The source folder to search recursively for files
        input: String,
        /// The output folder
        output: String,
        /// Ryujinx.ShaderTools executable
        shader_tools: String,
    },
    /// Export shader binaries
    NushdbMetadata {
        /// The folder containing the nushdb files
        nushdb_folder: String,
        /// The output folder
        output: String,
    },
    /// Annotate parameter names in decompiled shader code
    AnnotateDecompiledShaders {
        /// The folder of decompiled GLSL shaders
        source_folder: String,
        /// The folder containing the nushdb files
        nushdb_folder: String,
        /// The output folder
        output: String,
    },
    /// Find output dependencies for the given GLSL shader program.
    GlslOutputDependencies {
        /// The input fragment GLSL file.
        frag: String,
        /// The output txt file.
        output: String,
    },
    /// Print relevant lines for a variable assignment
    GlslDependencies {
        /// The GLSL shader file
        input: String,
        /// The relevant GLSL code
        output: String,
        /// The variable name like "out_attr0.x"
        var: String,
    },
}

fn main() -> anyhow::Result<()> {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Warn)
        .init()?;

    let cli = Cli::parse();
    let start = std::time::Instant::now();

    let count = match cli.command {
        Commands::Xmb { input, output } => xmb_to_xml(input, output),
        Commands::StageLighting { input, output } => batch_convert(
            input,
            output,
            "{light,light_}[0-9]*.nuanmb",
            "json",
            anim_data_to_json,
        ),
        Commands::ShaderDatabase {
            nufxlb,
            binary_folder,
            source_folder,
            output_json,
        } => export_shader_database(nufxlb, binary_folder, source_folder, output_json),
        Commands::ShaderBinaries { input, output } => export_shader_binaries(input, output),
        Commands::DecompileShaders {
            input,
            shader_tools,
            output,
        } => decompile_shaders(input, shader_tools, output),
        Commands::NushdbMetadata {
            nushdb_folder,
            output,
        } => export_nushdb_metadata(nushdb_folder, output),
        Commands::AnnotateDecompiledShaders {
            source_folder,
            nushdb_folder,
            output,
        } => annotate_decompiled_shaders(source_folder, nushdb_folder, output),
        Commands::GlslOutputDependencies { frag, output } => glsl_output_dependencies(frag, output),
        Commands::GlslDependencies { input, output, var } => glsl_dependencies(input, output, var),
    }?;
    println!("Converted {} files in {:?}", count, start.elapsed());
    Ok(())
}

fn batch_convert<F>(
    source_folder: String,
    destination_folder: String,
    input_pattern: &str,
    output_extension: &str,
    convert: F,
) -> anyhow::Result<usize>
where
    F: Fn(&Path, PathBuf) -> anyhow::Result<()> + Send + Sync,
{
    // Make sure the output directory exists.
    if !Path::new(&destination_folder).exists() {
        std::fs::create_dir(&destination_folder)?;
    }

    let paths: Vec<_> =
        globwalk::GlobWalkerBuilder::from_patterns(&source_folder, &[input_pattern])
            .build()?
            .filter_map(Result::ok)
            .collect();

    paths.par_iter().try_for_each(|path| {
        let output_full_path = flattened_output_path(
            path.path(),
            &source_folder,
            &destination_folder,
            output_extension,
        );

        convert(path.path(), output_full_path)
    })?;

    // Assume all files converted successfully.
    Ok(paths.len())
}

fn xmb_to_xml(path: String, output_full_path: String) -> anyhow::Result<usize> {
    match XmbFile::from_file(&path) {
        Ok(xmb_file) => {
            let element = xmb_file.to_xml()?;

            // Match the output of the original Python script where possible.
            let config = EmitterConfig::new()
                .perform_indent(true)
                .indent_string("    ")
                .pad_self_closing(false);

            let mut writer = BufWriter::new(File::create(output_full_path)?);
            element.write_with_config(&mut writer, config)?;
            Ok(1)
        }
        Err(e) => {
            eprintln!("Error reading {:?}: {:?}", path, e);
            Ok(0)
        }
    }
}

fn anim_data_to_json(path: &Path, output_full_path: PathBuf) -> anyhow::Result<()> {
    use std::io::Write;
    let anim = ssbh_data::anim_data::AnimData::from_file(path)
        .map_err(|e| anyhow!("Error reading {:?}: {:?}", path, e))?;
    let mut writer = std::fs::File::create(output_full_path)?;
    writer.write_all(serde_json::to_string_pretty(&anim)?.as_bytes())?;
    Ok(())
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
        .map(|c| c.as_os_str().to_string_lossy().into_owned())
        .collect::<Vec<String>>()
        .join("_");
    Path::new(destination_folder)
        .join(output_file)
        .with_extension(extension)
}

fn export_nushdb_metadata(nushdb_folder: String, output_folder: String) -> anyhow::Result<usize> {
    // Make sure the output directory exists.
    let output_folder = Path::new(&output_folder);
    if !output_folder.exists() {
        std::fs::create_dir(output_folder)?;
    }

    let paths: Vec<_> = globwalk::GlobWalkerBuilder::from_patterns(nushdb_folder, &["*.nushdb"])
        .build()?
        .filter_map(Result::ok)
        .collect();

    // Each nushdb file can contain multiple shader entry.
    // The shader entry contains the compiled code and metadata.
    // Split into separate files to match the binary/decompiled dumps.
    let count = paths
        .par_iter()
        .filter_map(|path| ShdrData::from_file(path.path()).ok())
        .flat_map(|data| data.shaders)
        .map(|shader| {
            let output_path = output_folder.join(&shader.name).with_extension("json");
            if let Ok(json) = serde_json::to_string_pretty(&shader)
                && let Err(e) = std::fs::write(&output_path, json)
            {
                println!("Error writing to {output_path:?}: {e}");
            }
        })
        .count();
    Ok(count)
}

fn annotate_decompiled_shaders(
    source_folder: String,
    nushdb_folder: String,
    output_folder: String,
) -> anyhow::Result<usize> {
    // Make sure the output directory exists.
    let output_folder = Path::new(&output_folder);
    if !output_folder.exists() {
        std::fs::create_dir(output_folder)?;
    }

    let nushdb_paths: Vec<_> =
        globwalk::GlobWalkerBuilder::from_patterns(nushdb_folder, &["*.nushdb"])
            .build()?
            .filter_map(Result::ok)
            .collect();

    // Collect the nushdb metadata for each shader program.
    let metadata_by_name: HashMap<String, (_, _)> = nushdb_paths
        .par_iter()
        .filter_map(|path| ShdrData::from_file(path.path()).ok())
        .flat_map(|data| data.shaders)
        .map(|shader| (shader.name, (shader.shader_stage, shader.meta_data)))
        .collect();

    let shader_paths: Vec<_> =
        globwalk::GlobWalkerBuilder::from_patterns(source_folder, &["*.glsl"])
            .build()?
            .filter_map(Result::ok)
            .collect();

    // Don't count decompiled shaders that can't be annotated.
    let count = shader_paths
        .par_iter()
        .filter_map(|e| annotate_and_write_glsl(e.path(), &metadata_by_name, output_folder))
        .count();
    Ok(count)
}

fn annotate_and_write_glsl(
    glsl_path: &Path,
    metadata_by_name: &HashMap<String, (ShaderStage, Metadata)>,
    output_folder: &Path,
) -> Option<()> {
    let glsl = std::fs::read_to_string(glsl_path).ok()?;
    let shader_name = glsl_path.with_extension("");
    let shader_name = shader_name.file_name()?.to_str()?;

    let (shader_type, metadata) = metadata_by_name.get(shader_name)?;
    let annotated_glsl = annotate_glsl(glsl, shader_type, metadata)?;

    // TODO: Move this out of the function?
    let output_path = output_folder.join(shader_name).with_extension("glsl");
    if let Err(e) = std::fs::write(&output_path, annotated_glsl) {
        println!("Error writing to {output_path:?}: {e}");
    }

    Some(())
}

fn export_shader_binaries(
    source_folder: String,
    destination_folder: String,
) -> anyhow::Result<usize> {
    // Make sure the output directory exists.
    if !Path::new(&destination_folder).exists() {
        std::fs::create_dir(&destination_folder)?;
    }

    let paths: Vec<_> = globwalk::GlobWalkerBuilder::from_patterns(&source_folder, &["*.nushdb"])
        .build()?
        .filter_map(Result::ok)
        .collect();

    paths.par_iter().try_for_each(|path| {
        let output_full_path =
            flattened_output_path(path.path(), &source_folder, &destination_folder, "bin");

        shdrs_to_bin(path.path(), output_full_path)
    })?;

    // Assume all files converted successfully.
    Ok(paths.len())
}

fn shdrs_to_bin(path: &Path, output: PathBuf) -> anyhow::Result<()> {
    // Use the lower level API from ssbh_lib to access the actual code.
    // The ssbh_data implementation is still a heavy WIP.
    let ssbh_lib::formats::shdr::Shdr::V12 { shaders } =
        ssbh_lib::formats::shdr::Shdr::from_file(path)?;
    for shader in shaders.elements {
        let output = output
            .with_file_name(shader.name.to_string_lossy())
            .with_extension("bin");

        std::fs::write(output, &shader.shader_binary.elements)?;
    }
    Ok(())
}

fn decompile_shaders(
    source_folder: String,
    shader_tools: String,
    destination_folder: String,
) -> anyhow::Result<usize> {
    // Make sure the output directory exists.
    if !Path::new(&destination_folder).exists() {
        std::fs::create_dir(&destination_folder)?;
    }

    let paths: Vec<_> = globwalk::GlobWalkerBuilder::from_patterns(&source_folder, &["*.bin"])
        .build()?
        .filter_map(Result::ok)
        .collect();

    paths.par_iter().try_for_each(|path| {
        let output_path = Path::new(&destination_folder)
            .join(path.path().with_extension("glsl").file_name().unwrap());

        let mut reader = Cursor::new(std::fs::read(path.path())?);
        let shader = ssbh_data::shdr_data::ShaderBinary::read(&mut reader)?;

        // Output just the program binary to work with Ryujinx.ShaderTools.
        // TODO: Add option to strip the 80 (0x50) byte header to support dissassembly.
        let binary_file = path.path().with_extension("temp");
        std::fs::write(&binary_file, shader.program_code)?;

        let output = std::process::Command::new(&shader_tools)
            .args([&binary_file])
            .output()?
            .stdout;
        let glsl = String::from_utf8(output)?;
        std::fs::write(output_path, glsl)?;

        std::fs::remove_file(binary_file)?;
        anyhow::Result::<()>::Ok(())
    })?;

    // TODO: Don't assume all files converted successfully.
    Ok(paths.len())
}

fn glsl_output_dependencies(frag: String, output: String) -> anyhow::Result<usize> {
    // TODO: make an argument for the vertex path since vertex names don't always match up.
    let vert_glsl = std::fs::read_to_string(Path::new(&frag.replace("_PS", "_VS")))?;
    let vert = GlslGraph::parse_glsl(&vert_glsl)?;

    let frag_glsl = std::fs::read_to_string(frag)?;
    let fragment = GlslGraph::parse_glsl(&frag_glsl)?;

    let shader = shader_from_glsl(vert, fragment);
    std::fs::write(output, shader_str(&shader)?)?;
    Ok(1)
}

fn glsl_dependencies(
    input_path: String,
    output_path: String,
    var: String,
) -> anyhow::Result<usize> {
    let source = std::fs::read_to_string(input_path)?;
    let code = source_dependencies(&source, &var);
    std::fs::write(output_path, code)?;
    Ok(1)
}

fn shader_str(s: &ShaderExprs) -> anyhow::Result<String> {
    use std::fmt::Write;

    // Use a condensed representation similar to GLSL for nicer diffs.
    let mut output = String::new();
    for (k, v) in &s.output_dependencies {
        let mut visited = IndexSet::default();
        write_expr_dependencies_recursive(&mut output, s, *v, &mut visited);
        write_assignment(&mut output, s, k, *v, &mut visited);
        writeln!(&mut output)?;
    }
    if let Some(i) = s.discard_condition {
        let mut visited = IndexSet::default();
        write_expr_dependencies_recursive(&mut output, s, i, &mut visited);
        write_assignment(&mut output, s, "discard", i, &mut visited);
        writeln!(&mut output).unwrap();
    }

    Ok(output)
}

// TODO: assume the index is used exactly once because of SSA and never write var{i}?
fn write_assignment(
    output: &mut String,
    s: &ShaderExprs,
    var: &str,
    i: usize,
    old_to_new_index: &mut IndexSet<usize>,
) {
    writeln!(
        output,
        "{var} = {};",
        arg_inlined_value(s, i, old_to_new_index)
    )
    .unwrap();
}

fn write_expr_dependencies_recursive(
    output: &mut String,
    s: &ShaderExprs,
    i: usize,
    old_to_new_index: &mut IndexSet<usize>,
) {
    // Write all values that this value depends on first.
    if !old_to_new_index.contains(&i) {
        let expr = &s.exprs[i];
        match expr {
            smush_shader::OutputExpr::Value(smush_shader::Value::Texture(t)) => {
                for arg in &t.texcoords {
                    write_expr_dependencies_recursive(output, s, *arg, old_to_new_index);
                }
            }
            smush_shader::OutputExpr::Func { op, args } => {
                for arg in args {
                    write_expr_dependencies_recursive(output, s, *arg, old_to_new_index);
                }

                // Write values inline to make the output easier to read.
                let args = args_inlined_values(s, args, old_to_new_index);
                let new_index = old_to_new_index.insert_full(i).0;
                writeln!(output, "var{new_index} = {op}({});", args.join(", ")).unwrap();
            }
            smush_shader::OutputExpr::Value(_) => (),
        }
    }
}

fn args_inlined_values(
    s: &ShaderExprs,
    args: &[usize],
    old_to_new_index: &mut IndexSet<usize>,
) -> Vec<String> {
    args.iter()
        .map(|a| arg_inlined_value(s, *a, old_to_new_index))
        .collect()
}

fn arg_inlined_value(s: &ShaderExprs, i: usize, old_to_new_index: &mut IndexSet<usize>) -> String {
    match &s.exprs[i] {
        smush_shader::OutputExpr::Value(v) => {
            if let smush_shader::Value::Texture(t) = v {
                let coords: Vec<_> = args_inlined_values(s, &t.texcoords, old_to_new_index);
                format!(
                    "Texture({}, {}){}",
                    t.name,
                    coords.join(", "),
                    t.channel.map(|c| format!(".{c}")).unwrap_or_default()
                )
            } else {
                v.to_string()
            }
        }
        _ => format!("var{}", old_to_new_index.insert_full(i).0),
    }
}
