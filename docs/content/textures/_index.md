+++
title = "Textures"
weight = 1
insert_anchor_links = "right"
sort_by = "weight"
+++

Textures are special image files used to render models and other effects in game such as UI elements. Compressed textures in Smash Ultimate use 
the same compression as DDS files. Textures can do more than control the color of a model and enable a variety of effects in game liked baked lighting and shadows, environment reflections, and color grading. Most textures are 2D textures and depend on a model's [texture coordinates](../vertex_attributes/texturecoordinates) to map the 2D textures to the 3D geometry of the model.

## Editing Textures
In game texture files are stored in a memory layout optimized for the Switch using a process called "swizzling" and must be converted using dedicated tools before the textures can be edited and viewed on a computer. Textures can be edited using applications like [Switch Toolbox](https://github.com/KillzXGaming/Switch-Toolbox/releases). 
Texture creation tools only support Windows at this time.