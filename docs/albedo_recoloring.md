---
---
<style>
    #imgCanvas {
        width: 100%;
    }
</style>

# Albedo Recoloring
Renders can be recolored by extracting the lighting information and then applying a new albedo color.
If the same color is used for the previous and new albedo, the final result is unchanged.
The render will accurately match the selected new albedo color without the need for a levels/curves adjustment.
This also preserves the color variations in the original lighting unlike automatic recoloring or similar HSL techniques.

<canvas id="imgCanvas" class="col-lg-7"></canvas>
Select a new albedo color to see the armor on the render update in real time.
<form>
    <label for="albedo">Previous Albedo</label>
    <input type="color" id="albedo" name="albedo" value="#B0AFA9">
    <label for="newAlbedo">New Albedo</label>
    <input type="color" id="newAlbedo" name="newAlbedo" value="#B0AFA9">
</form>

## Details
This technique approximates well how fully metallic objects are rendered in game (PRM red channel is 1.0) because
metallic objects have no diffuse component. The results for non metallic materials will still match the desired overall luminance,
but there may be discolorations. Potential fixes are discussed in the image editing section.

### How to Determine Albedo Color
The albedo values can be copied from the Col maps for non skin materials. For skin materials, copy paste the values from
Cross Mod's albedo rendering mode by taking a screenshot or using a screen color picker. This takes into account the fake subsurface scattering effect applied in game.
See [Skin Materials](skin_materials) for details.

### Example Code
```c
// Metals
final = albedo x specular_light

// Non Metals
final = (albedo x diffuse_light) + (specular_light)

// Recoloring Metals
lighting = final / col_rgb
recolored = lighting * new_albedo

// Recoloring Non Metals
lighting = final / previous_albedo
recolored = lighting * new_albedo
```

## Albedo Recoloring in an Image Editor
<img src="{{ "/assets/images/albedo_recoloring/gimp_2_1.jpg" | relative_url }}" height="auto" width="auto">
The arrangement of the layers depends on the image editor being used. The above image is from Gimp 2.1.
If using layer groups, make sure the blend mode for the group is set to Pass through.

The order is important when working in 8 bits per channel images. Multiplying first mitigates potential clipping issues
from the divide layer.
If the effect still introduces noticeable banding artifacts, try switching to 16 bits per channel.

### Gimp, Photoshop, Krita
- Recolor Group + Mask (Pass through)
- Previous Albedo (Divide)
- New Albedo (Multiply)
- Base Render

If the final result is very discolored, double check the color used for the original albedo.
Another copy of the new albedo layer can be added to even out the color with the opacity adjusted as needed.
The color blend mode should be available in most image editors.

- Recolor Group + Mask (Pass through)
- New Albedo (Color)
- Previous Albedo (Divide)
- New Albedo (Multiply)
- Base Render

### Affinity Photo
If the image editor doesn't support the divide blending mode, invert the previous albedo color and set the layer blend
mode to color dodge. This performs the same operation as divide.

- Recolor Group (Pass through)
- Mask
- 1 - Previous Albedo (Color Dodge)
- New Albedo (Multiply)
- Base Render

## Examples
<div class="col-lg-7">
    <img class="img-fluid" src="{{ "/assets/images/albedo_recoloring/corrin_m_c04.jpg" | relative_url }}">
</div>

## Further Reading
For custom renders, there are more render passes available that can perfectly recreate the final render.
See Blender's <a href="https://docs.blender.org/manual/en/latest/render/layers/passes.html#combining"
    target="_blank">AOV Documentation</a>
for details. Remember to composite AOVs in 32 bit floating point with linear gamma (1.0) for proper blending and to
avoid clipping!

<script type="module">
    import { AlbedoRecoloringDemo } from "./assets/javascript/albedo_recoloring.js";
    import * as DataBinding from "./assets/javascript/databinding.js";

    const albedoColorInput = document.getElementById("albedo");
    const newAlbedoColorInput = document.getElementById("newAlbedo");
    const imgCanvas = document.getElementById("imgCanvas");

    // The texture paths are preprocessed by jekyll to contain the full path.
    const demo = new AlbedoRecoloringDemo(window, imgCanvas, 
        "{{ "/assets/images/albedo_recoloring/corrin.png" | relative_url }}", 
        "{{ "/assets/images/albedo_recoloring/mask.png" | relative_url }}", 
        albedoColorInput.value, 
        newAlbedoColorInput.value);

    DataBinding.oneWayBindColor(albedoColorInput, demo.updateAlbedo.bind(demo));
    DataBinding.oneWayBindColor(newAlbedoColorInput, demo.updateNewAlbedo.bind(demo));
</script>