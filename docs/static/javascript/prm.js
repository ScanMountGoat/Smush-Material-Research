import { PrmDemo } from "./prm_demo.js";
import * as DataBinding from "./databinding.js";

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
    "./spec_cube/", 
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

// IOR <-> Specular text inputs.
const iorText = document.getElementById("iorText");
const specIorText = document.getElementById("specIor");

iorText.addEventListener("input", function () {
    const ior = parseFloat(iorText.value);
    const newSpec = Math.pow((ior - 1.0) / (ior + 1.0), 2.0) / 0.2;
    specIorText.value = newSpec;
});
specIorText.addEventListener("input", function () {
    const spec = parseFloat(specIorText.value);
    const f0 = spec * 0.2;
    const newIor = -(f0 + 1.0 + 2.0 * Math.sqrt(f0)) / (f0 - 1.0);
    iorText.value = newIor;
});

// Blender Specular <-> Smash Specular text inputs.
const smashSpecText = document.getElementById("smashSpec");
const blenderSpecText = document.getElementById("blenderSpec");

smashSpecText.addEventListener("input", function () {
    blenderSpecText.value = parseFloat(smashSpecText.value) * 2.5;
});
blenderSpecText.addEventListener("input", function () {
    smashSpecText.value = parseFloat(blenderSpecText.value) / 2.5;
});