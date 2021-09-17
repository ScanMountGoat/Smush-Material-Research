---
---
# PRM Maps (Texture6)
PRM maps control most of the important shading parameters for the more physically based materials introduced in Smash
Ultimate.
The different channels of the PRM maps correspond to four separate textures. 
The red channel is metalness, the green channel is roughness, the blue channel is ambient occlusion, and the alpha channel is specular.
PRM maps work similarly
to the inputs to Disney's principled shader, which is the basis for 
<a href="https://docs.blender.org/manual/en/latest/render/shader_nodes/shader/principled.html" target="_blank">Blender's Principled Shader</a>
and the shading in many modern games.

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
                    <label for="albedo" class="col-md-4 col-form-label">Albedo</label>
                    <input type="color" name="albedo" id="albedo" value="#990000" class="col-md-2">
                    <div class="col"></div>
                </div>
                <div class="form-group row justify-content-end">
                    <label for="prmColor" class="col-md-4 col-form-label">PRM RGB</label>
                    <input type="color" name="prmColor" id="prmColor" value="#007FFF" class="col-md-2">
                    <div class="col"></div>
                </div>
                <div class="form-group row justify-content-end">
                    <label for="metalness" class="col-md-4 col-form-label">Metalness</label>
                    <input type="text" value="0.0" name="metalness" id="metalnessText" class="col-md-2">
                    <input type="range" value="0.0" min="0.0" max="1.0" step="0.001" name="metalness" id="metalness"
                        class="col">
                </div>
                <div class="form-group row justify-content-end">
                    <label for="roughness" class="col-md-4 col-form-label">Roughness</label>
                    <input type="text" value="0.5" name="roughness" id="roughnessText" class="col-md-2">
                    <input type="range" value="0.5" min="0.0" max="1.0" step="0.001" name="roughness" id="roughness"
                        class="col">
                </div>
                <div class="form-group row justify-content-end">
                    <label for="ao" class="col-md-4 col-form-label">Ambient Occlusion</label>
                    <input type="text" value="1.0" name="ao" id="aoText" class="col-md-2">
                    <input type="range" value="1.0" min="0.0" max="1.0" step="0.001" name="ao" id="ao" class="col">
                </div>
                <div class="form-group row justify-content-end">
                    <label for="specular" class="col-md-4 col-form-label">Specular</label>
                    <input type="text" value="0.16" name="specular" id="specularText" class="col-md-2">
                    <input type="range" value="0.16" min="0.0" max="1.0" step="0.001" name="specular" id="specular"
                        class="col">
                </div>
            </form>
        </div>
    </div>
</div>

## PRM Color Channels
Although PRM maps are often very colorful (pink/cyan) when previewed in an image editor, *PRM maps do not contain color data*.
Avoid saving PRM maps as DDS or Nutexb with
srgb formats. Srgb formats have names that end in _SRGB. When rendering in programs such as Maya or Blender, set the PRM
maps to raw, non color data, or linear to ensure
the values aren't gamma corrected.

### Metalness (<span style="color:red">Red</span>)
<img src="{{ "/assets/images/prm/mario_metalness.jpg" | relative_url }}" height="auto" width="auto">
Metalness determines whether a surface is metallic or not and affects both specular and diffuse shading.
In general, materials should be either fully metallic (1.0) or non metallic (0.0).
Values in between 0.0 and 1.0 enable smoother blending between metallic and non metallic regions of a model.

Non metals have a white specular color and the diffuse component is colored by the albedo.
The specular intensity for non metals is controlled by the PRM's specular. Metals have no diffuse component
and specular intenisty is controlled entirely by albedo. In the demo above, note how the specular highlight becomes
closer in color to the albedo color as metalness increases.

Skin materials are a special case and instead use the metalness map as a mask for the fake subsurface scattering effect. 
See the [Skin Materials](skin_materials) page for details.

