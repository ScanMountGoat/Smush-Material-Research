# Col Maps
RGBA albedo textures. Unlike diffuse, albedo colors specular reflections for metallic materials.
The alpha channel is used for transparency.

### Naming Conventions
Textures in Smash Ultimate are referenced using strings in the materials. The table describes common
albedo texture naming conventions.

| Col Texture Name | Usage |
| --- | --- |
| _b | Default Iris |
| _l | World of Light Enemy Iris (Red) |
| _g | Final Smash Iris (Yellow) |
| _w | Default Eye White |
| _d | World of Light Enemy Dark Iris (Purple) |
| _wd | World of Light Enemy Dark Eye White |
| alp_ | Hair Color |
| def_ | Main Color |

# PRM Maps
The main PBR texture maps are packed into a single texture. This simplifies the number of textures
needed and can better take advantage of compression methods.

Metalness is either 0 (non metal) or 1
(metal). Values in between 0 and 1 are allowed but not physically correct.

Roughness adds finer surface details. Smooth materials such as metals will have low roughness values.

Ambient occlusion is a form of baked ambient
lighting.

Specular controls the reflectance at normal for non metals. The texture value is remapped to a range
of common values for non metals. Note that this is also how Blender's principled shader works.

| Channel | Usage |
| --- | --- |
| R | Metalness  |
| G | Roughness   |
| B | Ambient Occlusion |
| A | Specular Reflectivity |

*For more technical details, see the following paper by Disney, whose principles have become an industry standard for PBR materials. The PRM maps work in a similar manner.*  
[Principled Shading Paper by Disney](https://disney-animation.s3.amazonaws.com/library/s2012_pbs_disney_brdf_notes_v2.pdf)

# Emissive Maps
Emissive maps ared used for glowing effects such as Samus's lights. The majority of the texture will
be black (no emission). Some materials use emission in place of a col map for flat lighting. This is
common for retro stages and skyboxes.

# Normal Maps
The RG channels are used for the XY directions of the normal map. The Z direction of the normal map
is generated.

The blend map is used for blending between materials for ink, metal box, and gold forms
(Xerneas).

Cavity maps are similar to ambient occlusion maps but contain finer
details. The cavity map is used for specular occlusion, meaning that darker areas on the map will
have less intense reflections.

| Channel | Usage |
| --- | --- |
| R | Normal X+  |
| G | Normal Y+  |
| B | Blend Map  |
| A | Cavity Map|

# Irradiance Cubemaps
Cubemap for PBR diffuse stage lighting. Usually only 16x16. Often uses the default texture
#replace_cubemap.

# Specular Cubemaps
Cubemap for PBR specular stage lighting. Usually only 64x64. Often uses the default texture
#replace_cubemap.

# Gao Bake Maps
Baked ambient occlusion used for stage models. The default_White texture is often used, which has no
effect. These maps use the bake UV coordinates.

# Bake Lit Maps
Stores baked ambient lighting and shadows for stages. Like Gao maps, these maps use the bake UV
coordinates.

# Projection Light Maps
TODO: Used for some stages. The texture uses some form of projection instead of UVs.

# Color Grading LUT
A 2D lookup table for color grading. Each stage has a neutral texture. These textures are also used
for the various filters for screenshots (sepia, vivid, etc).
