+++
title = "Materials"
weight = 3
insert_anchor_links = "right"
sort_by = "weight"
+++

Models are rendered on the GPU using compiled programs called shaders. Shaders take multiple inputs such as 
textures, geometry, lighting data, and other visual parameters to calculate the rendered color or "shading" for an object on screen. 
The materials contained in the model.numatb helps determine some of the inputs passed to the shader program for drawing. The additional inputs for drawing come from the mesh parameters and other rendering configuration files. The required inputs for a shader program are hardcoded into the program code, so it is important that the material correctly specifies all the inputs needed by the shader listed in the shader label field of the material. Applications like SSBH Editor will give warnings if the material does not specify all required inputs for the shader.

Materials are assigned to meshes in a model using the model.numdlb file. Each mesh can only have one material active at a time, but each material can be assigned to any number of meshes. Some special effects like the ditto transformation use additional material files like metamon_model.numatb. The game will skip rendering any meshes that don't have a material assigned, which can be used to make a mesh invisible.

## Editing Materials
The easiest way to edit materials is using [SSBH Editor's Matl Editor](https://github.com/ScanMountGoat/ssbh_editor/wiki/Matl-Editor). The Matl Editor in SSBH Editor gives realtime feedback in the viewport for how most material edits will look in game. SSBH Editor also has additional features for improving the editing experience like saving and applying material presets and checking for common errors during editing.

Materials can also be edited using ssbh_lib_json or ssbh_data_json. Both programs are commandline programs that convert numatb files to and from JSON. JSON is a text format and can be edited in any text editor like notepad++, VSCode, etc. Simply drag the numatb file onto the executable, edit the JSON, and drag the JSON onto the executable to make a new numatb. If no file appears in any of these steps, try running the program from command prompt to check the error output. ssbh_data_json is generally recommended over ssbh_lib_json since ssbh_data_json uses a more concise and readable format. Both programs are available for Windows on the [ssbh_lib releases](https://github.com/ultimate-research/ssbh_lib/releases).

For writing Python scripts to edit materials, see [ssbh_data_py](https://github.com/ScanMountGoat/ssbh_data_py).