### Roughness (<span style="color:green">Green</span>)
<img src="{{ "/assets/images/prm/mario_roughness.jpg" | relative_url }}" height="auto" width="auto">
Roughness affects the size of the specular highlight. Rougher surfaces have larger specular highlights than glossy
surfaces.

The environment reflections will look blurrier for larger roughness values for both metals and non metals.
This is achieved using a cube map and mipmapping. Higher roughness values use smaller mipmaps,
resulting in more even specular lighting. In the demo above, set metalness to 1.0 and experiment with different
roughness values.

Physically based surfaces shouldn't reflect more light than they receive, so larger roughness values make the specular
highlight darker to preserve the overall amount of light being reflected. A rough, matte screen, for example, has reflections 
that are much darker than a glossy screen. Compare roughness values of 0.0, 0.25, 0.5, and 1.0 in the demo above to see 
the changes in both the size and brightness of the specular highlight.

### Ambient Occlusion (<span style="color:blue">Blue</span>)
<img src="{{ "/assets/images/prm/mario_ao.jpg" | relative_url }}" height="auto" width="auto">
Ambient occlusion maps are a baked approximation of shadows. The "ambient" refers to the soft lighting from the environment such as reflected light from the walls of a room or the sky. 
The "occlusion" refers to the darkening effect when light is "occluded" or blocked from reaching the surface. Cracks, crevices, and the corners of walls appear darker than the surrounding surface due to 
self shadowing that would be difficult to achieve with realtime shadows. Ambient occlusion maps improves the realism of the model by reducing unwanted ambient diffuse lighting and specular reflections
in certain areas like the backside of a cape or the roof of a character's mouth.

Ambient occlusion affects the intensity of specular and diffuse ambient lighting. In the demo above, note the
differences between setting ambient occlusion to 0.0 for metallic and non metallic materials. Setting ambient occlusion to white
(1.0) has no effect.

### Specular (Alpha)
<img src="{{ "/assets/images/prm/mario_specular.jpg" | relative_url }}" height="auto" width="auto">
Specular controls the base specular reflectivity of a surface. This effects the intensity of specular highlights and 
cube map reflections. The reflectivity always approaches 1.0 at glancing angles. Specular is scaled by 0.2, so a specular value of 0.5 
results in a specular reflectivity of 0.1. A reasonable starting value for specular is 0.16 (40 in RGB) to avoid overly bright highlights. 

<figure class="figure">
    <img src="{{ "/assets/images/prm/fresnel_lego.jpg" | relative_url }}" height="auto" width="auto">
    <figcaption class="figure-caption text-center">Note how the reflection in the glass is brightest when the camera is parallel with the glass.</figcaption>
</figure>
Surfaces in the real world exhibit something called the *fresnel effect* (pronounced "fre-nel") where the reflectivity 
of the surface depends on the angle between the orientation of the surface and the viewing direction. 
The effect is easiest to see on flat, glossy surfaces such as water or glass. In the below image of Mario, the edges 
of the model always have reflectance values close to 1.0 regardless of the base specular value from the PRM alpha. 
The effect is slightly less pronounced in game due to the intense rim lighting.   
  
<figure class="figure">
    <img src="{{ "/assets/images/prm/mario_f0.jpg" | relative_url }}" height="auto" width="auto">
    <figcaption class="figure-caption text-center">The specular reflectance for different PRM specular values. The buttons are not effected by specular since their metalness is 1.0.</figcaption>
</figure>
Metals work differently and use the albedo color from the col map as the specular reflectivity. This allows for specular to be 
tinted by the albedo color. Specular reflectivity is not scaled by 0.2 for metals, so metals can be significantly more reflective.
A metalness of 1.0 will ignore the specular map entirely and use the col map color to control specular intensity.

# PRM Compatibility Details
PBR textures are rendered slightly differently in different games and applications, so models will look
slightly different when PRM maps are used in Blender Cycles or Unreal Engine, for example. This section covers the
necessary technical details for accurately adapting PRM maps to work in other sources or adapting PBR textures from
other sources to work similarly in Smash Ultimate.

