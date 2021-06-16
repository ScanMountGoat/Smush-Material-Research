---
---
# Skin Materials
Many fighters have separate materials for approximating the subsurface scattering of skin.
The parameters can also be tweaked for more stylized diffuse shading, such as cel-shading or toon shading.

<style>
    #imgCanvas {
        width: 100%;
    }
</style>

<div class="container">
    <div class="row">
        <div class="col-md-5 my-auto">
            <canvas id="imgCanvas"></canvas>
        </div>
        <div class="col my-auto">
            <form>
                <div class="form-group row justify-content-end">
                    <label for="metalness" class="col-md-5 col-lg-4 col-form-label">Metalness</label>
                    <input type="text" value="1.0" name="metalness" id="metalnessText" class="col-md-2">
                    <input type="range" value="1.0" min="0.0" max="1.0" step="0.001" name="metalness" id="metalness"
                        class="col">
                </div>
                <div class="form-group row justify-content-end">
                    <label for="albedo" class="col-md-5 col-lg-4 col-form-label">Albedo</label>
                    <input type="color" name="albedo" id="albedo" value="#E6DEC7" class="col-md-2">
                    <div class="col"></div>
                </div>
                <div class="form-group row justify-content-end">
                    <label for="customVector11" class="col-md-5 col-lg-4 col-form-label">CustomVector11.rgb</label>
                    <input type="color" name="customVector11" id="customVector11" value="#401200" class="col-md-2">
                    <div class="col"></div>
                </div>
                <div class="form-group row justify-content-end">
                    <label for="customVector30x" class="col-md-5 col-lg-4 col-form-label">CustomVector30.x</label>
                    <input type="text" value="0.5" name="customVector30x" id="customVector30xText"
                        class="col-md-2">
                    <input type="range" value="0.5" min="0.0" max="1.0" step="0.001" name="customVector30x"
                        id="customVector30x" class="col">
                </div>
                <div class="form-group row justify-content-end">
                    <label for="customVector30y" class="col-md-5 col-lg-4 col-form-label">CustomVector30.y</label>
                    <input type="text" value="1.5" name="customVector30y" id="customVector30yText"
                        class="col-md-2">
                    <input type="range" value="1.5" min="0.0" max="30.0" step="0.01" name="customVector30y"
                        id="customVector30y" class="col">
                </div>
            </form>
        </div>
    </div>
</div>

## Blending Intensity 
<figure class="figure">
    <img src="{{ "/assets/images/prm/snake_comparison.jpg" | relative_url }}" height="auto" width="auto">
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
    <img src="{{ "/assets/images/albedo/ivysaur_albedo.jpg" | relative_url }}" height="auto" width="auto">
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
    <img src="{{ "/assets/images/blender_nodes/sss_albedo.png" | relative_url }}" height="auto" width="auto">
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

<script type="module">
    import { SssDemo } from "./assets/javascript/skin_materials.js";
    import * as DataBinding from "./assets/javascript/databinding.js";

    const imgCanvas = document.getElementById("imgCanvas");

    const albedo = document.getElementById("albedo");

    const customVector11 = document.getElementById("customVector11");

    const metalness = document.getElementById("metalness");
    const metalnessText = document.getElementById("metalnessText");

    const customVector30x = document.getElementById("customVector30x");
    const customVector30xText = document.getElementById("customVector30xText");

    const customVector30y = document.getElementById("customVector30y");
    const customVector30yText = document.getElementById("customVector30yText");

    const getRangeValue = function (range) { return parseFloat(range.value); };

    const demo = new SssDemo(window, imgCanvas,
        albedo.value,
        customVector11.value,
        getRangeValue(customVector30x),
        getRangeValue(customVector30y),
        getRangeValue(metalness));

    DataBinding.oneWayBindColor(albedo, demo.updateAlbedo.bind(demo));
    DataBinding.oneWayBindColor(customVector11, demo.updateCustomVector11.bind(demo));

    DataBinding.oneWayBindFloat(metalness, metalnessText, demo.updateMetalness.bind(demo));
    DataBinding.oneWayBindFloat(customVector30x, customVector30xText, demo.updateCustomVector30x.bind(demo));
    DataBinding.oneWayBindFloat(customVector30y, customVector30yText, demo.updateCustomVector30y.bind(demo));
</script>