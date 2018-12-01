# Col Maps
These are probably albedo textures due to the use of a metalness workflow.
RGBA = RGBA.

# PRM Maps
The main PBR textures. No idea what "prm" means.

*Note that these descriptions may change once more texture maps are analyzed.*  
R = metalness  
G = glossiness  
B = ambient occlusion  
A = specular (possibly reflectance at normal)  

# Emi Maps
TODO: Emission

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
TODO: Some form of baked stage lighting.

# Color Grading LUT
TODO: Some form of 2D lut for stage lighting. Most of the lut textures look identical.
