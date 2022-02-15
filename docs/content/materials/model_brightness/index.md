+++
title = "Model Brightness - WIP"
draft = true
+++
Custom models in Smash Ultimate often appear brighter than expected. 
The game's implementation of more physically based lighting and materials can lead to unexpected results,
especially when porting assets from non PBR games. 
This page explains the two main sources of overly bright models and potential fixes.

## Albedo Color
### Explanation
<figure class="figure">
    <img src="/textures/col/mario_albedo.jpg" height="auto" width="auto">
    <figcaption class="figure-caption text-center">Note how the model appears brighter than the col map's color in the shaded squares.</figcaption>
</figure>

The model's [col maps](/textures/col/) determine its overall albedo or base color. An col map brightness of roughly 75% will appear white in game. Albedo colors brighter than 75% will start to clip and lead to excessive bloom.  The [wikipedia albedo page](https://en.wikipedia.org/wiki/Albedo) has a diagram with common real world albedo values that also tend to work well in Smash Ultimate. Real world example of surfaces that reflect approximately 75% of incoming light are paper and some white paints.

### Fixes
As a general rule, keep albedo below 75% brightness. This means the colors in col maps, diffuse maps, and emi maps should be darker than 
(180, 180, 180) in RGB or (0.75, 0.75, 0.75) in float. 

TODO: material params, darkening textures, 

## Specular Highlights
### Explanation
TODO: Fix the metalness values in text
<figure class="figure">
    <img src="samus.jpg" height="auto" width="auto">
    <figcaption class="figure-caption text-center">Different PRM (metalness, roughness, ambient occlusion, specular) values with ambient occlusion left unchanged.</figcaption>
</figure>
Specular reflections are another common source of overly bright models and are the hardest to control. 
Specular reflections in Smash Ultimate try to be energy conserving, meaning that objects never reflect more light than they receive. 
In practice, specular highlights appear darker as the roughness increases since the same amount of light is reflected from a larger area. 
Decreasing roughness focuses the specular highlight into an intensely bright region. 

The GGX function used for specular in Smash Ultimate is prone to having bright highlights, especially for metallic materials with low roughness values. This clipping in bright regions occurs even without bloom. This effect is easiest to see in the [PRM demo](/textures/prm/), which doesn't contain any post processing. 

### Fixing Metallic Materials
TODO: "balance" metalness with roughness, avoid specular values above 0.16, start with a default PRM map

### Fixing Non Metallic Materials
TODO: Link to the PRM specular section for non metals