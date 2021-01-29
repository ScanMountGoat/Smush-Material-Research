# Material Parameter Descriptions
Material parameters marked as unused aren't present in any of the numatb files. These parameters may still be used in material animations, however.

## Table of Contents
- [UV Transforms](#UV-Transforms)
- [Blend State](#Blend-State)
- [Rasterizer State](#Rasterizer-State)
- [Custom Vectors](#Custom-Vectors)
- [Textures](#Textures)
- [Samplers](#Samplers)
- [Custom Floats](#Custom-Floats)
- [Custom Booleans](#Custom-Booleans)

## UV Transforms
Each texture has an associated UV transform parameter. The values are always (1, 1, 0, 0, 0).

| Param ID | Name | Description |
| --- | --- | --- |
| 0xFC  | UVTransform0 | Texture0 transform. |
| 0xFD  | UVTransform1 | Texture1 transform. |
| 0xFE  | UVTransform2 | Texture2 transform. |
| 0xFF  | UVTransform3 | Texture3 transform. |
| 0x100 | UVTransform4 | Texture4 transform. |
| 0x101 | UVTransform5 | Texture5 transform. |
| 0x102 | UVTransform6 | Texture6 transform. |
| 0x103 | UVTransform7 | Texture7 transform. |
| 0x104 | UVTransform8 | Texture8 transform. |
| 0x105 | UVTransform9 | Texture9 transform. |
| 0x106 | UVTransform10 | *Unused* |
| 0x107 | UVTransform11 | *Unused* |
| 0x108 | UVTransform12 | *Unused* |
| 0x109 | UVTransform13 | *Unused* |
| 0x10A | UVTransform14 | Texture14 transform. |
| 0x10B | UVTransform15 | *Unused* |
| 0x10C | DiffuseUVTransform1 | *Unused* |
| 0x10D | DiffuseUVTransform2 | *Unused* |
| 0x10E | SpecularUVTransform1 | *Unused* |
| 0x10F | SpecularUVTransform 2| *Unused* |
| 0x110 | NormalUVTransform1 | *Unused* |
| 0x111 | NormalUVTransform2 | *Unused* |
| 0x112 | DiffuseUVTransform | *Unused* |
| 0x113 | SpecularUVTransform | *Unused* |
| 0x114 | NormalUVTransform | *Unused* |

| Field | Values|
| --- | --- |
| Unk1 | 1 |
| Unk2 | 1 |
| Unk3 | 0 |
| Unk4 | 0 |
| Unk5 | 0 |

## Blend State
Alpha blending related parameters. Shaders for materials that use alpha blending typically use premultiplied alpha. 
The blending operation is always addition for both color and alpha.  

Alpha to coverage is an order-independent transparency effect similar to alpha testing. A certain pattern of pixels are drawn at full opacity or discarded based on the alpha. The model will look grainy but not have any of the sorting issues associated with alpha blending.

| Param ID | Name | Description |
| ---  | --- | --- |
| 0x118 | BlendState0 | Used for all materials. |
| 0x119 | BlendState1 | *Unused* |
| 0x11A | BlendState2 | *Unused* |
| 0x11B | BlendState3 | *Unused* |
| 0x11C | BlendState4 | *Unused* |
| 0x11D | BlendState5 | *Unused* |
| 0x11E | BlendState6 | *Unused* |
| 0x11F | BlendState7 | *Unused* |
| 0x120 | BlendState8 | *Unused* |
| 0x121 | BlendState9 | *Unused* |
| 0x122 | BlendState10 | *Unused* |

| Field | Values|
| --- | --- |
| Source Color | blend factor values |
| Unk2 | 0 |
| Destination Color | blend factor values |
| Unk4 | 0, 1 |
| Unk5 | 0 |
| Unk6 | 0, 1, 2, 6 |
| Alpha to Coverage | 1 = enabled, 0 = disabled |
| Unk8 | 0, 1 |
| Unk9 | 0 |
| Unk10| 0 or 5 |

| Blend Factor | Description |
| --- | --- |
| 0 | Zero |
| 1 | One |
| 2 | Source Alpha |
| 3 | Destination Alpha |
| 4 | Source Color |
| 5 | Destination Color |
| 6 | One Minus Source Alpha |
| 7 | One Minus Destination Alpha |
| 8 | One Minus Source Color |
| 9 | One Minus Destination Color |
| 10 | Source Alpha Saturate |


## Rasterizer State
| Param ID | Name | Description |
| ---  | --- | --- |
| 0x123 | RasterizerState0 | Used for all materials. |
| 0x124 | RasterizerState1 | *Unused* |
| 0x125 | RasterizerState2 | *Unused* |
| 0x126 | RasterizerState3 | *Unused* |
| 0x127 | RasterizerState4 | *Unused* |
| 0x128 | RasterizerState5 | *Unused* |
| 0x129 | RasterizerState6 | *Unused* |
| 0x12A | RasterizerState7 | *Unused* |
| 0x12B | RasterizerState8 | *Unused* |
| 0x12C | RasterizerState9 | *Unused* |
| 0x12D | RasterizerState10 | *Unused* |

| Field | Values|
| --- | --- |
| Polygon Fill | 0 = Line, 1 = Solid |
| Cull Mode | 0 = Back, 1 = Front, 2 = None |
| Depth Bias | Floating point depth offset. Values can be negative. |
| Unk4 | -1000, -100, 0, 100 |
| Unk5 | -100, 0 |
| Unk6 | 16777217 |

## Custom Vectors
Stores an RGBA color or 4 float parameters.

| Param ID | Name | Description |
| --- | --- | --- |
| 0x98 | CustomVector0 | Affects alpha testing. X = min texture alpha, YZW = ??? |
| 0x99 | CustomVector1 | RGBA? Used for s65_oniyon for pikmin_planet. Set to an orange color. |
| 0x9A | CustomVector2 | RGBA? Used for some s65 models for pikmin_planet. Set to yellow. |
| 0x9B | CustomVector3 | Color multiplier for emission color. Used to brighten/darken emissive maps. Values are often higher than 1 to increase bloom. |
| 0x9C | CustomVector4 | RGBA? Set to white for arwinbeam_set for fox_venom. |
| 0x9D | CustomVector5 | RGBA? Set to white for s65_oniyon for pikmin_planet.  |
| 0x9E | CustomVector6 | UV Transform layer 1. |
| 0x9F | CustomVector7 | RGBA? Always set to (1, 1, 1, 0). |
| 0xA0 | CustomVector8 | RGB color multiplier for the final color after diffuse, specular, etc. |
| 0xA1 | CustomVector9 | RGBA? Always (1, 1, 1, 1) or (0.2620026, -0.6427876, -0.7198463, 0). |
| 0xA2 | CustomVector10 | RGBA? Always (1, 0.8, 0.6, 0). |
| 0xA3 | CustomVector11 | Used with CustomVector30 to fake subsurface scattering. RGB = SSS color |
| 0xA4 | CustomVector12 | *Unused* |
| 0xA5 | CustomVector13 | RGB diffuse color multiplier. May also affect alpha. Usually present and set to (1, 1, 1, 1). |
| 0xA6 | CustomVector14 | Rim lighting color. RGB = color, A = blend factor. |
| 0xA7 | CustomVector15 | RGBA? Always (0, 1000, 1, 0.2). |
| 0xA8 | CustomVector16 | XYZW? Always (0.017, 35, 43.47, 0). Used for pikmin_planet. |
| 0xA9 | CustomVector17 | *Unused* |
| 0xAA | CustomVector18 | Used for sprite sheet animations. X * Y = W if the sprite sheet is full. X = column count, Y = row count, Z = frames per sprite, W = sprite count |
| 0xAB | CustomVector19 | RGBA? Usually (1, 1, 1, 1). Alpha is always 1. |
| 0x13B | CustomVector20 | RGBA? RGB values are as high as 10. Alpha is 0 to 1. |
| 0x13C | CustomVector21 | RGBA? Values are 0 to 1. |
| 0x13D | CustomVector22 | RGBA? RGB values are 0 to 2. Alpha is 0 or 1. |
| 0x13E | CustomVector23 | RGB controls some sort of fog color. W is as high as 2400. |
| 0x13F | CustomVector24 | RGBA? RGB values are 0 to 1. Alpha is 0 or 1. |
| 0x140 | CustomVector25 | ??? |
| 0x141 | CustomVector26 | *Unused* |
| 0x142 | CustomVector27 | Controls distance fog. X = Intensity, YZW = ??? |
| 0x143 | CustomVector28 | *Unused* |
| 0x144 | CustomVector29 | Used for rotating models for pilot wings, sky world, etc. X can be negative and has angle values (30, -60, or 720). Y and Z are always 0. W is always 300 or 0. |
| 0x145 | CustomVector30 | Fake SSS using CustomVector11. X = blend factor, Y = diffuse shading smooth factor (higher is sharper), ZW = Unused |
| 0x146 | CustomVector31 | UV Transform layer 2. |
| 0x147 | CustomVector32 | UV Transform layer 3. |
| 0x148 | CustomVector33 | UV Transform? Used for materials with 4 base color maps (Param IDS: 66, 67, 68, 69). |
| 0x149 | CustomVector34 | UV Transform? Normal UV transform? |
| 0x14A | CustomVector35 | RGBA? A is always 0 or 1. Used in conjunction with CB and CC. |
| 0x14B | CustomVector36 | *Unused* |
| 0x14C | CustomVector37 | Some sort of rotation/warp effect used for vegetation. XYZ = ???, W = ??? |
| 0x14D | CustomVector38 | Some sort of rotation/warp effect used for vegetation. XYZ = ???, W = ??? |
| 0x14E | CustomVector39 | Some sort of rotation/warp effect used for vegetation. XYZ = ???, W = ??? |
| 0x14F | CustomVector40 | XYZW? Always (1, 25, 1, 25). |
| 0x150 | CustomVector41 | *Unused* |
| 0x151 | CustomVector42 | Controls some sort of rim lighting by smoothing between Z and W. XY = *Unused*, Z = smooth start, W = smooth end |
| 0x152 | CustomVector43 | XYZW? Used for Wii Fit Trainer models for wiifit stage. Set to (0, 63, 0, 0) and (0, 25, 0, 0). |
| 0x153 | CustomVector44 | RGBA? Used for Wii Fit Trainer models for wiifit stage. Set to (0.1804, 0.3462, 0.1314, 1). |
| 0x154 | CustomVector45 | RGBA? Used for Wii Fit Trainer models for wiifit stage. Set to (0.008, 0.13, 0.02, 1). |
| 0x155 | CustomVector46 | RGBA? Only used for one of Bayonetta's body anim meshes. |
| 0x156 | CustomVector47 | Used for some stages. The color channels work the same as PRM maps. |
| 0x157 | CustomVector48 | *Unused* |
| 0x158 | CustomVector49 | *Unused* |
| 0x159 | CustomVector50 | *Unused* |
| 0x15A | CustomVector51 | *Unused* |
| 0x15B | CustomVector52 | *Unused* |
| 0x15C | CustomVector53 | *Unused* |
| 0x15D | CustomVector54 | *Unused* |
| 0x15E | CustomVector55 | *Unused* |
| 0x15F | CustomVector56 | *Unused* |
| 0x160 | CustomVector57 | *Unused* |
| 0x161 | CustomVector58 | *Unused* |
| 0x162 | CustomVector59 | *Unused* |
| 0x163 | CustomVector60 | *Unused* |
| 0x164 | CustomVector61 | *Unused* |
| 0x165 | CustomVector62 | *Unused* |
| 0x166 | CustomVector63 | *Unused* |


## Textures
| Param ID | Name | UV Attribute | Description |
| --- | --- | --- | --- |
| 0x5C | Texture0 | map1 | Col Map (layer 1) |
| 0x5D | Texture1 | uvSet | Col Map (layer 2) |
| 0x5E | Texture2 | *Cube Map* | Irradiance Cube Map |
| 0x5F | Texture3 | bake1 | Ambient Occlusion Map |
| 0x60 | Texture4 | map1 | NOR Map |
| 0x61 | Texture5 | map1 | Emissive Map (layer 1) |
| 0x62 | Texture6 | map1 | PRM Map |
| 0x63 | Texture7 | *Cube Map* | Specular Cube Map |
| 0x64 | Texture8 | *Cube Map* | Diffuse Cube Map |
| 0x65 | Texture9 | bake1 | Baked Lighting Map |
| 0x66 | Texture10 | map1 |  Diffuse Map (layer 1)  |
| 0x67 | Texture11 | uvSet |  Diffuse Map (layer 2) |
| 0x68 | Texture12 | ??? |  Diffuse Map (layer 3) |
| 0x69 | Texture13 | *Projection Coords* |  Projection Light Map |
| 0x6A | Texture14 | uvSet |  Emissive Map (layer 2) |
| 0x6B | Texture15 | *Unused* |  *Unused* |
| 0x133 | Texture16 | ??? | Ink Normal Map. Used for stage ink meshes. Often uses a default white texture. |
| 0x134 | Texture17 | *Unused* | *Unused* |
| 0x135 | Texture18 | *Unused* | *Unused* |
| 0x136 | Texture19 | *Unused* | *Unused* |

### Default Textures
| Name | Description | Param IDs |
| --- | --- | --- |
| #replace_cubemap | Used as a cubemap | 5E, 63 |
| default_Normal | Used as a normal map | 60 |
| default_White | Used as a base color map | 61, 133 |
| default_black | Used as a base color map | 61 |

## Samplers
Each texture parameter has a corresponding sampler parameter. The wrap modes should be set to Clamp to Border for border color to have an effect. 

| Param ID | Name | Description |
| --- | --- | --- |
| 0x6C | Sampler0 | Base Color Map (layer 1)|
| 0x6D | Sampler1 | Base Color Map (layer 2)|
| 0x6E | Sampler2 | Irradiance Cubemap |
| 0x6F | Sampler3 | Ambient Occlusion Map |
| 0x70 | Sampler4 | Nor Map |
| 0x71 | Sampler5 | Emi Map |
| 0x72 | Sampler6 | PRM Map |
| 0x73 | Sampler7 | Specular Cube Map |
| 0x74 | Sampler8 | Diffuse Cubemap |
| 0x75 | Sampler9 | Bake Lit Map |
| 0x76 | Sampler10 | Diffuse Map (layer 1) |
| 0x77 | Sampler11 | Diffuse Map (layer 2) |
| 0x78 | Sampler12 | Diffuse Map (layer 3) |
| 0x79 | Sampler13 | Projection Map |
| 0x7A | Sampler14 | Emissive Map (layer 2) |
| 0x7B | Sampler15 | *Unused* |
| 0x137 | Sampler16 | Ink Normal Map |
| 0x138 | Sampler17 | *Unused* |
| 0x139 | Sampler18 | *Unused* |
| 0x13A | Sampler19 | *Unused* |

| Field | Values|
| --- | --- |
| Wrap S | 0 = Repeat, 1 = Clamp To Edge, 2 = Mirrored Repeat, 3 = Clamp to Border |
| Wrap T | 0 = Repeat, 1 = Clamp To Edge, 2 = Mirrored Repeat, 3 = Clamp to Border |
| Wrap R | 0 = Repeat, 1 = Clamp To Edge, 2 = Mirrored Repeat, 3 = Clamp to Border |
| Min Filter | 0 = Nearest, 1 = Linear Mipmap Linear + ???, 2 = Linear Mipmap Linear + ??? |
| Mag Filter | 0 = Nearest, 1 = Linear + ???, 2 = Linear + ??? |
| Texture Filtering Type | 0 = No Anisotropic Filtering + ???, 1 = No Anisotropic Filtering + ???, 2 = Anisotropic Filtering |
| Border Color R | Floating point value between 0.0 and 1.0. May be gamma corrected. |
| Border Color B | Floating point value between 0.0 and 1.0. May be gamma corrected. |
| Border Color G | Floating point value between 0.0 and 1.0. May be gamma corrected. |
| Border Color A | Floating point value between 0.0 and 1.0. |
| Unk11 | 0 |
| Unk12 | Always 0x7F7FFFFF (127, 127, 255, 255). This may or may not be used. |
| LOD Bias | Values are floating point and clamped between `-MAX_LOD` and `+MAX_LOD` where `MAX_LOD = Mipmap Count`  |
| Max Anisotropy | 0 = 1x, 2 = 2x, 4 = 4x, 8 = 16x, 16 = 128x |

## Custom Floats
| Param ID | Name | Description |
| --- | --- | --- |
| 0xC0 | CustomFloat0 | Set to 0.997 for some models on pikmin_planet and fox_venom. |
| 0xC1 | CustomFloat1 | Ambient occlusion map intensity ([Texture3](https://github.com/ScanMountGoat/Smush-Material-Research/blob/master/Textures.md#ambient-occlusion-maps-texture3)). A value of 0 has full effect. Values higher than 0 increase the intensity of the darkening effect. |
| 0xC2 | CustomFloat2 | *Unused* |
| 0xC3 | CustomFloat3 | *Unused* |
| 0xC4 | CustomFloat4 | Controls the intensity of a UV distortion effect using the normal map. Used for water materials. |
| 0xC5 | CustomFloat5 | *Unused* |
| 0xC6 | CustomFloat6 | Set to 1 for the pikmin_planet models that use C0. |
| 0xC7 | CustomFloat7 | *Unused* |
| 0xC8 | CustomFloat8 | Controls albedo color tint intensity used for the specular pass. The effect is similar to the tint used for the principled shader. |
| 0xC9 | CustomFloat9 | ??? |
| 0xCA | CustomFloat10 | Specular anisotropy. A value of 0 produces isotropic specular. |
| 0xCB | CustomFloat11 | Values range from 0 to 20000 and are lower than CC. Used in conjunction with CC and 14A. |
| 0xCC | CustomFloat12 | Values range from -100 to 100000. Used in conjunction with CB and 14A. |
| 0xCD | CustomFloat13 | *Unused* |
| 0xCE | CustomFloat14 | *Unused* |
| 0xCF | CustomFloat15 | *Unused* |
| 0xD0 | CustomFloat16 | Set to 5.5 for the ore club eye material. |
| 0xD1 | CustomFloat17 | Set to 0.5 for the paper mario ship and nintendogs curtain. |
| 0xD2 | CustomFloat18 | Used with CustomVector37,38,39 for some sort of rotation/warp effect for vegetation. |
| 0xD3 | CustomFloat19 | Controls angle fade in addition to the specular IOR used for environment reflections. Higher values are less transparent and have more intense reflections. |

## Custom Booleans
Flags are separated into individual boolean parameters rather than being combined into a single value.

| Param ID | Name | Description |
| --- | --- | --- |
| 0xE8 | CustomBoolean0 | Always false? Used for pikmin_planet, poke_tengam, and fox_venom. |
| 0xE9 | CustomBoolean1 | PRM specular override. True = PRM alpha, False = 0.16 |
| 0xEA | CustomBoolean2 | Alpha Override. True = set alpha to 0.0, False = preserve alpha |
| 0xEB | CustomBoolean3 | Enables/disables the specular light contribution. True = enabled, False = disabled. |
| 0xEC | CustomBoolean4 | Enables/disables the specular cube map contribution. True = enabled, False = disabled. |
| 0xED | CustomBoolean5 | Used for meshes with scrolling textures. Used for stage morph, water, and other meshes with transparency effects. |
| 0xEE | CustomBoolean6 | Used for meshes with scrolling textures. Set to 1 for animal crossing island water. |
| 0xEF | CustomBoolean7 | Used for battlefield waterfalls and other meshes with transparency effects. |
| 0xF0 | CustomBoolean8 | Set to true for bossstage_final3 and poke_kalos interval wall. |
| 0xF1 | CustomBoolean9 | Can be used with CustomVector18. Scales down UVs per the XY values in said vector and repurposes Z value to represent which sprite in the sheet to use for that frame in an animation. |
| 0xF2 | CustomBoolean10 | Set to false for spirits_floor_model\damage for each stage. |
| 0xF3 | CustomBoolean11 | Affects texture blending of emission textures. Set to false for spirits_floor_model meshes. Used for the sun shaft for fe_siege. |
| 0xF4 | CustomBoolean12 | Set to true for fe_siege arena. False for ink meshes. |
| 0xF5 | CustomBoolean13 | *Unused* |
| 0xF6 | CustomBoolean14 | *Unused* |
| 0xF7 | CustomBoolean15 | *Unused* |
| 0xF8 | CustomBoolean16 | *Unused* |
| 0xF9 | CustomBoolean17 | *Unused* |
| 0xFA | CustomBoolean18 | *Unused* |
| 0xFB | CustomBoolean19 | *Unused* |
