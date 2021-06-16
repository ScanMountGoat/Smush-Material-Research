---
---
# Transparency - WIP
## Alpha Blending
Smash Ultimate uses alpha blending to simulate the appearance of transparent or translucent materials such as water, glass, smoke, etc. 
A background color called the "destination color" or "dst color" is blended with a foreground color called the "source color" or "src color". The effect is very similar to layer blend modes in Gimp or Photoshop. 

### Alpha Blending Blend Modes 
The alpha blending parameters are controlled by BlendState0 in the materials. The material's source factor 
and destination factor are controlled by the first and third value for BlendState0, respectively. 

See the <a href="https://github.com/ScanMountGoat/Smush-Material-Research/blob/master/Material%20Parameters.md#blending-state" target="_blank">BlendState0 documentation</a> for information on additional blending parameters a more complete list of possible blending factors.

### Alpha Blending Presets


## Alpha Testing 
Alpha testing skips rendering any pixels that have an alpha value less than a certain threshold. The texture's alpha channel acts as a mask to "cut out" transparent regions of the model. This avoids the sorting issues with alpha blending but can only be used for making regions of a model completely opaque or completely transparent. This works well for hair, leaves, or other materials that would 
have significant sorting issues with alpha blending and don't require being partially transparent. 

Alpha testing is hardcoded into certain shaders with a threshold of 0.5. Adjusting the first value of 
CustomVector0 to 1.0 effectively disables alpha testing because the texture alpha will be clamped to 1.0. 

```glsl
// Don't render any pixels with texture alpha less than 0.5.
// The texture alpha is usually from the col map.
float alpha = max(texture.a, CustomVector0.x);
if (alpha < 0.5)
    discard;
```

## Alpha Sample to Coverage




## Render Order
The order in which different meshes for a model are rendered depends on the shader label. The tag at the end of the shader label 
splits the meshes into different rendering passes. For example, all meshes for with "_far" will render before any meshes with "_sort" 
regardless of the parent model. In general, meshes with alpha blending should use _sort, and all other meshes should use _opaque.  

1. _opaque
2. _far
3. _sort
4. *bloom*
5. _near
6. *post processing*