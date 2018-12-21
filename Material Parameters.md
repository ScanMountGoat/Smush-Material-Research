# Material Parameter Descriptions
## UV Transforms
The values are usually (1, 1, 0, 0, 0).

| Param ID | Name | Description |
| --- | --- | --- |
| FC  | UVTransform0 | ??? |
| FD  | UVTransform1 | ??? |
| FE  | UVTransform2 | ??? |
| FF  | UVTransform3 | ??? |
| 100 | UVTransform4 | ??? |
| 101 | UVTransform5 | ??? |
| 102 | UVTransform6 | ??? |
| 103 | UVTransform7 | ??? |
| 104 | UVTransform8 | ??? |
| 105 | UVTransform9 | ??? |
| 106 | UVTransform10 | ??? |
| 107 | UVTransform11 | ??? |
| 108 | UVTransform12 | ??? |
| 109 | UVTransform13 | ??? |
| 10A | UVTransform14 | ??? |
| 10B | UVTransform15 | ??? |
| 10C | DiffuseUVTransform1 | ??? |
| 10D | DiffuseUVTransform2 | ??? |
| 10E | SpecularUVTransform1 | ??? |
| 10F | SpecularUVTransform 2| ??? |
| 110 | NormalUVTransform1 | ??? |
| 111 | NormalUVTransform2 | ??? |
| 112 | DiffuseUVTransform | ??? |
| 113 | SpecularUVTransform | ??? |
| 114 | NormalUVTransform | ??? |

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
| 118 | BlendState0 | Used for most materials. |
| 119 | BlendState1 | ??? |
| 11A | BlendState2 | ??? |
| 11B | BlendState3 | ??? |
| 11C | BlendState4 | ??? |
| 11D | BlendState5 | ??? |
| 11E | BlendState6 | ??? |
| 11F | BlendState7 | ??? |
| 120 | BlendState8 | ??? |
| 121 | BlendState9 | ??? |
| 122 | BlendState10 | ??? |


| Field | Values|
| --- | --- |
| Unk1 | 1 |
| Unk2 | 0 |
| Unk3 | 0 or 6 |
| Unk4 | 1 or 0 |
| Unk5 | 0 |
| Unk6 | 0 or 6 |
| Unk7 | 0 |
| Unk8 | 0 |
| Unk9 | 0 |
| Unk10| 0 or 5 |
| Unk11| 0 |
| Unk12| 0 |

## Rasterizer State
| Param ID | Name | Description |
| ---  | --- | --- |
| 123 | RasterizerState0 | Used for most materials. |
| 124 | RasterizerState1 | ??? |
| 125 | RasterizerState2 | ??? |
| 126 | RasterizerState3 | ??? |
| 127 | RasterizerState4 | ??? |
| 128 | RasterizerState5 | ??? |
| 129 | RasterizerState6 | ??? |
| 12A | RasterizerState7 | ??? |
| 12B | RasterizerState8 | ??? |
| 12C | RasterizerState9 | ??? |
| 12D | RasterizerState10 | ??? |

| Field | Values|
| --- | --- |
| Unk1 | ??? |
| Unk2 | ??? |
| Unk3 | ??? |
| Unk4 | ??? |
| Unk5 | ??? |
| Unk6 | ??? |
| Unk7 | ??? |
| Unk8 | ??? |

## Float Vector 4
Stores an RGBA color or 4 float parameters.

