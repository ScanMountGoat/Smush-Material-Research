+++
title = "Textures"
weight = 1
insert_anchor_links = "right"
sort_by = "weight"
+++

Textures are special image files used to render models and other effects in game such as UI elements. Compressed textures in Smash Ultimate use 
the same compression as DDS files. Textures can do more than control the color of a model and enable a variety of effects in game liked baked lighting and shadows, environment reflections, and color grading. Most textures are 2D textures and depend on a model's [texture coordinates](../vertex_attributes/texturecoordinates) to map the 2D textures to the 3D geometry of the model.

## Texture Names
Texture names in Smash Ultimate are paths to the texture file. All model textures are stored in .nutexb files, so the file extension is omitted. A texture name like "def_mario_001_col" refers to the file "def_mario_001_col.nutexb" in the model's folder. The string value stored in the .numatb file is the corresponding texture's file name with no extension. This can be a file in the current directory like `asf_ashley_col` or a full path like `/common/shader/sfxPBS/default_Params`. Cube map textures can specify `#replace_cubemap` to use the current stage's reflection cube map.

## Default Textures
Any material can reference default textures stored in the `/common/shader` directories. Note that the red and yellow checkerboards that appear for invalid models are rendered in screen space using shaders and don't use any textures. For a full list of default textures, see [Default Textures](https://github.com/ScanMountGoat/Smush-Material-Research/blob/master/Material%20Parameters.md#default-textures).

## Editing Textures
In game texture files are stored in a memory layout optimized for the Switch using a process called "swizzling" and must be converted using dedicated tools before the textures can be edited and viewed on a computer. Most textures in Smash Ultimate are also compressed using compression formats like BC7 or BC6. These formats are identical to the formats used for DDS files and can be edited using standalone tools or image editor plugins. 

Not all programs that edit DDS files support more modern formats like BC7. Textures can be easily converted to a variety of formats on Windows, Linux, or MacOS using [Ultimate Tex](https://github.com/ScanMountGoat/ultimate_tex/releases). Converting nutexb textures to PNG or TIFF will allow easier editing in more programs.