# Texture Naming Conventions
| Texture Name | Texture Type |
| --- | --- |
| _col | Base Color Map |
| _emi | Emissive Map |
| _gao | Ambient Occlusion Map |
| _lit or _bake_lit | Bake Lit Map |
| _prm | PRM Map |
| _nor | Normal Map |

# Base Color Maps
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

# PRM Maps
The main PBR texture maps are packed into a single texture. This simplifies the number of textures
needed and can better take advantage of compression methods.

Metalness is usually either 0 (not metallic) or 1 (metallic). Metals have their specular colored by albedo and no diffuse contribution. 
Skin materials have a metalness of 1, but they have special diffuse shading and use metalness to mask the fake subsurface scattering effect instead.

Roughness affects the roughness of specular reflections. Smooth materials such as metals will have low roughness values.

Ambient occlusion is a form of baked ambient lighting. The ambient occlusion map affects specular and ambient diffuse lighting.

Specular controls the reflectance at normal for non metals. The texture value is remapped to a range
of common values for non metals. A value of 1 results in a reflectance at normal of 0.2.

| Channel | Usage |
| --- | --- |
| R | Metalness  |
| G | Roughness   |
| B | Ambient Occlusion |
| A | Specular Reflectivity |


### [Principled Shading Paper by Disney](https://disney-animation.s3.amazonaws.com/library/s2012_pbs_disney_brdf_notes_v2.pdf)
For more technical details, see the above paper by Disney. Its principles have become an industry standard for PBR materials. 
The PRM maps work in a similar manner. The range of specular values will need to be adjusted to match other shaders. 
The other channels should work properly without modification.

# Emissive Maps
Emissive maps ared used for glowing effects such as Samus's lights. The majority of the texture will
be black (no emission). Some materials use emission in place of a base color map for flat lighting.
This is common for retro stages and skyboxes.

# Normal Maps
The RG channels are used for the XY directions of the normal map. The Z direction of the normal map
is generated.

The blend map is used for blending between materials for ink, metal box, and gold forms
(Xerneas) based on an animated threshold value.

Cavity maps are similar to ambient occlusion maps but only occlude the specular shading. These maps are usually set to a default value of white.

| Channel | Usage |
| --- | --- |
| R | Normal X+  |
| G | Normal Y+  |
| B | Transition Blend |
| A | Cavity |

# Irradiance Cube mMaps
Cube map for PBR diffuse stage lighting. Usually only 16x16. Setting the texture to #replace_cubemap
uses a default stage cube map.

# Specular Cube Maps
Cube map for PBR specular stage lighting. Usually only 64x64. Setting the texture to #replace_cubemap
uses a default stage cube map.

# Ambient Occlusion Maps
Baked ambient occlusion used for stage models. Fighters use the PRM texture instead. The default_White texture is often used, which has no
effect. The maps can also store colored lighting information. These maps use the bake1 UVs.

# Bake Lit Maps
Stores baked ambient lighting and shadows for stages. Like ambient occlusion maps, these maps use the bake1 UVs.
The RGB values are used for ambient diffuse lighting. The alpha channel stores baked shadows that
mask the stage's direct lighting.

| Channel | Usage |
| --- | --- |
| R | Ambient R  |
| G | Ambient G  |
| B | Ambient B |
| A | Baked Shadows |

# Projection Light Maps
TODO: Used for some stages. 

# Color Grading LUT
A 3D RGB lookup table for color grading. Most stages have a neutral lookup table texture. These
textures are also used for the various filters for screenshots (sepia, vivid, etc).

### [Color Grading LUT Examples](https://docs.unrealengine.com/en-us/Engine/Rendering/PostProcessEffects/UsingLUTs)
Smash Ultimate uses a completely different engine, but the color grading LUT textures are
functionally equivalent to the examples in the above link.  