| Param ID | Name | Description |
| --- | --- | --- |
| 98 | CustomVector0 | Usually (0.5, 0, 0, 0), (1, 0, 0, 0), or (0, 0, 0, 0). All values are used, however. 0 = texture alpha. 1 = no texture alpha. |
| 99 | CustomVector1 | RGBA? |
| 9A | CustomVector2 | RGBA? |
| 9B | CustomVector3 | RGBA? |
| 9C | CustomVector4 | XYZW? Always (0, 1000, 1, 0.2). |
| 9D | CustomVector5 | RGBA? Always (1, 1, 1, 1). |
| 9E | CustomVector6 | UV Sampler? |
| 9F | CustomVector7 | RGBA? Always (1, 1, 1, 0). |
| A0 | CustomVector8 | RGBA? A is always 0 or 1. Used for Mario Galaxy. |
| A1 | CustomVector9 | RGBA? Always (0.2620026, -0.6427876, -0.7198463, 0). |
| A2 | CustomVector10 | RGBA? Always (1, 0.8, 0.6, 0). |
| A3 | CustomVector11 | RGBA? A is always 1. Very dark or matches skin tone for skin mats (including non human characters). |
| A4 | CustomVector12 | ??? |
| A5 | CustomVector13 | RGB diffuse color multiplier. May also affect alpha. Usually present and set to (1, 1, 1, 1). |
| A6 | CustomVector14 | RGBA? Component values are usually equal. Controls some sort of rim lighting effect. |
| A7 | CustomVector15 | RGBA? Always (0, 1000, 1, 0.2). |
| A8 | CustomVector16 | XYZW? Always (0.017, 35, 43.47, 0). |
| A9 | CustomVector17 | ??? |
| AA | CustomVector18 | Used for sprite sheet animations. X = horizontal sprite count. Y = vertical sprite count. Z = frames per sprite. W = ??? |
| AB | CustomVector19 | RGBA? Usually (1, 1, 1, 1). |
| 13B | CustomVector20 | RGBA? |
| 13C | CustomVector21 | RGBA? |
| 13D | CustomVector22 | RGBA? |
| 13E | CustomVector23 | RGB controls some sort of fog color. W is as high as 2400. |
| 13F | CustomVector24 | RGBA? |
| 140 | CustomVector25 | ??? |
| 141 | CustomVector26 | ??? |
| 142 | CustomVector27 | XYZW? X has large values. YZW are always 0. |
| 143 | CustomVector28 | ??? |
| 144 | CustomVector29 | XYZW? X can be negative and has angle values (30, -60, or 720). Y and Z are always 0. W is always 300 or 0. Usually (0, 0, 0, 0). |
| 145 | CustomVector30 | RGBA? Commonly (0.5, 4, 1, 1). G is the max component. R is the min component. Used for skin materials. |
| 146 | CustomVector31 | UV Sampler? |
| 147 | CustomVector32 | UV Sampler? |
| 148 | CustomVector33 | UV Sampler? Used for materials with 4 base color maps (Param IDS: 66, 67, 68, 69). |
| 149 | CustomVector34 | UV Sampler? |
| 14A | CustomVector35 | RGBA? A is always 0 or 1. Used in conjunction with CB and CC. |
| 14B | CustomVector36 | ??? |
| 14C | CustomVector37 | XYZ values appear to be angle values. W is between 0 and 1 (usually 0.1). |
| 14D | CustomVector38 | Used for moving plants on Green Greens, Great Cave Offensive, etc. XYZ appear to be angle values and are 0, 45, 60, or 90. W = intensity? |
| 14E | CustomVector39 | RGBA? |
| 14F | CustomVector40 | XYZW? Always (1, 25, 1, 25). |
| 151 | CustomVector42 | RGBA? |
| 155 | CustomVector46 | RGBA? Only used for one of Bayonetta's body anim meshes. |
| 156 | CustomVector47 | RGBA? Used for some stages. |
| 157 | CustomVector48 | ??? |
| 158 | CustomVector49 | ??? |
| 159 | CustomVector50 | ??? |
| 15A | CustomVector51 | ??? |
| 15B | CustomVector52 | ??? |
| 15C | CustomVector53 | ??? |
| 15D | CustomVector54 | ??? |
| 15E | CustomVector55 | ??? |
| 15F | CustomVector56 | ??? |
| 160 | CustomVector57 | ??? |
| 161 | CustomVector58 | ??? |
| 162 | CustomVector59 | ??? |
| 163 | CustomVector60 | ??? |
| 164 | CustomVector61 | ??? |
| 165 | CustomVector62 | ??? |
| 166 | CustomVector63 | ??? |


## Textures
Textures may share a type due to using a different layer or UV channel.

| Param ID | Name | Description |
| --- | --- | --- |
| 5C | Texture0 | Base Color Map (layer 1) |
| 5D | Texture1 | Base Color Map (layer 2) |
| 5E | Texture2 | Irradiance Cubemap |
| 5F | Texture3 | Ambient Occlusion Map. |
| 60 | Texture4 | Normal Map |
| 61 | Texture5 | Emissive Map (layer 1) |
| 62 | Texture6 | PRM Map |
| 63 | Texture7 | Specular Cubemap |
| 64 | Texture8 | Diffuse Cubemap |
| 65 | Texture9 | Bake Lit Map |
| 66 | Texture10 | Base Color Map? |
| 67 | Texture11 | Base Color Map? |
| 68 | Texture12 | Base Color Map/Projection Light Map? |
| 69 | Texture13 | Base Color/Projection Light Map? |
| 6A | Texture14 | Emissive Map (layer 2) |
| 6B | Texture15 | ??? |
| 133 | Texture16 | Ink Normal Map. Used for stage ink meshes. Projection texture? |
| 134 | Texture17 | ??? |
| 135 | Texture18 | ??? |
| 136 | Texture19 | ??? |

### Default Textures
| Name | Description | Param IDs |
| --- | --- | --- |
| #replace_cubemap | Used as a cubemap | 5E, 63 |
| default_Normal | Used as a normal map | 60 |
| default_White | Used as a base color map | 61, 133 |
| default_black | Used as a base color map | 61 |

