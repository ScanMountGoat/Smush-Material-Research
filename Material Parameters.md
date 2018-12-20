# Material Parameters
| Data Type | Description | Param IDs |
| --- | --- | --- |
| 1 | Float | C8 D3 C6 CA D0 C4 C1 D2 CB CC C0 D1 |
| 2 | Boolean | E9 F3 EB EC EA EE ED EF F1 F4 E8 F2 F0 |
| 5 | Vector 4 | 98 9B 146 A0 A5 A6 145 144 A3 142 9E 9F 156 151 155 13C 147 AA 149 AB 13B 14C 14D 14E 13E 13D 14A A2 9C A1 99 A7 A8 9A 9D 14F 13F 148 |
| 10 | Struct |  FC FE 100 102 103 101 FD 105 10A FF |
| 11 | Struct | 118 |
| 12 | Struct | 123 |
| B | Texture | 6A 60 61 62 63 5C 5D 5E 64 67 66 5F 65 133 68 69 |
| E | Struct? |  7A 70 71 72 73 6C 6E 6D 74 77 76 6F 75 137 78 79 |

# Material Param Descriptions
### Struct Type 10
The values are always (1, 1, 0, 0, 0).

| Field | Values|
| --- | --- |
| Unk1 | 1 |
| Unk2 | 1 |
| Unk3 | 0 |
| Unk4 | 0 |
| Unk5 | 0 |

| Param ID | Name | Description |
| --- | --- | --- |
| FC | UVTransform0 | ??? |
| FD | UVTransform1 | ??? |
| FE | UVTransform2 | ??? |
| FF | UVTransform3  | ??? |  
| 100 | UVTransform4 |  ??? |
| 101 | UVTransform5 |  ??? |
| 102 | UVTransform6 |  ??? |
| 103 | UVTransform7 |  ??? |
| 105 | UVTransform9 |  ??? |
| 10A | UVTransform14 |  ??? |

### BlendState0 (Param ID 118)
Possibly related to alpha blending in some way.  

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

### RasterizerState0 (Param ID 123)
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

### Float Vector 4
*Stores an RGBA color or 4 float parameters*.

| Param ID | Name | Description |
| --- | --- | --- |
| 98 |  | Usually (0.5, 0, 0, 0), (1, 0, 0, 0), or (0, 0, 0, 0). All values are used, however. 0 = texture alpha. 1 = no texture alpha. |
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
| A5 | CustomVector13 | RGB diffuse color multiplier. May also affect alpha. Usually present and set to (1, 1, 1, 1). |
| A6 | CustomVector14 | RGBA? Component values are usually equal. Controls some sort of rim lighting effect. |
| A7 | CustomVector15 | RGBA? Always (0, 1000, 1, 0.2). |
| A8 | CustomVector16 | XYZW? Always (0.017, 35, 43.47, 0). |
| AA | CustomVector18 | Used for sprite sheet animations. X = horizontal sprite count. Y = vertical sprite count. Z = frames per sprite. W = ??? |
| AB | CustomVector19 | RGBA? Usually (1, 1, 1, 1). |
| 13B | CustomVector20 | RGBA? |
| 13C | CustomVector21 | RGBA? |
| 13D | CustomVector22 | RGBA? |
| 13E | CustomVector23 | RGB controls some sort of fog color. W is as high as 2400. |
| 13F | CustomVector24 | RGBA? |
| 142 | CustomVector27 | XYZW? X has large values. YZW are always 0. |
| 144 | CustomVector29 | XYZW? X can be negative and has angle values (30, -60, or 720). Y and Z are always 0. W is always 300 or 0. Usually (0, 0, 0, 0). |
| 145 | CustomVector30 | RGBA? Commonly (0.5, 4, 1, 1). G is the max component. R is the min component. Used for skin materials. |
| 146 | CustomVector31 | UV Sampler? |
| 147 | CustomVector32 | UV Sampler? |
| 148 | CustomVector33 | UV Sampler? Used for materials with 4 col maps (Param IDS: 66, 67, 68, 69). |
| 149 | CustomVector34 | UV Sampler? |
| 14A | CustomVector35 | RGBA? A is always 0 or 1. Used in conjunction with CB and CC. |
| 14C | CustomVector37 | XYZ values appear to be angle values. W is between 0 and 1 (usually 0.1). |
| 14D | CustomVector38 | Used for moving plants on Green Greens, Great Cave Offensive, etc. XYZ appear to be angle values and are 0, 45, 60, or 90. W = intensity? |
| 14E | CustomVector39 | RGBA? |
| 14F | CustomVector40 | XYZW? Always (1, 25, 1, 25). |
| 151 | CustomVector42 | RGBA? |
| 155 | CustomVector46 | RGBA? Only used for one of Bayonetta's body anim meshes. |
| 156 | CustomVector47 | RGBA? Used for some stages. |


