+++
title = "Color Sets - WIP"
aliases = ["colorsets"]
+++

The attribute names with "colorSet" store vertex color data. Color set attribute data is stored as RGBA colors with unsigned integer values from 0 to 255. The integer values are then converted to floating point values in the range 0.0 to *scale* where *scale* depends on the type of color set. The [Vertex Attribute](https://github.com/ScanMountGoat/Smush-Material-Research/blob/master/Vertex%20Attributes.md#color-set-attributes) page on the main repository contains a table of color set attributes and their scale values.

Whether or not the mesh's color set attribute data is used depends on the shader. For example, fighter meshes commonly contain data for a colorSet1 attribute but use a shader that never references colorSet1. The latest version of [Cross Mod](https://github.com/Ploaj/SSBHLib/releases) enables or disables color set rendering based on the shader listed in the material. 

## colorSet1

## colorSet2

## colorSet3

## colorSet4

## colorSet5
### (RGB)

### Texture Blend (Alpha)
<figure class="figure">
    <img src="colorset5.jpg" height="auto" width="auto">
    <figcaption class="figure-caption text-center">colorSet5 alpha values of 0 or 0.0 (left), 85 or 0.3333 (center), and 255 or 1.0 (right). The RGB values are unused.</figcaption>
</figure>
The alpha value of the colorSet5 attribute is used to blend between the first and second layer of textures. A value of 0.0 uses only the first layer. A value of 0.3333 (1.0 / 3.0) will only render the second layer since values are scaled by 3.0. Values greater than 0.3333 will continue to increase the blend intensity and generally produce undesirable artifacts.

<figure class="figure">
    <img src="colorset5_blend.jpg" height="auto" width="auto">
    <figcaption class="figure-caption text-center">The result of blending the rock (layer 1) and grass (layer 2) textures using colorSet5.</figcaption>
</figure>
colorSet5 enables blending between two layers of tiled textures. The rock and grass textures in this example are only 256x256, so it's not possible to specify an appropriate mask for the grassy regions using the texture alpha. colorSet5 has alpha values of roughly 85 or 0.3333 for the areas that should use the grass layer. The vertex color values are interpolated between adjacent vertices, which softens the transitions between textures. Duplicating faces along the desired boundares will prevent the values blending with neighboring faces and create a sharp seam.

## colorSet6

## colorSet7