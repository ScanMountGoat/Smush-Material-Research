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

| Param ID | Description |
| --- | --- |
| 100 | ??? |
| 101 | ??? |
| 102 | ??? |
| 103 | ??? |
| 105 | ??? |
| 10A | ??? |
| FC | ??? |
| FD | ??? |
| FE | ??? |
| FF | ??? |  

### Struct Type 11 (Param ID 118)
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

### Struct Type 12 (Param ID 123)
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

| Param ID | Description |
| --- | --- |
| 98 | Usually (0.5, 0, 0, 0), (1, 0, 0, 0), or (0, 0, 0, 0). All values are used, however. 0 = texture alpha. 1 = no texture alpha. |
| 99 | RGBA? |
| 9A | RGBA? |
| 9B | RGBA? |
| 9C | XYZW? Always (0, 1000, 1, 0.2). |
| 9D | RGBA? Always (1, 1, 1, 1). |
| 9E | UV Sampler? |
| 9F | RGBA? Always (1, 1, 1, 0). |
| A0 | RGBA? A is always 0 or 1. Used for Mario Galaxy. |
| A1 | RGBA? Always (0.2620026, -0.6427876, -0.7198463, 0). |
| A2 | RGBA? Always (1, 0.8, 0.6, 0). |
| A3 | RGBA? A is always 1. Very dark or matches skin tone for skin mats (including non human characters). |
| A5 | RGB diffuse color multiplier. May also affect alpha. Usually present and set to (1, 1, 1, 1). |
| A6 | RGBA? Component values are usually equal. Controls some sort of rim lighting effect. |
| A7 | RGBA? Always (0, 1000, 1, 0.2). |
| A8 | XYZW? Always (0.017, 35, 43.47, 0). |
| AA | Used for sprite sheet animations. X = horizontal sprite count. Y = vertical sprite count. Z = frames per sprite. W = ??? |
| AB | RGBA? Usually (1, 1, 1, 1). |
| 13B | RGBA? |
| 13C | RGBA? |
| 13D | RGBA? |
| 13E | RGB controls some sort of fog color. W is as high as 2400. |
| 13F | RGBA? |
| 142 | XYZW? X has large values. YZW are always 0. |
| 144 | XYZW? X can be negative and has angle values (30, -60, or 720). Y and Z are always 0. W is always 300 or 0. Usually (0, 0, 0, 0). |
| 145 | RGBA? Commonly (0.5, 4, 1, 1). G is the max component. R is the min component. Used for skin materials. |
| 146 | UV Sampler? |
| 147 | UV Sampler? |
| 148 | UV Sampler? Used for materials with 4 col maps (Param IDS: 66, 67, 68, 69). |
| 149 | UV Sampler? |
| 14A | RGBA? A is always 0 or 1. |
| 14C | XYZ values appear to be angle values. W is between 0 and 1 (usually 0.1). |
| 14D | Used for moving plants on Green Greens, Great Cave Offensive, etc. XYZ appear to be angle values and are 0, 45, 60, or 90. W = intensity? |
| 14E | RGBA? |
| 14F | XYZW? Always (1, 25, 1, 25). |
| 151 | RGBA? |
| 155 | RGBA? Only used for one of Bayonetta's body anim meshes. |
| 156 | RGBA? Used for some stages. |


### Textures
*Note that multiple params use col maps. This is mosty likely due to separate UV channels and texture blending.*

| Param ID | Description |
| --- | --- |
| 5C | Col Map. The main col texture. Also used for eye white textures. |
| 5D | Col Map. A secondary col texture. Used for iris textures. |
| 5E | Irradiance Cubemap |
| 5F | Gao Map. Commonly uses default_White. |
| 60 | Normal Map |
| 61 | Emission Map/Eye Col Map |
| 62 | PRM Map |
| 63 | Specular Cubemap |
| 64 | Col Map |
| 65 | Bake Lit Map |
| 66 | Col Map |
| 67 | Col Map |
| 68 | Col Map/Projection Light Map |
| 69 | Col/Projection Light Map |
| 6A | Emission Map |
| 133 | Ink Normal Map. Only used for stage ink meshes. Usually set to a white texture. |

### Default Textures
| Name | Description | Param IDs |
| --- | --- | --- |
| #replace_cubemap | Used as a cubemap | 5E, 63 |
| default_Normal | Used as a normal map | 60 |
| default_White | Used as a col map | 61, 133 |
| default_black | Used as a col map | 61 |

### Struct Type E
Probably some sort of texture information. Each texture parameter has a corresponding parameter of
type E specified in the same order. The first 6 values are often (0, 0, 0, 1, 1, 1) or
(1, 1, 1, 1, 1, 1).

| Param ID | Description |
| --- | --- |
| 6C | Col Map (5C) |
| 6D | Col Map (5D) |
| 6E | ??? |
| 6F | Gao Map (6F) |
| 70 | Nor Map (60) |
| 71 | Emi Map (61) |
| 72 | PRM Map (62) |
| 73 | Specular Cube Map (63) |
| 74 | ??? |
| 75 | Bake Lit Map (65) |
| 76 | ??? |
| 77 | ??? |
| 78 | ??? |
| 79 | Projection Map (69) |
| 7A | ??? |
| 137 | Ink Normal Map (133) |

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
| Param ID | Description |
| --- | --- |
| C0 | Set to 0.997 for some models on pikmin_planet and fox_venom. |
| C1 | Usually set to 0. Set to 0.3 for stc_bg_chikei_set for xeno_gaur. Non zero for kirby_fountain models. |
| C4 | ??? |
| C6 | Set to 1 for the pikmin_planet models that use C0. |
| C8 | Present for most fighters. |
| CA | Used for anisotropic hair materials. |
| CB | ??? |
| CC | ??? |
| D0 | ??? |
| D1 | ??? |
| D2 | ??? |
| D3 | ??? |

### Boolean Flags
| Param ID | Description |
| --- | --- |
| E8 | Boolean? |
| E9 | Set to 0 for fountain of dreams water. Seems to enables/disable specular occlusion for fighters. |
| EA | Set to 0 for Olimar's helmet. Set to 1 for Incineroar emission mesh. Usually not present. |
| EB | Boolean? |
| EC | Boolean? |
| ED | Boolean? |
| EE | Set to 1 for animal crossing island water. |
| EF | Boolean? |
| F0 | Boolean? |
| F1 | Some sort of sprite sheet scale toggle. Used in conjunction with param AA. |
| F2 | Boolean? |
| F3 | Boolean? |
| F4 | Boolean? |
