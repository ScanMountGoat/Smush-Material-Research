---
---
<style>
    .pixelated {
        -ms-interpolation-mode: nearest-neighbor;
        image-rendering: pixelated;
    }
</style>

# Post Processing Passes
<figure class="figure">
    <img src="{{ "/assets/images/post_processing/post_processing.jpg" | relative_url }}" height="auto" width="auto">
    <figcaption class="figure-caption text-center">From left to right: the base image, the image after adding bloom, the image after applying the color grading LUT (sepia), and the final post processing result.</figcaption>
</figure>
Smash Ultimate contains several post processing steps that grealy impact the final look of the image after all the models and effects are rendered. Post processing does not affect the color of UI elements such as stock icons and character portraits. Steps 2 to 4 are the post processing steps. 

1. Model and effect rendering
2. Add bloom 
3. Apply color grading LUT 
4. Post process to brighten the image 

## Bloom 
Bloom adds a glow around bright parts of the image. Any pixel in the frame that is brighter than a certain threshold contributes to bloom. The brighter the pixel, the more intense the bloom. The bloom threshold is calculated as follows. 
```glsl
float componentMax = max(max(inputColor.r, max(inputColor.g, inputColor.b)), 0.001);
float scale = max(0.925 * -0.5 + componentMax, 0.0);
vec3 bloomColor = inputColor.rgb / componentMax * scale * 6.0;
```

The graph below demonstrates the bloom intensity for different brightness values. The brightness threshold is roughly 75%, so any pixels with a brightness of 75% or higher will have some blooming. The graph only shows input values in the range 0.0 to 1.0, but its normal for models to have rendered brightness values much higher than 1.0 due to specular highlights, bright lighting, or certain material parameters.
<img src="{{ "/assets/images/post_processing/bloom_threshold.png" | relative_url }}" height="auto" width="auto">

## Color Grading LUTs
<div class="col-md-5">
    <img class="img-fluid float-left" src="{{ "/assets/images/post_processing/color_lut3d.png" | relative_url }}" height="auto" width="auto">
</div>
The normal and battlefield forms for each stage have a 3D LUT (lookup table) texture to add color grading to the final rendered image. The same technique is used for the [snapshot filters](snapshot). The color grading LUT stores a transformation from the unedited colors to their corresponding edited colors. The <a href="https://docs.unrealengine.com/en-US/RenderingAndGraphics/PostProcessEffects/UsingLUTs/index.html" target="_blank">Unreal Engine Docs</a> have a good description of how a 3D LUT can be used to perform color grading.

Each input RGB color is used as XYZ coordinates for the color grading LUT. For a 256x256x256 color grading LUT, determining the resulting pixel color is straightforward. Given an RGB color like (64,128,255), replace it with the color at position 64, 128, 255 in the LUT texture. A neutral LUT has a pixel color equal to the coordinates at every point in the LUT, so each input color maps to itself. The 3d plot shows the colors for the default color_grading_lut.nutexb. The corners of the cube correspond to the colors black, red, green, blue, magenta, cyan, and white. 

The in game LUTs, however, are only 16x16x16. Colors that don't have a corresponding point in the LUT get their color from a blend of nearby colors using linear filtering. The effect is similar to a gradient map or color ramp with 16 steps but in 3D. Color grading LUTs can't be used to perfectly recreate pixelated or cel shaded effects. Despite its small size, a 16x16x16 3D LUT can still accurately recreate a wide variety of image adjustments.

The LUT applies to all models and effects on screen but not UI. More extreme LUTs may make it difficult to differentiate between certain moves and fighters. An entirely black LUT, for example, would produce a black screen with only the fighter portraits and other UI elements visible.

<figure class="figure col">
    <img class="pixelated" src="{{ "/assets/images/post_processing/neutral_lut.png" | relative_url }}" height="auto" width="100%">
    <figcaption class="figure-caption text-center">The texture for the neutral color grading LUT above. Each of the 16 layers are separated into individual 16x16 slices for display.</figcaption>
</figure>

### Editing Color Grading LUTs
<figure class="figure">
<img src="{{ "/assets/images/post_processing/dreamland_lut.jpg" | relative_url }}" height="auto" width="auto">
    <figcaption class="figure-caption text-center">The result of editing the color_grading_lut.nutexb for Dreamland GB using a gradient map. The 16 slices of the LUT are displayed on the top of the image.</figcaption>
</figure>
A useful property of color grading LUTs is that any image editing operations that don't target individual pixels such as curves, levels, exposure, color balance, HSL, gradient maps, etc applied to the LUT will also apply to the final image. Simply apply the adjustments to a neutral color grading LUT and then save the result. A tool for creating color grading LUTs is available on the <a href="https://github.com/ScanMountGoat/Smush-LUT" target="_blank">Smush LUT Github repository</a>.

## Final Post Processing Pass 
Smash Ultimate applies an additional post processing pass after bloom and the color grading LUT has been applied. This step brightens the overall image significantly. The resulting image is roughly 1.4x brighter. The exact code is below. 

```glsl
vec3 GetPostProcessingResult(vec3 linear)
{
    // Convert to SRGB before applying the LUT.
    vec3 srgb = pow(fragColor0.rgb, vec3(0.454545));
    vec3 result = srgb * 0.9375 + 0.03125;

    // Apply the color grading LUT.
    result = texture(colorGradingLut, result.rgb).rgb;

    // Brighten the overall image. 
    result = (result - srgb) * 0.99961 + srgb;
    result *= 1.3703;

    // Convert back to linear.
    result = pow(result, vec3(2.2));
    return result;
}
```