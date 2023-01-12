# Material Parameter Descriptions
## Table of Contents
- [UV Transforms](#UV-Transforms)
- [Blend State](#Blend-State)
- [Rasterizer State](#Rasterizer-State)
- [Custom Vectors](#Custom-Vectors)
- [Textures](#Textures)
- [Samplers](#Samplers)
- [Custom Floats](#Custom-Floats)
- [Custom Booleans](#Custom-Booleans)

Material parameters marked as *Unused* aren't referenced in any of the shader programs, so there is no way to use them without modifying the shader binaries themselves.

The "Default Value" is a value that has no effect or the same effect as not having the parameter defined in the material. This is typically 0.0 for offset values (a + 0.0 = a), 1.0 for scale values (1.0 * a = a), and 1.0 for exponents (a^1.0 = a). The default value for intensities or blend factors for mixing between values may be 0.0 or 1.0 depending on the context. 

The following pesudocode shows how to correctly use a parameter for rendering. See the nufx and shader dumps for what parameters are required by a particular shader. This code is intended for "uber shaders" that expose all inputs regardless of whether an input is used or not. Another approach is to create individual shaders for each in game shader, so non required values are simply never set rather than using a default value.
```python
def get_parameter_value_for_rendering(shader, material, parameter):
    if shader.requires_parameter(parameter):
        if material.has_parameter(parameter):
            return material.get_value(parameter)
        else:
            # Missing required values default to (0.0, 0.0, 0.0, 0.0), false, etc.
            # Missing textures are handled differently.
            return parameter.zero
    else:
        # The parameter is unused, so use the default listed in the tables below.
        return parameter.default_value
```

## UV Transforms
Each texture has an associated UV transform parameter. The values are always (1, 1, 0, 0, 0).

| Param ID | Name | Description |
| --- | --- | --- |
| 252 (0xFC)  | UVTransform0 | Texture0 transform. |
| 253 (0xFD)  | UVTransform1 | Texture1 transform. |
| 254 (0xFE)  | UVTransform2 | Texture2 transform. |
| 255 (0xFF)  | UVTransform3 | Texture3 transform. |
| 256 (0x100) | UVTransform4 | Texture4 transform. |
| 257 (0x101) | UVTransform5 | Texture5 transform. |
| 258 (0x102) | UVTransform6 | Texture6 transform. |
| 259 (0x103) | UVTransform7 | Texture7 transform. |
| 260 (0x104) | UVTransform8 | Texture8 transform. |
| 261 (0x105) | UVTransform9 | Texture9 transform. |
| 262 (0x106) | UVTransform10 | *Unused* |
| 263 (0x107) | UVTransform11 | *Unused* |
| 264 (0x108) | UVTransform12 | *Unused* |
| 265 (0x109) | UVTransform13 | *Unused* |
| 266 (0x10A) | UVTransform14 | Texture14 transform. |
| 267 (0x10B) | UVTransform15 | *Unused* |
| 268 (0x10C) | DiffuseUVTransform1 | *Unused* |
| 269 (0x10D) | DiffuseUVTransform2 | *Unused* |
| 270 (0x10E) | SpecularUVTransform1 | *Unused* |
| 271 (0x10F) | SpecularUVTransform 2| *Unused* |
| 272 (0x110) | NormalUVTransform1 | *Unused* |
| 273 (0x111) | NormalUVTransform2 | *Unused* |
| 274 (0x112) | DiffuseUVTransform | *Unused* |
| 275 (0x113) | SpecularUVTransform | *Unused* |
| 276 (0x114) | NormalUVTransform | *Unused* |

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
| 280 (0x118) | BlendState0 | Used for all materials. |
| 281 (0x119) | BlendState1 | *Unused* |
| 282 (0x11A) | BlendState2 | *Unused* |
| 283 (0x11B) | BlendState3 | *Unused* |
| 284 (0x11C) | BlendState4 | *Unused* |
| 285 (0x11D) | BlendState5 | *Unused* |
| 286 (0x11E) | BlendState6 | *Unused* |
| 287 (0x11F) | BlendState7 | *Unused* |
| 288 (0x120) | BlendState8 | *Unused* |
| 289 (0x121) | BlendState9 | *Unused* |
| 290 (0x122) | BlendState10 | *Unused* |

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
| 291 (0x123) | RasterizerState0 | Used for all materials. |
| 292 (0x124) | RasterizerState1 | *Unused* |
| 293 (0x125) | RasterizerState2 | *Unused* |
| 294 (0x126) | RasterizerState3 | *Unused* |
| 295 (0x127) | RasterizerState4 | *Unused* |
| 296 (0x128) | RasterizerState5 | *Unused* |
| 297 (0x129) | RasterizerState6 | *Unused* |
| 298 (0x12A) | RasterizerState7 | *Unused* |
| 299 (0x12B) | RasterizerState8 | *Unused* |
| 300 (0x12C) | RasterizerState9 | *Unused* |
| 301 (0x12D) | RasterizerState10 | *Unused* |

| Field | Values|
| --- | --- |
| Polygon Fill | 0 = Line, 1 = Solid |
| Cull Mode | 0 = Back, 1 = Front, 2 = None |
| Depth Bias | Floating point depth offset. Values can be negative. |
| Unk4 | -1000, -100, 0, 100 |
| Unk5 | -100, 0 |
| Unk6 | 16777217 |

## Custom Vectors
Stores an RGBA color or 4 float parameters. If a custom vector required by the shader is missing, the value is set to black (0.0, 0.0, 0.0, 0.0).

| Param ID | Name | Default Value | Description |
| --- | --- | --- | --- |
| 152 (0x98) | CustomVector0 | (0,0,0,0) | X = min texture alpha for alpha testing, Y = Ryu/Ken focus intensity, Z = ???, W = *Unused* |
| 153 (0x99) | CustomVector1 | | RGBA? Used for s65_oniyon for pikmin_planet. Set to an orange color. |
| 154 (0x9A) | CustomVector2 | | RGBA? Used for some s65 models for pikmin_planet. Set to yellow. |
| 155 (0x9B) | CustomVector3 | (1,1,1,1) | Color multiplier for emission color. Used to brighten/darken emissive maps. Values are often higher than 1 to increase bloom. |
| 156 (0x9C) | CustomVector4 | | RGBA? Set to white for arwinbeam_set for fox_venom. |
| 157 (0x9D) | CustomVector5 | | RGBA? Set to white for s65_oniyon for pikmin_planet.  |
| 158 (0x9E) | CustomVector6 | (1,1,0,0) | UV Transform layer 1. |
| 159 (0x9F) | CustomVector7 | *Unused* | *Unused* |
| 160 (0xA0) | CustomVector8 | (1,1,1,1) | RGB color multiplier for the final color after diffuse, specular, etc. |
| 161 (0xA1) | CustomVector9 | | RGBA? Always (1, 1, 1, 1) or (0.2620026, -0.6427876, -0.7198463, 0). |
| 162 (0xA2) | CustomVector10 | *Unused* | *Unused* |
| 163 (0xA3) | CustomVector11 | (0,0,0,1) | Used with CustomVector30 to fake subsurface scattering. RGB = SSS color |
| 164 (0xA4) | CustomVector12 | *Unused* | *Unused* |
| 165 (0xA5) | CustomVector13 | (1,1,1,1) | RGB diffuse color multiplier. May also affect alpha. Usually present and set to (1, 1, 1, 1). |
| 166 (0xA6) | CustomVector14 | | Rim lighting color. RGB = color, A = blend factor. |
| 167 (0xA7) | CustomVector15 | *Unused* | *Unused* |
| 168 (0xA8) | CustomVector16 | *Unused* | *Unused* |
| 169 (0xA9) | CustomVector17 | *Unused* | *Unused* |
| 170 (0xAA) | CustomVector18 | (1,1,1,1) | Used for sprite sheet animations. X * Y = W if the sprite sheet is full. X = column count, Y = row count, Z = frames per sprite, W = sprite count |
| 171 (0xAB) | CustomVector19 | | RGBA? Usually (1, 1, 1, 1). Alpha is always 1. |
| 315 (0x13B) | CustomVector20 | | RGBA? RGB values are as high as 10. Alpha is 0 to 1. |
| 316 (0x13C) | CustomVector21 | | RGBA? Values are 0 to 1. |
| 317 (0x13D) | CustomVector22 | | RGBA? RGB values are 0 to 2. Alpha is 0 or 1. |
| 318 (0x13E) | CustomVector23 | | RGB controls some sort of fog color. W is as high as 2400. |
| 319 (0x13F) | CustomVector24 | | RGBA? RGB values are 0 to 1. Alpha is 0 or 1. |
| 320 (0x140) | CustomVector25 | | ??? |
| 321 (0x141) | CustomVector26 | *Unused* | *Unused* |
| 322 (0x142) | CustomVector27 | | Controls distance fog. X = Intensity, YZW = ??? |
| 323 (0x143) | CustomVector28 | *Unused* | *Unused* |
| 324 (0x144) | CustomVector29 | | Used for rotating models for pilot wings, sky world, etc. X can be negative and has angle values (30, -60, or 720). Y and Z are always 0. W is always 300 or 0. |
| 325 (0x145) | CustomVector30 | (0,0,0,1) | Used with CustomVector11 to fake subsurface scattering. X = blend factor, Y = diffuse shading smooth factor (higher is sharper), ZW = *Unused* |
| 326 (0x146) | CustomVector31 | (1,1,0,0) | UV Transform layer 2. |
| 327 (0x147) | CustomVector32 | (1,1,0,0) | UV Transform layer 3. |
| 328 (0x148) | CustomVector33 | | UV Transform? Used for materials with 4 base color maps (Param IDS: 66, 67, 68, 69). |
| 329 (0x149) | CustomVector34 | (1,1,0,0) | UV Transform dual normal. Used for water materials. |
| 330 (0x14A) | CustomVector35 | | RGBA? A is always 0 or 1. Used in conjunction with CB and CC. |
| 331 (0x14B) | CustomVector36 | *Unused* | *Unused* |
| 332 (0x14C) | CustomVector37 | | Some sort of rotation/warp effect used for vegetation. XYZ = ???, W = ??? |
| 333 (0x14D) | CustomVector38 | | Some sort of rotation/warp effect used for vegetation. XYZ = ???, W = ??? |
| 334 (0x14E) | CustomVector39 | | Some sort of rotation/warp effect used for vegetation. XYZ = ???, W = ??? |
| 335 (0x14F) | CustomVector40 | | XYZW? Always (1, 25, 1, 25). |
| 336 (0x150) | CustomVector41 | *Unused* | *Unused* |
| 337 (0x151) | CustomVector42 | | Controls some sort of rim lighting by smoothing between Z and W. XY = *Unused*, Z = smooth start, W = smooth end |
| 338 (0x152) | CustomVector43 | | Used to smoothly blend between CustomVector44 and CustomVector45 using vertex position. X = height threshold start, Y = height threshold end, ZW = *Unused* |
| 339 (0x153) | CustomVector44 | | Start color for blending with CustomVector43. RGB = color, A = *Unused* |
| 340 (0x154) | CustomVector45 | | End color for blending with CustomVector43. RGB = color, A = *Unused* |
| 341 (0x155) | CustomVector46 | | RGBA? Only used for one of Bayonetta's body anim meshes. |
| 342 (0x156) | CustomVector47 | | A parameter for PRM map values. R = metalness, G = roughness, B = ambient occlusion, A = alpha |


## Textures
Some texture names link to their corresponding page on the material research website.

The string value stored in the `.numatb` file is the corresponding texture's file name with no extension. This can be a file in the current directory like `asf_ashley_col` or a full path like `/common/shader/sfxPBS/default_Params`. Cube map textures can specify `#replace_cubemap` to use the current stage's reflection cube map.

Diffuse maps are additional textures with shader specific usages. Diffuse maps are used for color layers, alpha masks, and various other effects.  

| Param ID | Name | UV Attribute | Description |
| --- | --- | --- | --- |
| 92 (0x5C) | [Texture0](https://scanmountgoat.github.io/Smush-Material-Research/textures/col/) | map1 | Col Map (layer 1) |
| 93 (0x5D) | [Texture1](https://scanmountgoat.github.io/Smush-Material-Research/textures/col/) | uvSet | Col Map (layer 2) |
| 94 (0x5E) | Texture2 | *Cube Map* | Irradiance Cube Map |
| 95 (0x5F) | Texture3 | bake1 | Ambient Occlusion Map |
| 96 (0x60) | [Texture4](https://scanmountgoat.github.io/Smush-Material-Research/textures/nor/) | map1 | NOR Map |
| 97 (0x61) | [Texture5](https://scanmountgoat.github.io/Smush-Material-Research/textures/emi/) | map1 | Emissive Map (layer 1) |
| 98 (0x62) | [Texture6](https://scanmountgoat.github.io/Smush-Material-Research/textures/prm/) | map1 | PRM Map |
| 99 (0x63) | [Texture7](https://scanmountgoat.github.io/Smush-Material-Research/textures/specularcube/) | *Cube Map* | Specular Cube Map |
| 100 (0x64) | [Texture8](https://scanmountgoat.github.io/Smush-Material-Research/textures/difcube/) | *Cube Map* | Diffuse Cube Map |
| 101 (0x65) | Texture9 | bake1 | Baked Lighting Map |
| 102 (0x66) | Texture10 | map1 |  Diffuse Map (layer 1)  |
| 103 (0x67) | Texture11 | uvSet |  Diffuse Map (layer 2) |
| 104 (0x68) | Texture12 | ??? |  Diffuse Map (layer 3) |
| 105 (0x69) | Texture13 | *Projection Coords* |  Projection Light Map |
| 106 (0x6A) | [Texture14](https://scanmountgoat.github.io/Smush-Material-Research/textures/emi/) | uvSet |  Emissive Map (layer 2) |
| 107 (0x6B) | Texture15 | *Unused* |  *Unused* |
| 307 (0x133) | Texture16 | ??? | Ink Normal Map. Used for stage ink meshes. Often uses a default white texture. |
| 308 (0x134) | Texture17 | *Unused* | *Unused* |
| 309 (0x135) | Texture18 | *Unused* | *Unused* |
| 310 (0x136) | Texture19 | *Unused* | *Unused* |

### Default Textures
Any material can reference default textures stored in the `/common/shader` directories. Note that the red and yellow checkerboards that appear for invalid models are rendered in screen space using shaders and don't use any textures.  

| Name | RGBA |
| --- | --- |
| #replace_cubemap | *stage reflection_cubemap.nutexb* |
| /common/shader/sfxpbs/default_black | (0, 0, 0, 255) |
| /common/shader/sfxpbs/default_color | (255, 255, 255, 255) |
| /common/shader/sfxpbs/default_color2 | (255, 255, 255, 255) |
| /common/shader/sfxpbs/default_color3 | (255, 255, 255, 255) |
| /common/shader/sfxpbs/default_color4 | (255, 255, 255, 255) |
| /common/shader/sfxpbs/default_diffuse2 | (255, 255, 0, 25) and (0,0,0,0) |
| /common/shader/sfxpbs/default_gray | (123, 121, 123, 255) |
| /common/shader/sfxpbs/default_metallicbg | (0, 255, 255, 58) |
| /common/shader/sfxpbs/default_normal | (132, 120, 255, 255) |
| /common/shader/sfxpbs/default_params | (0, 255, 255, 58) |
| /common/shader/sfxpbs/default_params_r000_g025_b100 | (0, 65, 255, 255) |
| /common/shader/sfxpbs/default_params_r100_g025_b100 | (255, 65, 255, 255) |
| /common/shader/sfxpbs/default_params2 | (255, 255, 255, 255) |
| /common/shader/sfxpbs/default_params3 | (0, 117, 255, 58) |
| /common/shader/sfxpbs/default_params3 | (58, 61, 58, 255) |
| /common/shader/sfxpbs/default_white | (255, 255, 255, 255) |
| /common/shader/sfxpbs/fighter/default_normal | (128, 128, 255, 255) |
| /common/shader/sfxpbs/fighter/default_params | (0, 255, 255, 58) |

## Samplers
Each texture parameter has a corresponding sampler parameter. 
Border color determines sampled color for UVs outside the 0.0 to 1.0 range when the wrap mode is set to Clamp to Border.
Wrap S and Wrap T correspond to the U and V coordinates of the texture coordinates. Wrap R is used for cube maps.

| Param ID | Name | Description |
| --- | --- | --- |
| 108 (0x6C) | Sampler0 | Base Color Map (layer 1)|
| 109 (0x6D) | Sampler1 | Base Color Map (layer 2)|
| 110 (0x6E) | Sampler2 | Irradiance Cubemap |
| 111 (0x6F) | Sampler3 | Ambient Occlusion Map |
| 112 (0x70) | Sampler4 | Nor Map |
| 113 (0x71) | Sampler5 | Emi Map |
| 114 (0x72) | Sampler6 | PRM Map |
| 115 (0x73) | Sampler7 | Specular Cube Map |
| 116 (0x74) | Sampler8 | Diffuse Cubemap |
| 117 (0x75) | Sampler9 | Bake Lit Map |
| 118 (0x76) | Sampler10 | Diffuse Map (layer 1) |
| 119 (0x77) | Sampler11 | Diffuse Map (layer 2) |
| 120 (0x78) | Sampler12 | Diffuse Map (layer 3) |
| 121 (0x79) | Sampler13 | Projection Map |
| 122 (0x7A) | Sampler14 | Emissive Map (layer 2) |
| 123 (0x7B) | Sampler15 | *Unused* |
| 311 (0x137) | Sampler16 | Ink Normal Map |
| 312 (0x138) | Sampler17 | *Unused* |
| 313 (0x139) | Sampler18 | *Unused* |
| 314 (0x13A) | Sampler19 | *Unused* |

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
| Unk12 | Always 2139095022 (0x7F7FFFEE) (127, 127, 255, 238). This may or may not be used. |
| LOD Bias | Values are floating point and clamped between `-MAX_LOD` and `+MAX_LOD` where `MAX_LOD = Mipmap Count`  |
| Max Anisotropy | 1 = 1x, 2 = 2x, 4 = 4x, 8 = 8x, 16 = 16x |

## Custom Floats
If a custom float required by a shader is missing, the value is set to 0.0.

| Param ID | Name | Default Value | Description |
| --- | --- | --- | --- |
| 192 (0xC0) | CustomFloat0 | *Unused* | *Unused* |
| 193 (0xC1) | CustomFloat1 | 1.0 | Ambient occlusion map intensity ([Texture3](https://github.com/ScanMountGoat/Smush-Material-Research/blob/master/Textures.md#ambient-occlusion-maps-texture3)). A value of 0 has full effect. Values higher than 0 increase the intensity of the darkening effect. |
| 194 (0xC2) | CustomFloat2 | *Unused* | *Unused* |
| 195 (0xC3) | CustomFloat3 | *Unused* | *Unused* |
| 196 (0xC4) | CustomFloat4 | 0.0 | Controls the intensity of a UV distortion effect using the normal map. Used for water materials. |
| 197 (0xC5) | CustomFloat5 | *Unused* | *Unused* |
| 198 (0xC6) | CustomFloat6 | *Unused* | *Unused* |
| 199 (0xC7) | CustomFloat7 | *Unused* | *Unused* |
| 200 (0xC8) | CustomFloat8 | 0.0 | Controls albedo color tint intensity used for the rim lighting pass. |
| 201 (0xC9) | CustomFloat9 | | ??? |
| 202 (0xCA) | CustomFloat10 | 0.0 | Specular anisotropy. A value of 0 produces isotropic specular. |
| 203 (0xCB) | CustomFloat11 | | Values range from 0 to 20000 and are lower than CC. Used in conjunction with CC and 14A. |
| 204 (0xCC) | CustomFloat12 | | Values range from -100 to 100000. Used in conjunction with CB and 14A. |
| 205 (0xCD) | CustomFloat13 | *Unused* | *Unused* |
| 206 (0xCE) | CustomFloat14 | *Unused* | *Unused* |
| 207 (0xCF) | CustomFloat15 | *Unused* | *Unused* |
| 208 (0xD0) | CustomFloat16 | 0.0 | Vertex shader Z offset. Used to adjust depth sorting. |
| 209 (0xD1) | CustomFloat17 | | Set to 0.5 for the paper mario ship and nintendogs curtain. |
| 210 (0xD2) | CustomFloat18 | | Used with CustomVector37,38,39 for some sort of rotation/warp effect for vegetation. |
| 211 (0xD3) | CustomFloat19 | | Controls angle fade in addition to the specular IOR used for environment reflections. Higher values are less transparent and have more intense reflections. |

## Custom Booleans
Material flags are separated into individual boolean parameters rather than being combined into a single value. 

| Param ID | Name | Default Value | Description |
| --- | --- | --- | --- |
| 232 (0xE8) | CustomBoolean0 | *Unused* | *Unused* |
| 233 (0xE9) | CustomBoolean1 | True | PRM specular override. True = PRM alpha, False = 0.16 |
| 234 (0xEA) | CustomBoolean2 | False | Alpha Override. True = set alpha to 0.0, False = preserve alpha |
| 235 (0xEB) | CustomBoolean3 | True | Enables/disables the specular light contribution. |
| 236 (0xEC) | CustomBoolean4 | True | Enables/disables the specular cube map contribution. |
| 237 (0xED) | CustomBoolean5 | | Enables/disables the effect of CustomVector6 (UV Transform Layer 1). |
| 238 (0xEE) | CustomBoolean6 | | Enables/disables the effect of CustomVector31 (UV Transform Layer 2). |
| 239 (0xEF) | CustomBoolean7 | | Enables/disables the effect of CustomVector32 (UV Transform Layer 3). |
| 240 (0xF0) | CustomBoolean8 | | Set to true for bossstage_final3 and poke_kalos interval wall. |
| 241 (0xF1) | CustomBoolean9 | | Adjusts the effect of CustomVector18. Some shaders ignore CustomBoolean9. When enabled, CustomVector18.z selects the sprite in the texture to use. |
| 242 (0xF2) | CustomBoolean10 | | Set to false for spirits_floor_model\damage for each stage. |
| 243 (0xF3) | CustomBoolean11 | False | Texture blending mode. True = additive blending, False = alpha blending |
| 244 (0xF4) | CustomBoolean12 | | Set to true for fe_siege arena. False for ink meshes. |
| 245 (0xF5) | CustomBoolean13 | *Unused* | *Unused* |
| 246 (0xF6) | CustomBoolean14 | *Unused* | *Unused* |
| 247 (0xF7) | CustomBoolean15 | *Unused* | *Unused* |
| 248 (0xF8) | CustomBoolean16 | *Unused* | *Unused* |
| 249 (0xF9) | CustomBoolean17 | *Unused* | *Unused* |
| 250 (0xFA) | CustomBoolean18 | *Unused* | *Unused* |
| 251 (0xFB) | CustomBoolean19 | *Unused* | *Unused* |
