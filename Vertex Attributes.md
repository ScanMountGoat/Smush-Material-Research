# Position0
The position of the vertices.

# Normal0
The normal vector used for shading.  

# Tangent0
The tangent vector used for calculating normal maps and anisotropic specular. The bitangent vector needed for these calculations is generated.
The W component is used to flip the generated bitangent to account for normal maps with mirrored UVs. 

# Color Set Attributes
Not all color sets contain color data. 

The unsigned byte values are normalized to floats and then multiplied by some constant.  
This allows storing floating point values higher than 1.0 in a single byte at the cost of precision. 
The scale depends on the colorSet, and the same scale is applied to each of the RGBA channels.

| colorSet Value | Calculated Value | 
| --- | --- |
| 0 | 0.0 * scale |
| 128 | 0.5 * scale |
| 255 | 1.0 * scale | 

| Attribute | Description | Values | Default Value | Scale |
| --- | --- | --- | --- | --- |
| colorSet1 | Vertex color and alpha | RGB = vertex color, A = vertex alpha | (0.5, 0.5, 0.5, 0.5) | 2.0 | 
| colorSet2 | Baked diffuse lighting. The RGB values are squared before multiplying by the scale. | RGB = lighting color, A = *unused* | ??? | 7.0 
| colorSet2_1 | Similar to colorSet2 | RGB = lighting color, A = *unused* | ??? | 7.0 |
| colorSet2_2 | Similar to colorSet2 | RGB = lighting color, A = *unused* | ??? | 7.0 |
| colorSet2_3 | Similar to colorSet2 | RGB = lighting color, A = *unused* | ??? | 7.0 |
| colorSet3 | Vertex color and alpha. Similar to colorSet1. | RGB = vertex color, A = ??? | (0.5, 0.5, 0.5, 0.5) | 2.0 | 
| colorSet4 | Some sort of specular tint | RGB = color, A = blend factor | ??? | 2.0 |
| colorSet5 | Used for texture blending. Some shaders just use alpha to blends between the primary and secondary col maps.  | R = diffuse layer 1?, B = diffuse layer 2?, G = ??, A = blend factor | (0, 0, 0, 0) or (0, 0, 0, 1/3) |  3.0 | 
| colorSet6 | Controls the warping effect used for plants along with CustomVector37,38,39 | XYZ = ???, W = ??? | ??? | 1.0 |
| colorSet7 | Controls some sort of fog effect | XYZ = ???, W = intensity | ??? |  1.0 | 

# UV Attributes
The XY components of the vertex attribute are used as UV coordinates. The ZW components are unused. 
| Attribute | Description |
| --- | --- |
| map1 | UV coordinates for the first texture layer |
| bake1 | UV coordinates for Bake Lit and Gao Maps | 
| uvSet | UV coordinates for the second texture layer | 
| uvSet1 | ??? |
| uvSet2 | ??? |