### Textures
*Note that multiple params use col maps. This is mosty likely due to separate UV channels and texture blending.*

| Param ID | Name | Description |
| --- | --- | --- |
| 5C | Texture0 | Col Map. The main col texture. |
| 5D | Texture1 | Col Map. A secondary col texture. |
| 5E | Texture2 | Irradiance Cubemap |
| 5F | Texture3 | Gao Map. |
| 60 | Texture4 | Normal Map |
| 61 | Texture5 | Emission Map |
| 62 | Texture6 | PRM Map |
| 63 | Texture7 | Specular Cubemap |
| 64 | Texture8 | Diffuse Cubemap |
| 65 | Texture9 | Bake Lit Map |
| 66 | Texture1 | Col Map |
| 67 | Texture11 | Col Map |
| 68 | Texture12 | Col Map/Projection Light Map |
| 69 | Texture13 | Col/Projection Light Map |
| 6A | Texture14 | Emission Map |
| 6B | Texture15 | ??? |
| 133 | Texture16 | Ink Normal Map. Only used for stage ink meshes. Usually set to a white texture. |
| 134 | Texture17 | ??? |
| 135 | Texture18 | ??? |
| 136 | Texture19 | ??? |

### Default Textures
| Name | Description | Param IDs |
| --- | --- | --- |
| #replace_cubemap | Used as a cubemap | 5E, 63 |
| default_Normal | Used as a normal map | 60 |
| default_White | Used as a col map | 61, 133 |
| default_black | Used as a col map | 61 |

### Samplers
Probably some sort of texture information. Each texture parameter has a corresponding parameter of
type E specified in the same order. The first 6 values are often (0, 0, 0, 1, 1, 1) for 2D textures or
(1, 1, 1, 1, 1, 1) for cubemaps.

| Param ID | Name | Description |
| --- | --- | --- |
| 6C | Sampler0 | Col Map (5C) |
| 6D | Sampler1 | Col Map (5D) |
| 6E | Sampler2 | Irradiance Cubemap (5E) |
| 6F | Sampler3 | Gao Map (6F) |
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

### Float
| Param ID | Name | Description |
| --- | --- | --- |
| C0 | CustomFloat0 | Set to 0.997 for some models on pikmin_planet and fox_venom. |
| C1 | CustomFloat1 | Usually set to 0. Set to 0.3 for stc_bg_chikei_set for xeno_gaur. Non zero for kirby_fountain models. |
| C4 | CustomFloat4 | ??? |
| C6 | CustomFloat6 | Set to 1 for the pikmin_planet models that use C0. |
| C8 | CustomFloat8 | Present for most fighters. |
| CA | CustomFloat10 | Used for anisotropic hair materials. |
| CB | CustomFloat11 | Values range from 0 to 20000 and are lower than CC. Used in conjunction with CC and 14A. |
| D0 | CustomFloat16 | Set to 5.5 for the ore club eye material. |
| D1 | CustomFloat17 | Set to 0.5 for the paper mario ship and nintendogs curtain. |
| D2 | CustomFloat18 | Has very large values. Values can be as high as 3000. |
| D3 | CustomFloat19 | Used for water and glass materials. Values are 1 or higher. |

### Boolean Flags
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
