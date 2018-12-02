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
*Note: The _d and _wd textures are possibly unused.*

# PRM Maps
The main PBR textures. No idea what "prm" means.

*Note that these descriptions may change once more texture maps are analyzed.*  
R = Metalness
G = Glossiness  
B = Ambient Occlusion  
A = Specular (possibly reflectance at normal)  

# Emi Maps
Emission maps used for glowing effects such as Samus's lights. The majority of the texture will be
black (no emission).

# Normal Maps
The RG channels are used for the XY directions normal map. The Z direction of the normal map is
generated. The masking texture is used for blending between materials for ink, metal box, and other material transformations. Cavity maps are similar to ambient occlusion maps but contain finer details.

R = X+  
G = Y+  
B = Material Mask  
A = Cavity Map

# Irradiance Cubemaps
Cubemap for PBR diffuse stage lighting. Usually only 16x16. Often uses the default texture
#replace_cubemap.

# Specular Cubemaps
Cubemap for PBR specular stage lighting. Usually only 64x64. Often uses the default texture
#replace_cubemap.

# Gao Bake Maps
Baked ambient occlusion used for stage models. The default_White texture is often used. Despite
storing ambient occlusion data, the maps are RGB.

# Bake Lit Maps
Baked stage lighting. These maps are more colorful than Gao maps.

# Projection Light Maps
TODO: Used for some stages.

# Color Grading LUT
TODO: Some form of 2D lut for stage lighting. Most of the lut textures look identical.
