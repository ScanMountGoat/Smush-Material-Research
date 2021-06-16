---
---
# Material Transitions
Smash Ultimate contains a number of animated material transitions. The transitions happen quite quickly, so it's easy to miss the effect during normal gameplay. 
Examples of material transitions include inkling's ink or the metal box item. An animated threshold value combined with the model's NOR map blue channel 
enables models to smoothly transition between two completely different materials. The demo below demonstrates the effect of adjusting the threshold value.

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
                    <label for="threshold" class="col-md-4 col-form-label">Transition Threshold</label>
                    <input type="text" value="0.4" name="threshold" id="thresholdText" class="col-md-2">
                    <input type="range" value="0.4" min="0.0" max="1.0" step="0.001" name="threshold" id="thresholdRange" class="col">
                </div>
                <div class="form-group row justify-content-end">
                    <label for="threshold" class="col-md-4 col-form-label">Render Mode</label>
                    <div class="col">
                        <div class="form-check form-check-inline">
                            <input type="radio" name="renderMode" id="shaded" class="form-check-input" value="shaded" checked>
                            <label for="shaded" class="form-check-label">Shaded</label>
                        </div>
                        <div class="form-check form-check-inline">
                            <input type="radio" name="renderMode" id="norB" class="form-check-input" value="norB">
                            <label for="norB" class="form-check-label">NOR.b</label>
                        </div>
                        <div class="form-check form-check-inline">
                            <input type="radio" name="renderMode" id="transitionBlend" class="form-check-input" value="transitionBlend">
                            <label for="transitionBlend" class="form-check-label">Transition Blend (Calculated)</label>
                        </div>
                    </div>
                </div>
            </form>
        </div>
    </div>
</div>

## Material Transition Blending 
The pseudocode below describes how the blending factor is calculated using the model's NOR map. 
The smooth light to dark transitions in the NOR map blue channel are converted to values of either 0.0 or 1.0 using a threshold value.
In the demo above, note the difference in appearance between the NOR.b and calculated transition blend values.

As the threshold value approaches 1.0, the transition material fills in more of the model, starting 
with brighter regions of the NOR map blue channel and growing into darker regions. A NOR map blue channel value of 0.0 would transition instantly, a value of 0.5 would take later to transition, and a value of 1.0 takes the longest to transition to the transition material. As the threshold value approaches 0.0, the transition is reversed with the brightest regions of the NOR map blue channel being the last regions of the model to return to the normal material. 

Materials are blended together by blending their input parameters rather than blending the final colors for both materials. 
Blending the input parameters is generally more efficient because the model only has to be rendered once rather than rendering with each material separately and blending the results.

```glsl
// Calculate the blending factor between the original and transition material.
float transitionBlend = 0.0;
if (norColor.b >= (1.0 - threshold)) {
    transitionBlend = 1.0;
}
// Blend the relevant shading parameters. 
shadingParameter = mix(originalParameter, transitionMaterialParameter, transitionBlend);
```

## Transition Materials - WIP
### Inkling's Ink 
### Metal Box 
### Gold (Xerneas)
### Ditto

## Creating Transition Blend Maps - WIP


<script type="module">
    import { MaterialTransitionsDemo } from "./assets/javascript/material_transitions.js";
    import * as DataBinding from "./assets/javascript/databinding.js";

    const imgCanvas = document.getElementById("imgCanvas");

    const thresholdText = document.getElementById("thresholdText");
    const thresholdRange = document.getElementById("thresholdRange");

    const demo = new MaterialTransitionsDemo(window, imgCanvas,
        parseFloat(thresholdText.value),
        "{{ "/assets/images/material_transitions/ink_base.png" | relative_url }}", 
        "{{ "/assets/images/material_transitions/ink_effect.png" | relative_url }}", 
        "{{ "/assets/images/material_transitions/ink_norb.png" | relative_url }}");

    const shaded = document.getElementById("shaded");
    shaded.addEventListener("change", function () { demo.updateRenderMode(0); }, false);

    const norB = document.getElementById("norB");
    norB.addEventListener("change", function () { demo.updateRenderMode(1); }, false);

    const transitionBlend = document.getElementById("transitionBlend");
    transitionBlend.addEventListener("change", function () { demo.updateRenderMode(2); }, false);

    DataBinding.oneWayBindFloat(thresholdText, thresholdRange, demo.updateThreshold.bind(demo));
</script>