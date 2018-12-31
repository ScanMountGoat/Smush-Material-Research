# Position0
The position of the vertices.

# Tangent0
The tangent vector used for calculating normal maps and anisotropic specular. The bitangent vector needed for these calculations is generated.

# colorSet
Vertex color sets. RGBA values are typically 0 to 128, which are remapped to a standard 0.0 to 1.0 range. Ex: 0 = black. 128 = white.

| Attribute | Description |
| --- | --- |
| colorSet1 | The model color is multiplied by the vertex color. Commonly stores ambient occlusion or baked lighting data for fighters, stages, etc. |
| colorSet2 | Some sort of baked ambient lighting. |
| colorSet3 | ??? |
| colorSet2_1 | ??? |
| colorSet2_2 | ??? |
| colorSet2_3 | ??? |
| colorSet4 | ??? |
| colorSet5 | RGB = ??? Alpha is used for blending between the primary (5C) and secondary (5D) col maps.  |
| colorSet6 | ??? |
| colorSet7 | ??? |

# bake1
bake1 stores UV coordinates for Bake Lit and Gao maps.

X = bake map U  
Y = bake map V  

# map1
Stores the main UV coordinates for most textures.

# uvSet
Stores additional UV coordinates. These are used for stage materials with multiple col maps.

| Attribute | Description |
| --- | --- |
| uvSet | ??? |
| uvSet1 | ??? |
| uvSet2 | ??? |
