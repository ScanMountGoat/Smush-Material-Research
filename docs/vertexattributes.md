# Vertex Shading Attributes - WIP

## Normal0
The XYZ components of the Normal0 attribute contain the vertex normal. 

## Tangent0
Tangents and bitangents are used for normal map rendering and anisotropic specular. The tangent vector is the XYZ components of the Tangent0 attribute. The bitangent vector is generated from the tangent vector in the shaders, so there is no way to directly control the bitangent vector. 