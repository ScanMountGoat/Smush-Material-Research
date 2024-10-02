+++
title = "Introduction"
weight = 0
insert_anchor_links = "right"
+++
The tutorials, demos, and other documentation for rendering in Smash Ultimate can be accessed from the navigation menu.
Many of these pages contain interactive demos using WebGL, which should work on most computers, phones, and tablets. The [Col](../textures/col/), [PRM](../textures/prm/), and [NOR](../textures/nor/) texture pages are a good starting point. Click the search icon to search any of the pages on this site.

The main [Github repository](https://github.com/ScanMountGoat/Smush-Material-Research) contains documentation pages that serve as quick references for material parameters, texture types, etc. The documentation pages are updated more frequently than the website but contain less detailed explanations. 

## Conventions
The terms "texture" and "map" both refer to the images used to color a model to adjust its shading. The term "texture" is the technical term used for programming 3D and 2D graphics. The term "map" is usually combined with a specific usage of a texture like "albedo map" or "ambient occlusion map". The guides use both terms interchangeably. 

RGB colors can use integer values like 255, 0, 128 or floating point values like 1.0, 0.0, 0.5. Textures typically only allow values between 0 and 255, which converts to a floating point range of 0.0 to 1.0.

CustomVector parameters have 4 parameter values. These values are floating point numbers like 0.0, -200.4, or 1.3333. 
The 4 values will be referred to as XYZW or RGBA. For example, CustomVector11.g and CustomVector11.y both refer to the second value for CustomVector11. CustomVector11.w and CustomVector11.a both refer to the fourth value. Values can be written as (r, g, b, a) or (x, y, z,w ). Mixing and matching representations in the same context will be avoided where possible. 

The four values of a texture can also be written using the "texture.b" or "texture.z" notation depending on if it makes more sense to refer to the channels as RGBA colors or not. Phrases like "PRM alpha", "PRM.a" or "PRM.w" all refer to the fourth channel of the PRM texture. Not all alpha channels are used for transparency, but the convention in image editing programs is to show the fourth channel as alpha regardless of its usage.

## Tools 
- [SSBH Editor](https://github.com/ScanMountGoat/ssbh_editor/releases) - view, edit, and validate models
- [ssbh_lib_json/ssbh_data_json](https://github.com/ultimate-research/ssbh_lib/releases) - command line tools for editing model formats as text files
- [smash-ultimate-blender](https://github.com/ssbucarlos/smash-ultimate-blender) - import and export models with Blender
- [Switch Toolbox](https://github.com/KillzXGaming/Switch-Toolbox) - edit bntx and nutexb files (Windows only)
- [Ultimate Tex](https://github.com/ScanMountGoat/ultimate_tex/releases) - batch convert texture files like bntx, dds, or nutexb
- [Smush LUT](https://github.com/ScanMountGoat/Smush-LUT) - edit color_grading_lut.nutexb files

## Support
Report any bugs in [issues](https://github.com/ScanMountGoat/Smush-Material-Research/issues) if one of the links is broken or a page isn't rendering correctly.

## Credits
- [three.js](https://threejs.org/) - WebGL rendering library
- [Zola](https://www.getzola.org/) - static site generator
- [book](https://www.getzola.org/themes/book/) - theme used to style this site