## Converting Metalness
PRM metalness maps are generally compatible between different games and applications. Some applications use separate
materials
for metals like gold or chrome and non metals. In that case, assume anything with a PRM metalness close to 1.0 is fully
metallic.

Metalness values between 0.0 and 1.0 in Smash Ultimate are often used to tint the specular color using the albedo color.
A similar effect can be achieved by using a non metallic material and tinting the specular color directly. This is
called "Specular Tint" in [Blender's Principled
Shader](https://docs.blender.org/manual/en/latest/render/shader_nodes/shader/principled.html).

Skin materials in Smash Ultimate are a special case in that they aren't actually metallic. Metalness should be set to
0.0 or use a
standard non metallic material and use the PRM metalness map to control subsurface scattering intensity. Similarly,
maps that mask subsurface scattering can be used as a metalness maps for skin materials in Smash Ultimate.

## Converting Roughness
Smash Ultimate squares its roughness values, which is common in other PBR renderers. Use this page as a reference to
match roughness values between Smash Ultimate
and other PBR textures. Roughness has values between 0.0 and 1.0, so squaring has the biggest impact on on values closer
to 0.0.
This has the effect of making smooth surfaces much smoother. Squaring roughness can be done in an image editor using a
gamma adjustment of 2.0.
Similarly, taking the square root of roughness is a gamma adjustment of 0.5.

Smash Ultimate clamps roughness values to 0.01 to avoid dividing by 0.0 in the shader. Roughness of 0.0 is a special
case that may be handled differently by different applications.
In general, try to use roughness values close to 0.01 instead of 0.0 for extremely smooth surfaces.

## Converting Ambient Occlusion
Ambient occlusion can be baked and/or painted by hand. Some games have very dark ambient occlusion maps that may not
work well with Smash Ultimate's lighting.
In that case, the original ambient occlusion map can be rebaked, adjusted to be lighter, or set to white (1.0) if the
added contrast and depth to shading isn't required.

Some physically based renderers such as Blender Cycles don't have shader inputs for ambient occlusion.
The shadowing effect of ambient occlusion is already simulated by the simulation of light rays in the scene.
If the PRM ambient occlusion map contains detail not present in the model's geometry (scales, fur, etc), the albedo
color can be multiplied by the
ambient occlusion to achieve a similar effect.

## Converting Specular
Specular defines the reflectance at normal. This may also be referred to as "f0" or "F0" in some applications.
The specular values are scaled by 0.2, so a specular of 1.0 in Smash Ultimate corresponds to a reflectance at normal of 0.2 and 0.0
still corresponds to 0.0. 

```
f0 = 0.2 * smashSpecular
```

### Convert Blender's Specular to Smash Ultimate
It's common for applications to define their own specular scale. Smash Ultimate has a specular range of 0% to 20%. Blender's Principled BSDF has a specular range of 0% to 8%, so the PRM specular should be converted as
```
smashSpecular = blenderSpecular * 0.4
``` 
Use a fill layer with color (0,0,40) in HSL or (102,102,102) in RGB set to the multiply blend mode. This is equivalent to dividing by 2.5. 

- Fill Layer (Multiply)
- Specular

### Convert Smash Ultimate's Specular to Blender
Converting Blender's specular to Smash Ultimate can be computed as
```
blenderSpecular = smashSpecular * 2.5
```
Image editors don't support multiplying by colors greater than 1.0 by default. Use a fill layer with color (0,0,40) in HSL or (102,102,102) in RGB set to the divide blend mode instead. This is equivalent to multiplying by 2.5. 

- Fill Layer (Divide)
- Specular

If the application doesn't support the divide blend mode, set the fill layer color to (0,0,60) in HSL or (153,153,153) in RGB. 

- Fill Layer (Color Dodge)
- Specular

### Convert IOR to Smash Ultimate Specular
Some applications may use IOR (Index of Reflection or Refraction) instead of specular.
IOR values can be converted using the following code or by entering IOR or PRM specular values below.
<div class="container">
    <form>
        <div class="row">
            <label for="ior" class="col">IOR</label>
            <div class="col-1"></div>
            <label for="spec" class="col">PRM Specular</label>
        </div>
        <div class="row">
            <input type="text" name="ior" id="ior" class="col" value="1.0">
            <div class="col-1 text-center">=</div>
            <input type="text" name="spec" id="specIor" class="col" value="0.0">
        </div>
    </form>
</div>
<br>
```
// IOR -> Smash PRM Specular
f0 = ((1 - ior) / (1 + ior)) ^ 2 
prmSpecular = f0 / 0.2

// Smash PRM Specular -> IOR
f0 = prmSpecular * 0.2
ior = -(f0 + 1 + 2*sqrt(f0)) / (f0 - 1)
```


<script type="module">
    import { PrmDemo } from "./assets/javascript/prm.js";
    import * as DataBinding from "./assets/javascript/databinding.js";

    const imgCanvas = document.getElementById("imgCanvas");

    const getRangeValue = function (range) { return parseFloat(range.value); };

    const albedo = document.getElementById("albedo");

    const metalnessText = document.getElementById("metalnessText");
    const metalnessRange = document.getElementById("metalness");

    const roughnessText = document.getElementById("roughnessText");
    const roughnessRange = document.getElementById("roughness");

    const aoText = document.getElementById("aoText");
    const aoRange = document.getElementById("ao");

    const specularText = document.getElementById("specularText");
    const specularRange = document.getElementById("specular");

    const demo = new PrmDemo(window, imgCanvas,
        albedo.value,
        parseFloat(metalnessText.value),
        parseFloat(roughnessText.value),
        parseFloat(aoText.value),
        parseFloat(specularText.value),
        "{{ "/assets/images/spec_cube/" | relative_url }}", 
        6,
        ".png");

    // Databind PRM Color.
    const prmColor = document.getElementById("prmColor");

    DataBinding.twoWayBindInputsToColor(metalnessText, roughnessText, aoText, prmColor);
    DataBinding.twoWayBindInputsToColor(metalnessRange, roughnessRange, aoRange, prmColor);

    prmColor.addEventListener("input", function () {
        // Update the rendering when the color changes.
        const rgb = DataBinding.hexColorToRgb(prmColor.value);
        demo.updateMetalness(rgb[0]);
        demo.updateRoughness(rgb[1]);
        demo.updateAmbientOcclusion(rgb[2]);
    }, false);

    DataBinding.oneWayBindFloat(metalnessText, metalnessRange, demo.updateMetalness.bind(demo));
    DataBinding.oneWayBindFloat(roughnessText, roughnessRange, demo.updateRoughness.bind(demo));
    DataBinding.oneWayBindFloat(aoText, aoRange, demo.updateAmbientOcclusion.bind(demo));
    DataBinding.oneWayBindFloat(specularText, specularRange, demo.updateSpecular.bind(demo));
    DataBinding.oneWayBindColor(albedo, demo.updateAlbedo.bind(demo));

    const iorText = document.getElementById("ior");
    const specIorText = document.getElementById("specIor");

    iorText.addEventListener("input", function () {
        const ior = parseFloat(iorText.value);
        console.log(ior);
        const newSpec = Math.pow((ior - 1.0) / (ior + 1.0), 2.0) / 0.2;
        console.log(newSpec);
        specIorText.value = newSpec;
    });
    //-[f0 + 1 + 2*sqrt(f0)] / (f0 - 1)
    specIor.addEventListener("input", function () {
        const spec = parseFloat(specIor.value);
        const f0 = spec * 0.2;
        const newIor = -(f0 + 1.0 + 2.0 * Math.sqrt(f0)) / (f0 - 1.0);
        ior.value = newIor;
    });
</script>