# Col Maps
RGB albedo textures. Unlike diffuse, albedo can color specular reflections for metallic materials. The alpha channel is used for transparency.

| Eye Texture Name | Usage |
| --- | --- |
| _b_col | Default Iris |
| _l_col | World of Light Enemy Iris (Red) |
| _g_col | Final Smash Iris (Yellow) |
| _w_col | Default Eye White |
| _d_col | Dark Iris (Purple) |
| _wd_col | Dark Eye White |

# PRM Maps
The main PBR textures. No idea what "prm" means.

*Note that these descriptions may change once more texture maps are analyzed.*  
R = metalness  
G = glossiness  
B = ambient occlusion  
A = specular (possibly reflectance at normal)  

# Emi Maps
Emission maps used for glowing effects such as Samus's lights. The majority of the texture will be black (no emission).

# Normal Maps
The RG channels are used for the normal map. The B channel is likely a separate texture. The A channel also appears to be a separate
texture.  

# Irradiance Cubemaps
TODO : PBR lighting

# Specular Cubemaps
TODO: PBR lighting

# Gao Bake Maps
Baked ambient occlusion used for stage models. The default_White texture is often used. Despite storing ambient occlusion data, the maps are RGB.

# Bake Lit Maps
Baked stage lighting. These maps are more colorful than Gao maps.

# Color Grading LUT
TODO: Some form of 2D lut for stage lighting. Most of the lut textures look identical.
