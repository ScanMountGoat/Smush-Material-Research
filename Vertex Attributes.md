# Position0
The position of the vertices.

# Normal0
The normal vector used for shading.  

# Tangent0
The tangent vector used for calculating normal maps and anisotropic specular. The bitangent vector needed for these calculations is generated.
The W component is used to flip the generated bitangent to account for normal maps with mirrored UVs. 

# Color Set Attributes
The unsigned byte values are normalized to floats and then multiplied by some constant.  
This allows storing floating point values higher than 1.0 in a single byte at the cost of precision. Not all color sets contain color data. The scale depends on the colorSet.

| colorSet Value | Calculated Value | 
| --- | --- |
| 0 | 0.0 * scale |
| 128 | 0.5 * scale |
| 255 | 1.0 * scale | 

| Attribute | Description | Values | Scale | 
| --- | --- | --- | --- |
| colorSet1 | Vertex color and alpha | RGB = vertex color, A = vertex alpha | 2.0 |
| colorSet2 | Some sort of baked ambient lighting for stages. Similar in appearance to bake lit maps. | RGB = ???, A = ??? | ??? |
| colorSet2_1 | Similar to colorSet2 | RGB = ???, A = ??? | ??? |
| colorSet2_2 | Similar to colorSet2 | RGB = ???, A = ??? | ??? |
| colorSet2_3 | Similar to colorSet2 | RGB = ???, A = ??? | ??? |
| colorSet3 | Vertex color and alpha. Similar to colorSet1. | RGB = vertex color, A = ??? | 2.0 |
| colorSet4 | Some sort of specular tint | RGB = color, A = blend factor | 2.0 |
| colorSet5 | Blends between the primary and secondary col maps | RGB = *unused*, A = alpha | 3.0 |
| colorSet6 | Controls the warping effect used for plants along with CustomVector37,38,39 | XYZ = ???, W = ???  | 1.0 |
| colorSet7 | Controls some sort of fog effect | XYZ = ???, W = intensity | 1.0 |

# UV Attributes
The XY components of the vertex attribute are used as UV coordinates. The ZW components are unused. 
| Attribute | Description |
| --- | --- |
| map1 | UV coordinates for the first texture layer |
| bake1 | UV coordinates for Bake Lit and Gao Maps | 
| uvSet | UV coordinates for the second texture layer | 
| uvSet1 | ??? |
| uvSet2 | ??? |
