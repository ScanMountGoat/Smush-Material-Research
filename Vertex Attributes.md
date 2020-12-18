# Position0
The position of the vertices.

# Tangent0
The tangent vector used for calculating normal maps and anisotropic specular. The bitangent vector needed for these calculations is generated.

# colorSet
Vertex color sets. RGBA values are typically 0 to 128, which are remapped to a standard 0.0 to 1.0 range. Ex: 0 = black. 128 = white.

| Attribute | Description |
| --- | --- |
| colorSet1 | Vertex color and alpha. RGBA values are multipled by 2. RGB = vertex color, A = vertex alpha. |

| Attribute | Description |
| --- | --- |
| colorSet2 | Some sort of baked ambient lighting for stages. Similar in appearance to bake lit maps. |
| colorSet2_1 | Similar to colorSet2. These probably refer to additional lights. |
| colorSet2_2 | Similar to colorSet2. These probably refer to additional lights. |
| colorSet2_3 | Similar to colorSet2. These probably refer to additional lights. |

| Attribute | Description |
| --- | --- |
| colorSet3 | Similar to colorSet1. RGBA values are multipled by 2. RGB = vertex color, A = ???  |
| colorSet4 | Some sort of specular tint. RGBA values are multipled by 2. RGB = color, A = blend |
| colorSet5 | RGB = ??? Alpha is used for blending between the primary (5C) and secondary (5D) col maps.  |
| colorSet6 | Used for some vegetation meshes. Red is often white. Blue and green are gradients.  |
| colorSet7 | RGBA control separate intensity values for some sort of effect. May affect fog color. |

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
| uvSet | Used for the second layer for col maps. |
| uvSet1 | ??? |
| uvSet2 | ??? |
