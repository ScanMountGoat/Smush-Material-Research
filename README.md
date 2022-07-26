# Smash Ultimate Material Research
A collection of dumps, scripts, and markdown files for research into lighting, shading, rendering, and materials for Smash Ultimate.
The markdown files contain information on researched values for materials and can be viewed directly
in the browser. Please submit an issue or pull request for any incorrect or missing information.

# [Website](https://scanmountgoat.github.io/Smush-Material-Research/)
The repository has a website for tutorials and real time demos. If any pages don't load properly or one of the demos is messed up, report the bug in [issues](https://github.com/ScanMountGoat/Smush-Material-Research/issues). 

# Generating Data - WIP
Most of the dumps are generated using the `smush_materials` Rust project. The Rust toolchain can be installed from https://www.rust-lang.org/tools/install. Build the project with `cargo build --release` from the `smush_materials` folder. For commandline usage, run `cargo run --release -- --help`.    

## SQLite Material Database
Most of the data is stored in an SQLite database generated by the program in the [Smush-Material-DB](https://github.com/ScanMountGoat/Smush-Material-DB) repo. The repo contains instructions for how to build and run the tool. The database can be used with the provided python scripts for data visualization even without any SQL knowledge.

## XMB to XML
`cargo run --release -- xmb <ARC root folder> <export folder>`

## Stage Light NUANMB JSON
`cargo run --release -- stage_lighting <ARC root folder> <export folder>`

## Shader Info
`cargo run --release -- shader_info <nuc2effectlibrary.nufxlb> <export folder>`  

## Shader Database
The JSON dump of the Nufx file can be converted to an SQLite database for more efficient querying.  
`python create_shader_db.py nuc2effectlibrary.json nufx.db`  
The file contains duplicate entries for each render pass (_opaque, _sort, etc). These entries can be removed by adding the `--remove_duplicates` flag.  
`python create_shader_db.py nuc2effectlibrary.json nufx.db --remove_duplicates`   

## Decompiled Shaders - WIP
First extract the shader binaries from the .nushdb files since each file contains multiple shaders.  
`cargo run --release -- shader_binaries <render folder> <binary export folder>`

Now the shaders can be decompiled using Ryujinx's ShaderTools. This currently requires a slightly modified build to start from offset 2896 of the binary.  
`python batch_decompile_shaders.py <ShaderTools.exe> <binary export folder> <export folder>`  

The `shaders_discard.txt` file contains the shader labels without their render pass tags for shader programs that may support alpha testing. 
This list is based on searching the decompiled shader dump for shaders with the `discard;` keyword. There may be some false positives in this list since it's possible 
for the shader code to contain `discard;` and not perform alpha testing based on the model and texture alpha. There are unlikely to be any missing shaders since 
alpha testing is always done in game using shader code rather than through a graphics API call to enable or disable alpha testing.

# Additional Tools
[ssbh_lib](https://github.com/ultimate-research/ssbh_lib) - Contains the `ssbh_lib_json` and `ssbh_data_json` executables for editing various rendering related file types as JSON  
[xmb_lib](https://github.com/ultimate-research/xmb_lib) - Contains the `xmb` executable that can be used to convert XMB files to and from XML