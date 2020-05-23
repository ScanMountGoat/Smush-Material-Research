# Material Parameter Descriptions
Material parameters marked as unused aren't present in any of the numatb files. These parameters may still be used in material animations, however.

## UV Transforms
Each texture has an associated UV transform parameter. The values are always (1, 1, 0, 0, 0).

| Param ID | Name | Description |
| --- | --- | --- |
| FC  | UVTransform0 | Texture0 transform. |
| FD  | UVTransform1 | Texture1 transform. |
| FE  | UVTransform2 | Texture2 transform. |
| FF  | UVTransform3 | Texture3 transform. |
| 100 | UVTransform4 | Texture4 transform. |
| 101 | UVTransform5 | Texture5 transform. |
| 102 | UVTransform6 | Texture6 transform. |
| 103 | UVTransform7 | Texture7 transform. |
| 104 | UVTransform8 | Texture8 transform. |
| 105 | UVTransform9 | Texture9 transform. |
| 106 | UVTransform10 | *Unused* |
| 107 | UVTransform11 | *Unused* |
| 108 | UVTransform12 | *Unused* |
| 109 | UVTransform13 | *Unused* |
| 10A | UVTransform14 | Texture14 transform. |
| 10B | UVTransform15 | *Unused* |
| 10C | DiffuseUVTransform1 | *Unused* |
| 10D | DiffuseUVTransform2 | *Unused* |
| 10E | SpecularUVTransform1 | *Unused* |
| 10F | SpecularUVTransform 2| *Unused* |
| 110 | NormalUVTransform1 | *Unused* |
| 111 | NormalUVTransform2 | *Unused* |
| 112 | DiffuseUVTransform | *Unused* |
| 113 | SpecularUVTransform | *Unused* |
| 114 | NormalUVTransform | *Unused* |

| Field | Values|
| --- | --- |
| Unk1 | 1 |
| Unk2 | 1 |
| Unk3 | 0 |
| Unk4 | 0 |
| Unk5 | 0 |

## Blending State
Related to alpha blending in some way.  

| Param ID | Name | Description |
| ---  | --- | --- |
| 118 | BlendState0 | Used for all materials. |
| 119 | BlendState1 | *Unused* |
| 11A | BlendState2 | *Unused* |
| 11B | BlendState3 | *Unused* |
| 11C | BlendState4 | *Unused* |
| 11D | BlendState5 | *Unused* |
| 11E | BlendState6 | *Unused* |
| 11F | BlendState7 | *Unused* |
| 120 | BlendState8 | *Unused* |
| 121 | BlendState9 | *Unused* |
| 122 | BlendState10 | *Unused* |


| Field | Values|
| --- | --- |
| SrcFactor | 0 = Zero, 1 = One |
| Unk2 | 0 |
| BlendFactor1 | 0 = Zero, 1 = One, 2 = Source Alpha, 6 = One Minus Source Alpha |
| Unk4 | 0, 1 |
| Unk5 | 0 |
| BlendFactor2 | 0 = Zero, 1 = One, 2 = Source Alpha, 6 = One Minus Source Alpha |
| Unk7 | 0, 1 |
| Unk8 | 0, 1 |
| Unk9 | 0 |
| Unk10| 0 or 5 |
| Unk11| 0 |
| Unk12| 0 |

BlendFactor1 and BlendFactor2 both seem to affect the DstFactor because premultiplied alpha is used.
Unk7 and Unk8 determine if an order-independent transparency effect is used. This is common for hair meshes.  

## Rasterizer State
| Param ID | Name | Description |
| ---  | --- | --- |
| 123 | RasterizerState0 | Used for all materials. |
| 124 | RasterizerState1 | *Unused* |
| 125 | RasterizerState2 | *Unused* |
| 126 | RasterizerState3 | *Unused* |
| 127 | RasterizerState4 | *Unused* |
| 128 | RasterizerState5 | *Unused* |
| 129 | RasterizerState6 | *Unused* |
| 12A | RasterizerState7 | *Unused* |
| 12B | RasterizerState8 | *Unused* |
| 12C | RasterizerState9 | *Unused* |
| 12D | RasterizerState10 | *Unused* |

| Field | Values|
| --- | --- |
| Polygon Fill | 0 = Line, 1 = Solid |
| Cull Mode | 0 = Back, 1 = Front, 2 = Both |
| Unk3 | -1000, -100, -0.7, -0.05, 0, 1, 3, 10, 300, 400, 600, 700, 800, 1000, 5250, 9000, 10000, 20000, 999999 |
| Unk4 | -1000, -100, 0, 100 |
| Unk5 | -100, 0 |
| Unk6 | 16777217 |
| Unk7 | 0 |
| Unk8 | 0, 1599620691 |

