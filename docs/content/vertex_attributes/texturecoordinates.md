+++
title = "Texture Coordinates - WIP"
aliases = ["texturecoordinates"]
weight = 1
+++

Texture coordinates or UV coordinates (often shortened to UVs) map the vertices to locations on the textures. The UVs define a 2D position in texture space for each vertex. When rendering the 3D model, elements from the 2D texture are sampled using the UV coordinates through a process called "UV Mapping". UV coordinates project or "flatten" the 3D model onto a 2D image, usually involving multiple cuts or "seams" to make the mapping work properly. Real life anologies include cutting a flat cloth to make a shirt or wrapping oddly shaped presents from a flat piece of wrapping paper.

Most meshes have a map1 attribute for the main layer of textures. Additional texture coordinate attributes allow additional layers to have separate texture layouts and use the limited texture resolution more efficiently.

## map1
map1 is the main texture layer for col, PRM, NOR, and emissive maps. The map1 attribute is used at asset creation time along with the model's vertex normals to create the [Tangent0](../vertex_attributes) attribute.

## bake1
bake1 is designed for baked textures such as baked stage lighting or baked ambient occlusion. The bake1 uvs should have no overlapping or mirrored UVs. All UV coordinates should be in the range 0.0 to 1.0 and fit within the first quadrant in the UV editor. This ensures each vertex can have a unique color in the texture map and avoid issues when baking.

## uvSet

## uvSet1 

## uvSet2