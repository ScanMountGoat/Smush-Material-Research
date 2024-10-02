# Texture Coordinates
Texture coordinates or UV coordinates (often shortened to UVs) map the vertices to locations on the textures through a process called "UV Mapping". The UVs define a 2D position in texture space for each vertex in the mesh.

<figure class="figure">
    <img src="lucas_uvs.jpg">
    <figcaption class="figure-caption">UVs for the front (1), back (2), sleeves (3), bottom (4), and top (5). Seams are marked in red.</figcaption>
</figure>

UV coordinates project or "flatten" the 3D model onto 2D space. Real life anologies include sewing a shirt or stuffed animal, wrapping candy, or vinyl wrapping a car. In all these examples, a flat plane is wrapped onto a 3D shape after applying multiple cuts to create seams. UV maps for clothing meshes often match the patterns used to sew physical pieces of clothing. The image above shows the texture coordinates for Lucas's shirt mesh.

Most meshes have a map1 attribute for the main layer of textures. Additional texture coordinate attributes allow additional layers to have separate texture layouts and use the limited texture resolution more efficiently.

## map1
map1 is the main texture layer for col, PRM, NOR, and emissive maps. The map1 attribute is used at asset creation time along with the model's vertex normals to create the [Tangent0](../vertex-attributes) attribute.

## bake1
bake1 is designed for baked textures such as baked stage lighting or baked ambient occlusion. The bake1 uvs should have no overlapping or mirrored UVs. All UV coordinates should be in the range 0.0 to 1.0 and fit within the first quadrant in the UV editor. This ensures each vertex can have a unique color in the texture map and avoid issues when baking.

## uvSet
uvSet is used for the second texture layer for col, diffuse, and emissive maps. This includes the iris texture layer used for animated eye movement.

## uvSet1 
uvSet1 adds an additional texture layer and is mostly used for stage materials with many diffuse map layers.

## uvSet2
uvSet2 adds an additional texture layer and is mostly used for stage materials with many diffuse map layers.
