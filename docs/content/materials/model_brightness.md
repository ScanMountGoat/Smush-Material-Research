+++
title = "Model Brightness - WIP"
draft = true
+++
Artists used to making assets for games without PBR materials or modern games with more advanced PBR rendering engines 
often find models in Smash Ultimate to appear brighter than expected. Physically based rendering or "PBR" is not a precise 
technical term. Different games and applications often have very different implementations with varying degrees of realism, 
so assets from one game or application often don't appear properly in Smash Ultimate. This page explains the two main sources of overly bright models and potential fixes.

## Albedo Color
### Explanation
Smash Ultimate takes a slightly different approach than past entries in the series by mapping textures slightly darker than pure 
white (255 RGB or 1.0) to appear white in game. The game designers may have done this to match the fact that many surfaces 
in real life don't reflect the majority of incoming light and have a relatively low albedo. The [wikipedia albedo page](https://en.wikipedia.org/wiki/Albedo) has a diagram of common real world albedo values that also tend to work well in Smash Ultimate.

### Fixes
TODO: link to col map image

As a general rule, keep albedo below 75% brightness. This means the colors in col maps, diffuse maps, and emi maps should be darker than 
(180, 180, 180) in RGB or (0.75, 0.75, 0.75) in float. A real world example is white paper or some white paints that also have reflect approximately 70% of incoming light.

TODO: material params, darkening textures, 

## Specular Highlights
### Explanation
TODO: Samus reflections with varying roughness values
Specular reflections are another common source of overly bright models and are the hardest to control. 
Specular reflections in Smash Ultimate try to be energy conserving, meaning that objects never reflect more light than they receive. 
In practice, specular highlights appear darker as the roughness increases since the same amount of light is reflected from a larger area. 
Decreasing roughness focuses the specular highlight into an intensely bright region. The GGX function used for specular in Smash Ultimate is 
prone to having bright highlights, especially for metallic materials with low roughness values.

### Fixes
TODO: "balance" metalness with roughness, avoid specular values above 0.16, start with a default PRM map