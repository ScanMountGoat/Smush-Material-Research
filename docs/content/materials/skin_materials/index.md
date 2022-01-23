+++
title = "Skin Materials"
aliases = ["skin_materials"]
+++
Many fighters have separate materials for approximating the subsurface scattering of skin.
The parameters can also be tweaked for more stylized diffuse shading, such as cel-shading or toon shading.

<style>
    .container {
        display: flex;
        flex-wrap: wrap;
    }
    
    .form-container {
        display: grid;
        grid-template-columns: 175px 50px 1fr;
        grid-auto-rows: 40px;
        row-gap: 10px;
        flex: 2;
    }

    canvas {
        flex: 1;
        width: 300px
    }
</style>

<div class="container">
    <canvas id="imgCanvas"></canvas>
    <form class="form-container">
            <label for="metalness">Metalness</label>
            <input type="text" value="1.0" name="metalness" id="metalnessText">
            <input type="range" value="1.0" min="0.0" max="1.0" step="0.001" name="metalness" id="metalness">
            <label for="albedo">Albedo</label>
            <input type="color" name="albedo" id="albedo" value="#E6DEC7">
            <div></div>
            <label for="customVector11">CustomVector11.rgb</label>
            <input type="color" name="customVector11" id="customVector11" value="#401200">
            <div></div>
            <label for="customVector30x">CustomVector30.x</label>
            <input type="text" value="0.5" name="customVector30x" id="customVector30xText">
            <input type="range" value="0.5" min="0.0" max="1.0" step="0.001" name="customVector30x" id="customVector30x">
            <label for="customVector30y">CustomVector30.y</label>
            <input type="text" value="1.5" name="customVector30y" id="customVector30yText">
            <input type="range" value="1.5" min="0.0" max="30.0" step="0.01" name="customVector30y" id="customVector30y">
    </form>
</div>

## Blending Intensity
<figure class="figure">
    <img src="snake_comparison.jpg" height="auto" width="auto">
    <figcaption class="figure-caption text-center">PRM.r set to 0.0 (left), PRM.r set to 1.0 (center left), Snake's PRM.r mask (center right), PRM.r set to the mask (right)</figcaption>
</figure>
The overall intensity of the effect is controlled by CustomVector30.x, which should have values between 0.0 and 1.0 to avoid artifacts. 
Skin materials always have a metalness of 0. The metalness map stored in the PRM red channel instead acts like a mask for the skin shading effect.
For full effect, both the metalness map and CustomVector30.x should be set to 1.0. 
If either CustomVector30.x or the metalness map are 0.0, the material will use the default diffuse shading.

```glsl
// CustomVector30.x is the overall intensity.
// Metalness acts like a mask.
float sssBlend = CustomVector30.x * metalness;
```

## Albedo Color
<figure class="figure">
    <img src="ivysaur_albedo.jpg" height="auto" width="auto">
    <figcaption class="figure-caption text-center">Ivysaur's col map (left), CustomVector11.rgb (center), and calculated albedo color (right)</figcaption>
</figure>
The RGB values for CustomVector11 control the subsurface color. This is typically a dark red color to approximate skin. Bright colors will likely cause unwanted bloom. 
The blending intensity is controlled by the PRM red channel and CustomVector30.x as described in the previous section.  

```glsl
// Blend the col map color with the subsurface color.
vec3 albedoFinal = mix(col.rgb, CustomVector11.rgb, sssBlend);
```
<br>
<figure class="figure">
    <img src="sss_albedo.png" height="auto" width="auto">
    <figcaption class="figure-caption text-center">The GLSL code converted to a node group in Blender's Shader Editor</figcaption>
</figure>

### Diffuse Shading
CustomVector30.y is multiplied by the diffuse shading to control the smoothness of the shading.
Using a very high value for the second value of CustomVector30 creates a cel-shaded look because diffuse shading is clamped to 1.0.
A very similar technique is used for Breath of the Wild's shaders, for example.

```glsl
float skinShading = nDotL * CustomVector30.y * 0.5 + 0.5;
skinShading = clamp(skinShading, 0.0, 1.0) ;
float finalDiffuseShading = mix(nDotL, skinShading, sssBlend);
```

The third and fourth parameters are unused, despite having values set for some models.

{{ demo(path="/javascript/skin_materials.js")}}