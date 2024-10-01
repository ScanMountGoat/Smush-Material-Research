import { AlbedoRecoloringDemo } from "./albedo_recoloring_demo.js";
import * as DataBinding from "./databinding.js";

const albedoColorInput = document.getElementById("albedo");
const newAlbedoColorInput = document.getElementById("newAlbedo");
const imgCanvas = document.getElementById("imgCanvas");

// The texture paths are preprocessed by jekyll to contain the full path.
const demo = new AlbedoRecoloringDemo(window, imgCanvas, 
    "corrin.png", 
    "mask.png", 
    albedoColorInput.value, 
    newAlbedoColorInput.value);

DataBinding.oneWayBindColor(albedoColorInput, demo.updateAlbedo.bind(demo));
DataBinding.oneWayBindColor(newAlbedoColorInput, demo.updateNewAlbedo.bind(demo));