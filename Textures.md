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
The main PBR texture maps are packed into a single texture. Metalness is either 0 (non metal) or 1 (metal). Glosiness adds finer surface details. Ambient occlusion is a form of baked lighting. Specular controls the reflection intensity for non metals.

R = Metalness  
G = Glossiness   
B = Ambient Occlusion  
A = Specular

*For more technical details, see the following paper by Disney, whose principles have become an industry standard for PBR materials. The PRM maps work in a similar manner.*  
[Principled Shading Paper by Disney](https://disney-animation.s3.amazonaws.com/library/s2012_pbs_disney_brdf_notes_v2.pdf)

# Emi Maps
Emission maps used for glowing effects such as Samus's lights. The majority of the texture will be
black (no emission).

# Normal Maps
The RG channels are used for the XY directions of the normal map. The Z direction of the normal map is
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
