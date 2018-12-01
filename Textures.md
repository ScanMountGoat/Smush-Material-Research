# Col Maps
These are probably albedo textures due to the use of a metalness workflow.
RGBA = RGBA.

| Eye Texture Name | Usage |
| --- | --- |
| _b_col | Default Iris |
| _l_col | ??? Iris (Red) |
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
RG are used for the normal map. The B channel is likely a separate texture. A is possibly another
texture.

# Irradiance Cubemaps
TODO : PBR lighting

# Specular Cubemaps
TODO: PBR lighting

# Gao Bake Maps
Baked ambient occlusion used for stage models. The default_White texture is often used instead.

# Bake Lit Maps
Baked stage lighting. Unlike Gao maps, these textures are often colored.

# Color Grading LUT
TODO: Some form of 2D lut for stage lighting. Most of the lut textures look identical.
