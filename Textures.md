# Texture Descriptions
Textures with links link to their corresponding page on the material research website. 
## Texture Naming Conventions
| Texture Name | Texture Type |
| --- | --- |
| _col | Base Color Map |
| _emi | Emissive Map |
| _gao | Ambient Occlusion Map |
| _lit or _bake_lit | Bake Lit Map |
| _prm | PRM Map |
| _nor | NOR Map |

## [Base Color Maps (Texture0,Texture1)](https://scanmountgoat.github.io/Smush-Material-Research/textures/col/)
RGBA albedo textures. Unlike diffuse, albedo colors specular reflections for metallic materials.
The alpha channel is used for transparency.

### Base Color Map Names
Textures in Smash Ultimate are referenced using strings in the materials. The table describes common
texture naming conventions.

| Base Color Texture Name | Usage |
| --- | --- |
| _b | Default Iris |
| _l | World of Light Enemy Iris (Red) |
| _g | Final Smash Iris (Yellow) |
| _w | Default Eye White |
| _d | World of Light Enemy Dark Iris (Purple) |
| _wd | World of Light Enemy Dark Eye White |
| alp_ | Hair Color |
| def_ | Main Color |

## [PRM Maps (Texture6)](https://scanmountgoat.github.io/Smush-Material-Research/textures/prm/)
The main PBR texture maps are packed into a single texture. This simplifies the number of textures
needed and can better take advantage of compression methods.

Metalness is usually either 0 (not metallic) or 1 (metallic). Metals have their specular colored by albedo and no diffuse contribution. Skin materials have a PRM metalness value of 1, but the material is not rendered as metallic. The PRM metalness is instead used to mask the fake subsurface scattering.

Roughness affects the roughness of specular highlights and specular cube map reflections. Smooth materials such as metals will have low roughness values.

Ambient occlusion is a form of baked ambient lighting. The ambient occlusion map affects specular and ambient diffuse.

Specular controls the reflectance at normal for non metals. The texture value is remapped to a range
of common values for non metals. A value of 1 results in a reflectance at normal of 0.2. The specular channel has no effect if metalness is set to 1 (fully metallic). Some hair materials with anisotropic specular (CustomFloat10) use the PRM alpha to rotate the specular highlights. Alpha values of 0.0 to 1.0 are mapped to angle values of 0 to 180 degrees. 

| Channel | Usage | Secondary Usage |
| --- | --- | --- |
| R | Metalness | SSS Mask  |
| G | Roughness   | |
| B | Ambient Occlusion | |
| A | Specular Reflectivity | Anisotropic Rotation |


### [Principled Shading Paper by Disney](https://static1.squarespace.com/static/58586fa5ebbd1a60e7d76d3e/t/593a3afa46c3c4a376d779f6/1496988449807/s2012_pbs_disney_brdf_notes_v2.pdf)
For more technical details, see the above paper by Disney. Its principles have become an industry standard for PBR materials. 
The PRM maps work in a similar manner. The range of specular values will need to be adjusted to match other shaders. 
The other channels should work properly without modification.

## [Emissive Maps (Texture5,Texture14)](https://scanmountgoat.github.io/Smush-Material-Research/textures/emi/)
Emissive maps ared used for glowing effects such as Samus's lights. The majority of the texture will
be black (no emission). Some materials use emission in place of a base color map for flat lighting.
This is common for retro stages and skyboxes.

## [NOR Maps (Texture4)](https://scanmountgoat.github.io/Smush-Material-Research/textures/nor/)
The RG channels are used for the XY directions of the normal map. The Z direction of the normal map
is generated for materials that use the B channel as a blend map. Some stage materials use the B channel as the Z component of the normal map.

The blend map is used for blending between materials for ink, metal box, and gold forms
(Xerneas) based on an animated threshold value. 

Cavity maps occlude specular and rim lighting but do not affect ambient diffuse lighting. The NOR alpha channel is usually set to a default value of white and isn't used by all shaders.

| Channel | Usage | Secondary Usage |
| --- | --- | --- |
| R | Normal X+  | |
| G | Normal Y+  | |
| B | Transition Blend | Normal Z+ |
| A | Cavity | |

### Note on Differences in Normal Map Color Channels
Normal maps have right handedness (Y+) in Smash Ultimate, which is used by OpenGL, Blender, and Maya. The green channel may need to be inverted for normal maps generated for other applications that default to DirectX (Y-). The correct setting will likely have "OpenGL" or "Y+" in the name. Compare with in game normal maps to ensure the channels are correct.

## Irradiance Cube Maps (Texture2)
Cube map for PBR diffuse stage lighting. Usually only 16x16. Setting the texture to `#replace_cubemap`
uses a default stage cube map.

## [Specular Cube Maps (Texture7)](https://scanmountgoat.github.io/Smush-Material-Research/textures/specularcube/)
Cube map for PBR specular stage lighting. Usually only 64x64. Setting the texture to `#replace_cubemap`
uses the stage specific cube map from the `stage_name\render\` folder.

## Ambient Occlusion Maps (Texture3)
Baked ambient occlusion used for stage models. Fighters use the PRM texture instead. The default_White texture is often used, which has no
effect. The maps can also store colored lighting information. These maps use the bake1 UVs.

## Baked Lighting Maps (Texture9)
Baked diffuse lighting and shadows for stages. Like ambient occlusion maps, these maps use the bake1 UVs. The RGB values are multiplied by 8.0, which allows storing lighting intensities 
much higher than 1.0 in a standard 8 bits per channel image at the cost of precision. The alpha channel is not scaled like the RGB channels. The ambient lighting from the baked lighting maps 
is not affected by ambient occlusion from PRM maps.  

The RGB color is added to the stage's ambient lighting after being scaled by 8.0. The alpha channel occludes the direct lighting, so an alpha value of 0.0 will have no direct lighting and 
appear to be in shadow. This produces a default baked lighting map color of (0, 0, 0, 1) to produce no effect.

| Channel | Usage |
| --- | --- |
| RGB | Baked Ambient Light |
| A | Baked Shadows |

## Projection Light Maps (Texture13)
Used as a specular lighting color multiplier for some stages. The UV coords are generated by transforming the vertex positions using a fixed projection. 

## Color Grading LUT
A 3D RGB lookup table for color grading. Most stages have a neutral lookup table texture. These
textures are also used for the various filters for screenshots (sepia, vivid, etc).

### [Color Grading LUT Examples](https://docs.unrealengine.com/en-us/Engine/Rendering/PostProcessEffects/UsingLUTs)
Smash Ultimate uses a completely different engine, but the color grading LUT textures are
functionally equivalent to the examples in the above link.  
