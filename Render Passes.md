# Render Passes
Rendering in Smash Ultimate is grouped into several different passes. This enables proper transparency rendering and post-processing effects such as bloom. 
Passes are described below in order from first to last. 

The different passes such as `_far` or `_sort` allow changing the mesh render order without moving the mesh's vertices or adjusting the order in the mesh list. The meshes in a render passes are grouped together, so all meshes with `_sort` render after all meshes with `_opaque`, for example.

## 1. Depth Pass
Renders models into a depth buffer to be used later for shadow mapping.

## 2. Opaque Pass
Renders models with a shader label ending in `_opaque`. This applies to most materials.

## 3. Alpha Blending Pass
Renders models with a shader label ending in `_far` and then models with a shader label ending in `_sort`.
These models typically use some form of alpha blending or alpha testing.

## 4. Bloom Pass
The scene is rendered into progressively smaller framebuffers to allow for bloom, which blurs bright regions of the scene. This causes bright objects rendered in previous passes to appear to glow.

## 5. Near Pass
Renders models with a shader label ending in `_near`. This occurs after bloom, so models in this pass will not contribute
to the bloom pass. This technique is used for many bright meshes that shouldn't appear to glow.

## 6. Color Grading
Performs final gamma correction and color correction. Color grading is done using a [3D RGB LUT texture](https://github.com/ScanMountGoat/Smush-Material-Research/blob/master/Textures.md#color-grading-lut)
 with the final scene color as input. 