## Float Vector 4
Stores an RGBA color or 4 float parameters.

| Param ID | Name | Description |
| --- | --- | --- |
| 98 | CustomVector0 | Affects alpha testing. X = min texture alpha, YZW = ??? |
| 99 | CustomVector1 | RGBA? Used for s65_oniyon for pikmin_planet. Set to an orange color. |
| 9A | CustomVector2 | RGBA? Used for some s65 models for pikmin_planet. Set to yellow. |
| 9B | CustomVector3 | Color multiplier for emission color. Used to brighten/darken emissive maps. Values are often higher than 1 to increase bloom. |
| 9C | CustomVector4 | RGBA? Set to white for arwinbeam_set for fox_venom. |
| 9D | CustomVector5 | RGBA? Set to white for s65_oniyon for pikmin_planet.  |
| 9E | CustomVector6 | UV Transform layer 1. |
| 9F | CustomVector7 | RGBA? Always set to (1, 1, 1, 0). |
| A0 | CustomVector8 | RGB color multiplier for the final color after diffuse, specular, etc. |
| A1 | CustomVector9 | RGBA? Always (1, 1, 1, 1) or (0.2620026, -0.6427876, -0.7198463, 0). |
| A2 | CustomVector10 | RGBA? Always (1, 0.8, 0.6, 0). |
| A3 | CustomVector11 | Used with CustomVector30 to fake subsurface scattering. RGB = SSS color |
| A4 | CustomVector12 | *Unused* |
| A5 | CustomVector13 | RGB diffuse color multiplier. May also affect alpha. Usually present and set to (1, 1, 1, 1). |
| A6 | CustomVector14 | Specular edge tint. Higher blend values increase the intensity of the effect. RGB = color, A = blend intensity. |
| A7 | CustomVector15 | RGBA? Always (0, 1000, 1, 0.2). |
| A8 | CustomVector16 | XYZW? Always (0.017, 35, 43.47, 0). Used for pikmin_planet. |
| A9 | CustomVector17 | *Unused* |
| AA | CustomVector18 | Used for sprite sheet animations. X * Y = W if the sprite sheet is full. X = column count, Y = row count, Z = frames per sprite, W = sprite count |
| AB | CustomVector19 | RGBA? Usually (1, 1, 1, 1). Alpha is always 1. |
| 13B | CustomVector20 | RGBA? RGB values are as high as 10. Alpha is 0 to 1. |
| 13C | CustomVector21 | RGBA? Values are 0 to 1. |
| 13D | CustomVector22 | RGBA? RGB values are 0 to 2. Alpha is 0 or 1. |
| 13E | CustomVector23 | RGB controls some sort of fog color. W is as high as 2400. |
| 13F | CustomVector24 | RGBA? RGB values are 0 to 1. Alpha is 0 or 1. |
| 140 | CustomVector25 | *Unused* |
| 141 | CustomVector26 | *Unused* |
| 142 | CustomVector27 | XYZW? X is between -100 and 30. YZW are between 0 and 1. |
| 143 | CustomVector28 | *Unused* |
| 144 | CustomVector29 | Used for rotating models for pilot wings, sky world, etc. X can be negative and has angle values (30, -60, or 720). Y and Z are always 0. W is always 300 or 0. |
| 145 | CustomVector30 | Fake SSS using CustomVector11. X = blend intensity, Y = smooth factor (higher is sharper), ZW = Unused |
| 146 | CustomVector31 | UV Transform layer 2. |
| 147 | CustomVector32 | UV Transform layer 3. |
| 148 | CustomVector33 | UV Transform? Used for materials with 4 base color maps (Param IDS: 66, 67, 68, 69). |
| 149 | CustomVector34 | UV Transform? Normal UV transform? |
| 14A | CustomVector35 | RGBA? A is always 0 or 1. Used in conjunction with CB and CC. |
| 14B | CustomVector36 | *Unused* |
| 14C | CustomVector37 | Some sort of rotation/warp effect used for grass. XYZ = ??? W = intensity |
| 14D | CustomVector38 | Related to 14C. XYZ= ??? W = intensity? |
| 14E | CustomVector39 | RGBA? Used in conjunction with 14C for backlit flags, vegetation, etc. |
| 14F | CustomVector40 | XYZW? Always (1, 25, 1, 25). |
| 150 | CustomVector41 | *Unused* |
| 151 | CustomVector42 | Affects specular shading. Values are 0 to 1. X = ??? Y = ??? Z = edge tint??? W = intensity??? |
| 152 | CustomVector43 | XYZW? Used for Wii Fit Trainer models for wiifit stage. Set to (0, 63, 0, 0) and (0, 25, 0, 0). |
| 153 | CustomVector44 | RGBA? Used for Wii Fit Trainer models for wiifit stage. Set to (0.1804, 0.3462, 0.1314, 1). |
| 154 | CustomVector45 | RGBA? Used for Wii Fit Trainer models for wiifit stage. Set to (0.008, 0.13, 0.02, 1). |
| 155 | CustomVector46 | RGBA? Only used for one of Bayonetta's body anim meshes. |
| 156 | CustomVector47 | Used for some stages. The color channels work the same as PRM maps. |
| 157 | CustomVector48 | *Unused* |
| 158 | CustomVector49 | *Unused* |
| 159 | CustomVector50 | *Unused* |
| 15A | CustomVector51 | *Unused* |
| 15B | CustomVector52 | *Unused* |
| 15C | CustomVector53 | *Unused* |
| 15D | CustomVector54 | *Unused* |
| 15E | CustomVector55 | *Unused* |
| 15F | CustomVector56 | *Unused* |
| 160 | CustomVector57 | *Unused* |
| 161 | CustomVector58 | *Unused* |
| 162 | CustomVector59 | *Unused* |
| 163 | CustomVector60 | *Unused* |
| 164 | CustomVector61 | *Unused* |
| 165 | CustomVector62 | *Unused* |
| 166 | CustomVector63 | *Unused* |


