# Material Parameters
| Data Type | Description | Param IDs |
| --- | --- | --- |
| 1 | ??? | C8 D3 C6 CA D0 C4 C1 D2 CB CC C0 D1 |
| 2 | Boolean | E9 F3 EB EC EA EE ED EF F1 F4 E8 F2 F0 |
| 5 | Vector 4 | 98 9B 146 A0 A5 A6 145 144 A3 142 9E 9F 156 151 155 13C 147 AA 149 AB 13B 14C 14D 14E 13E 13D 14A A2 9C A1 99 A7 A8 9A 9D 14F 13F 148 |
| 10 | ??? |  FC FE 100 102 103 101 FD 105 10A FF |
| 11 | Struct?| 118 |
| 12 | Struct? | 123 |
| B | Texture | 6A 60 61 62 63 5C 5D 5E 64 67 66 5F 65 133 68 69 |
| E | Struct? |  7A 70 71 72 73 6C 6E 6D 74 77 76 6F 75 137 78 79 |

# Material Param Descriptions
| Param ID | Description |
| --- | --- |
| 100 | ??? |
| 101 | ??? |
| 102 | ??? |
| 103 | ??? |
| 105 | ??? |
| 10A | ??? |

### Struct Type 11
| Param ID | Description |
| --- | --- |
| 118 | Struct? |

### Struct Type 12
| Param ID | Description |
| --- | --- |
| 123 | Struct? |

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
| A0 | RGBA? A is always 0 or 1. |
| A1 | RGBA? Always (0.2620026, -0.6427876, -0.7198463, 0). |
| A2 | RGBA? Always (1, 0.8, 0.6, 0). |
| A3 | RGBA? A is always 1. Very dark or matches skin tone for skin mats (including non human characters). |
| A5 | RGBA? A is always 0 or 1. Colors are similar to skin tones when used in skin mats (including non human characters). |
| A6 | RGBA? Component values are usually equal. Controls some sort of rim lighting effect. |
| A7 | RGBA? Always (0, 1000, 1, 0.2). |
| A8 | XYZW? Always (0.017, 35, 43.47, 0). |
| AA | XYZW? Values are very large and sometimes powers of 2. |
| AB | RGBA? Usually (1, 1, 1, 1). |
| 13B | RGBA? |
| 13C | RGBA? |
| 13D | RGBA? |
| 13E | XYZ are between 0 and 1. W is as high as 2400. |
| 13F | RGBA? |
| 142 | XYZW? X has large values. YZW are always 0. |
| 144 | XYZW? X can be negative and has angle values (30, -60, or 720). Y and Z are always 0. W is always 300 or 0. Usually (0, 0, 0, 0). |
| 145 | RGBA? Commonly (0.5, 4, 1, 1). G is the max component. R is the min component. |
| 146 | UV Sampler? |
| 147 | UV Sampler? |
| 148 | UV Sampler? Used for materials with 4 col maps (Param IDS: 66, 67, 68, 69). |
| 149 | UV Sampler? |
| 14A | RGBA? A is always 0 or 1. |
| 14C | XYZ values appear to be angle values. W is between 0 and 1 (usually 0.1). |
| 14D | XYZ appear to be angle values and are 0, 45, 60, or 90. W is between 0 and 1. |
| 14E | RGBA? |
| 14F | XYZW? Always (1, 25, 1, 25). |
| 151 | RGBA? |
| 155 | RGBA? Only used for a bayo skin mat. |
| 156 | RGBA? |


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
| 65 | Lit/Bake Lit Map |
| 66 | Col Map |
| 67 | Col Map |
| 68 | Col Map |
| 69 | Col/Projection Light Map |
| 6A | Eye Col |
| 133 | Col Map: usually `default_White`, `noise_for_warp` for ink mats |

### Default Textures
| Name | Description | Param IDs |
| --- | --- | --- |
| #replace_cubemap | Used as a cubemap | 5E, 63 |
| default_Normal | Used as a normal map | 60 |
| default_White | Used as a col map | 61, 133 |
| default_black | Used as a col map | 61 |

### Struct Type E
| Param ID | Description |
| --- | --- |
| 6C | Struct? |
| 6D | Struct? |
| 6E | Struct? |
| 6F | Struct? |
| 70 | Struct? |
| 71 | Struct? |
| 72 | Struct? |
| 73 | Struct? |
| 74 | Struct? |
| 75 | Struct? |
| 76 | Struct? |
| 77 | Struct? |
| 78 | Struct? |
| 79 | Struct? |
| 7A | Struct? |
| 137 | Struct? |


| Param ID | Description |
| --- | --- |
| C0 | ??? |
| C1 | ??? |
| C4 | ??? |
| C6 | ??? |
| C8 | ??? |
| CA | ??? |
| CB | ??? |
| CC | ??? |
| D0 | ??? |
| D1 | ??? |
| D2 | ??? |
| D3 | ??? |
| FC | ??? |
| FD | ??? |
| FE | ??? |
| FF | ??? |  

### Boolean Flags
| Param ID | Description |
| --- | --- |
| E8 | Boolean? |
| E9 | Boolean? |
| EA | Set to 0 for Olimar's helmet. Set to 1 for Incineroar emission mesh. Usually not present. |
| EB | Boolean? |
| EC | Boolean? |
| ED | Boolean? |
| EE | Boolean? |
| EF | Boolean? |
| F0 | Boolean? |
| F1 | Boolean? |
| F2 | Boolean? |
| F3 | Boolean? |
| F4 | Boolean? |
