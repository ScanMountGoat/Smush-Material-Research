+++
title = "Texture Coordinates - WIP"
aliases = ["texturecoordinates"]
+++

Texture coordinates or UV coordinates (often shortened to UVs) map the vertices to locations on the textures. Most meshes have a map1 attribute for the main layer of textures. Additional texture coordinate attributes allow additional layers to have separate texture layouts and use the limited texture resolution more efficiently. 

## map1
map1 is the main texture layer for col, PRM, NOR, and emissive maps. 

## bake1
bake1 is designed for baked textures such as baked stage lighting or baked ambient occlusion. The bake1 uvs are typically made of lots of tiny UV islands with no mirroring to prevent artifacts when baking.

## uvSet

## uvSet1 

## uvSet2