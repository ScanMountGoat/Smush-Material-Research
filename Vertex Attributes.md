# Position0
The position of the vertices.

# Normal0
The normal vector used for shading.  

# Tangent0
The tangent vector used for calculating normal maps and anisotropic specular. The bitangent vector needed for these calculations is generated.
The W component is used to flip the generated bitangent to account for normal maps with mirrored UVs. 

# Vertex Color Attributes
Vertex color sets. RGBA values are typically 0 to 128, which are normalized to floats and then multiplied by 2.  
Ex: 0 = 0.0, 128 = 1.0, 255 = 2.0.  

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

# UV Attributes
The XY components of the vertex attribute are used as UV coordinates. The ZW components are unused. 
| Attribute | Description |
| --- | --- |
| map1 | UV coordinates for the first texture layer |
| bake1 | UV coordinates for Bake Lit and Gao Maps | 
| uvSet | UV coordinates for the second texture layer | 
| uvSet1 | ??? |
| uvSet2 | ??? |