## Textures
*note: The uv channel used for each texture type is still largely speculation.*

| Param ID | Name | Description |
| --- | --- | --- |
| 5C | Texture0 | Base Color Map (layer 1) |
| 5D | Texture1 | Base Color Map (layer 2) |
| 5E | Texture2 | Irradiance Cubemap |
| 5F | Texture3 | Ambient Occlusion Map |
| 60 | Texture4 | Normal Map |
| 61 | Texture5 | Emissive Map (layer 1) |
| 62 | Texture6 | PRM Map |
| 63 | Texture7 | Specular Cubemap |
| 64 | Texture8 | Diffuse Cubemap |
| 65 | Texture9 | Bake Lit Map |
| 66 | Texture10 | Diffuse Map (layer 1)  |
| 67 | Texture11 | Diffuse Map (layer 2) |
| 68 | Texture12 | Diffuse Map (layer 3) |
| 69 | Texture13 | Base Color/Projection Light Map? |
| 6A | Texture14 | Emissive Map (layer 2) |
| 6B | Texture15 | *Unused* |
| 133 | Texture16 | Ink Normal Map. Used for stage ink meshes. Often uses a default white texture. |
| 134 | Texture17 | *Unused* |
| 135 | Texture18 | *Unused* |
| 136 | Texture19 | *Unused* |

### Default Textures
| Name | Description | Param IDs |
| --- | --- | --- |
| #replace_cubemap | Used as a cubemap | 5E, 63 |
| default_Normal | Used as a normal map | 60 |
| default_White | Used as a base color map | 61, 133 |
| default_black | Used as a base color map | 61 |

## Samplers
Each texture parameter has a corresponding sampler parameter. The border color for texture filtering is black.

| Param ID | Name | Description |
| --- | --- | --- |
| 6C | Sampler0 | Base Color Map (layer 1)|
| 6D | Sampler1 | Base Color Map (layer 2)|
| 6E | Sampler2 | Irradiance Cubemap |
| 6F | Sampler3 | Ambient Occlusion Map |
| 70 | Sampler4 | Nor Map |
| 71 | Sampler5 | Emi Map |
| 72 | Sampler6 | PRM Map |
| 73 | Sampler7 | Specular Cube Map |
| 74 | Sampler8 | Diffuse Cubemap |
| 75 | Sampler9 | Bake Lit Map |
| 76 | Sampler10 | Diffuse Map (layer 1) |
| 77 | Sampler11 | Diffuse Map (layer 2) |
| 78 | Sampler12 | Diffuse Map (layer 3) |
| 79 | Sampler13 | Projection Map |
| 7A | Sampler14 | Emissive Map (layer 2) |
| 7B | Sampler15 | *Unused* |
| 137 | Sampler16 | Ink Normal Map |
| 138 | Sampler17 | *Unused* |
| 139 | Sampler18 | *Unused* |
| 13A | Sampler19 | *Unused* |