## Samplers
Each texture parameter has a corresponding sampler parameter. The first 6 values are often (0, 0, 0, 1, 1, 1) for 2D textures or
(1, 1, 1, 1, 1, 1) for cubemaps.

| Param ID | Name | Description |
| --- | --- | --- |
| 6C | Sampler0 | Base Color Map (5C) |
| 6D | Sampler1 | Base Color Map (5D) |
| 6E | Sampler2 | Irradiance Cubemap (5E) |
| 6F | Sampler3 | Ambient Occlusion Map (6F) |
| 70 | Sampler4 | Nor Map (60) |
| 71 | Sampler5 | Emi Map (61) |
| 72 | Sampler6 | PRM Map (62) |
| 73 | Sampler7 | Specular Cube Map (63) |
| 74 | Sampler8 | Diffuse Cubemap (74) |
| 75 | Sampler9 | Bake Lit Map (65) |
| 76 | Sampler10 | ??? |
| 77 | Sampler11 | ??? |
| 78 | Sampler12 | ??? |
| 79 | Sampler13 | Projection Map (69) |
| 7A | Sampler14 | ??? |
| 7B | Sampler15 | ??? |
| 137 | Sampler16 | Ink Normal Map (133) |
| 138 | Sampler17 | ??? |
| 139 | Sampler18 | ??? |
| 13A | Sampler19 | ??? |

| Field | Values|
| --- | --- |
| WrapS | 0 (Repeat), 1 (Clamp?), 2 (Mirrored Repeat) |
| WrapT | 0 (Repeat), 1 (Clamp?), 2 (Mirrored Repeat) |
| WrapR? | 0 (Repeat), 1 (Clamp?), 2 (Mirrored Repeat) |
| Unk4 | 1 |
| Unk5 | 1 |
| Unk6 | 1, 2  |
| Unk7 | 0 |
| Unk8 | 0 |
| Unk9 | 0 |
| Unk10 | 0  |
| Unk11 | 0 |
| Unk12 | 3.40282E+38 |
| Unk13 | 0 |
| Unk14 | 2, 4 |
| Unk15 | 0, 1, 8, 1065353216 |

## Float
| Param ID | Name | Description |
| --- | --- | --- |
| C0 | CustomFloat0 | Set to 0.997 for some models on pikmin_planet and fox_venom. |
| C1 | CustomFloat1 | Usually set to 0. Set to 0.3 for stc_bg_chikei_set for xeno_gaur. Non zero for kirby_fountain models. |
| C4 | CustomFloat4 | Values are usually 0 to 1. Used for water materials. |
| C6 | CustomFloat6 | Set to 1 for the pikmin_planet models that use C0. |
| C8 | CustomFloat8 | Present for most fighters. |
| CA | CustomFloat10 | Used for anisotropic hair materials. |
| CB | CustomFloat11 | Values range from 0 to 20000 and are lower than CC. Used in conjunction with CC and 14A. |
| D0 | CustomFloat16 | Set to 5.5 for the ore club eye material. |
| D1 | CustomFloat17 | Set to 0.5 for the paper mario ship and nintendogs curtain. |
| D2 | CustomFloat18 | Has very large values. Values can be as high as 3000. |
| D3 | CustomFloat19 | Used for water and glass materials. Values are 1 or higher. |

## Boolean Flags
Flags are separated into individual boolean parameters rather than being combined into a single value.

| Param ID | Name | Description |
| --- | --- | --- |
| E8 | CustomBoolean0 | ??? |
| E9 | CustomBoolean1 | Set to 0 for fountain of dreams water. Seems to enables/disable specular occlusion for fighters. |
| EA | CustomBoolean2 | Set to 0 for Olimar's helmet. Set to 1 for Incineroar emission mesh. Usually not present. |
| EB | CustomBoolean3 | ??? |
| EC | CustomBoolean4 | ??? |
| ED | CustomBoolean5 | ??? |
| EE | CustomBoolean6 | Set to 1 for animal crossing island water. |
| EF | CustomBoolean7 | ??? |
| F0 | CustomBoolean8 | ??? |
| F1 | CustomBoolean9 | Some sort of sprite sheet scale toggle. Used in conjunction with param AA. |
| F2 | CustomBoolean10 | ??? |
| F3 | CustomBoolean11 | ??? |
| F4 | CustomBoolean12 | ??? |
| F5 | CustomBoolean13 | ??? |
| F6 | CustomBoolean14 | ??? |
| F7 | CustomBoolean15 | ??? |
| F8 | CustomBoolean16 | ??? |
| F9 | CustomBoolean17 | ??? |
| FA | CustomBoolean18 | ??? |
| FB | CustomBoolean19 | ??? |
