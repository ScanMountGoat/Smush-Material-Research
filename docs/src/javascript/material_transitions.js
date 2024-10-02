import { MaterialTransitionsDemo } from "./material_transitions_demo.js";
import * as DataBinding from "./databinding.js";

const imgCanvas = document.getElementById("imgCanvas");

const thresholdText = document.getElementById("thresholdText");
const thresholdRange = document.getElementById("thresholdRange");

const demo = new MaterialTransitionsDemo(window, imgCanvas,
    parseFloat(thresholdText.value),
    "ink_base.png", 
    "ink_effect.png", 
    "ink_norb.png");

const shaded = document.getElementById("shaded");
shaded.addEventListener("change", function () { demo.updateRenderMode(0); }, false);

const norB = document.getElementById("norB");
norB.addEventListener("change", function () { demo.updateRenderMode(1); }, false);

const transitionBlend = document.getElementById("transitionBlend");
transitionBlend.addEventListener("change", function () { demo.updateRenderMode(2); }, false);

DataBinding.oneWayBindFloat(thresholdText, thresholdRange, demo.updateThreshold.bind(demo));