| Field | Values|
| --- | --- |
| Wrap S | 0 = Repeat, 1 = Clamp To Edge, 2 = Mirrored Repeat, 3 = Clamp to Border |
| Wrap T | 0 = Repeat, 1 = Clamp To Edge, 2 = Mirrored Repeat, 3 = Clamp to Border |
| Wrap R | 0 = Repeat, 1 = Clamp To Edge, 2 = Mirrored Repeat, 3 = Clamp to Border |
| Min Filter | 0 = Nearest, 1 = Linear , 2 = Linear |
| Mag Filter | 0 = Nearest, 1 = Linear , 2 = Linear |
| Unk6 | 0, 1 , 2 |
| Unk7 | 0 |
| Unk8 | 0 |
| Unk9 | 0 |
| Unk10 | 0  |
| Unk11 | 0 |
| Unk12 | 2139095022 |
| Unk13 | -1000000, -0.5, -0.8, 0, 0.6, 1, 2  |
| Unk14 | 2, 4, 8, 16 |

## Float
| Param ID | Name | Description |
| --- | --- | --- |
| C0 | CustomFloat0 | Set to 0.997 for some models on pikmin_planet and fox_venom. |
| C1 | CustomFloat1 | Ambient occlusion map intensity ([Texture3](https://github.com/ScanMountGoat/Smush-Material-Research/blob/master/Textures.md#ambient-occlusion-maps)). A value of 0 has full effect. Values higher than 0 increase the intensity of the darkening effect. |
| C2 | CustomFloat2 | *Unused* |
| C3 | CustomFloat3 | *Unused* |
| C4 | CustomFloat4 | Values are usually 0 to 1. Used for water materials. Probably has a similar effect to a du dv map. |
| C5 | CustomFloat5 | *Unused* |
| C6 | CustomFloat6 | Set to 1 for the pikmin_planet models that use C0. |
| C7 | CustomFloat7 | *Unused* |
| C8 | CustomFloat8 | Controls albedo color tint intensity used for the specular pass. The effect is similar to the tint used for the principled shader. |
| C9 | CustomFloat9 | *Unused* |
| CA | CustomFloat10 | Specular anisotropy. A value of 0 produces isotropic specular. |
| CB | CustomFloat11 | Values range from 0 to 20000 and are lower than CC. Used in conjunction with CC and 14A. |
| CC | CustomFloat12 | Values range from -100 to 100000. Used in conjunction with CB and 14A. |
| CD | CustomFloat13 | *Unused* |
| CE | CustomFloat14 | *Unused* |
| CF | CustomFloat15 | *Unused* |
| D0 | CustomFloat16 | Set to 5.5 for the ore club eye material. |
| D1 | CustomFloat17 | Set to 0.5 for the paper mario ship and nintendogs curtain. |
| D2 | CustomFloat18 | Has very large values. Values can be as high as 3000. |
| D3 | CustomFloat19 | Used for water and glass materials. Probably some sort of IOR value. Values are 0 or higher. |

## Boolean Flags
Flags are separated into individual boolean parameters rather than being combined into a single value.

| Param ID | Name | Description |
| --- | --- | --- |
| E8 | CustomBoolean0 | Always false? Used for pikmin_planet, poke_tengam, and fox_venom. |
| E9 | CustomBoolean1 | Set to 0 for fountain of dreams water. Set to 0 for some meshes for wii fit trainer. |
| EA | CustomBoolean2 | Set to 0 for Olimar's helmet. Set to 1 for Incineroar emission mesh. Often used for meshes using additive blending. |
| EB | CustomBoolean3 | Set to true for the wufuisland arch and false for the buildings. |
| EC | CustomBoolean4 | Used in conjunction with EC. |
| ED | CustomBoolean5 | Used for meshes with scrolling textures. Used for stage morph, water, and other meshes with transparency effects. |
| EE | CustomBoolean6 | Used for meshes with scrolling textures. Set to 1 for animal crossing island water. |
| EF | CustomBoolean7 | Used for battlefield waterfalls and other meshes with transparency effects. |
| F0 | CustomBoolean8 | Set to true for bossstage_final3 and poke_kalos interval wall. |
| F1 | CustomBoolean9 | Some sort of sprite sheet scale toggle. Used in conjunction with param AA. |
| F2 | CustomBoolean10 | Set to false for spirits_floor_model\damage for each stage. |
| F3 | CustomBoolean11 | Set to false for spirits_floor_model meshes. Used for the sun shaft for fe_siege. |
| F4 | CustomBoolean12 | Set to true for fe_siege arena. False for ink meshes. |
| F5 | CustomBoolean13 | *Unused* |
| F6 | CustomBoolean14 | *Unused* |
| F7 | CustomBoolean15 | *Unused* |
| F8 | CustomBoolean16 | *Unused* |
| F9 | CustomBoolean17 | *Unused* |
| FA | CustomBoolean18 | *Unused* |
| FB | CustomBoolean19 | *Unused* |
