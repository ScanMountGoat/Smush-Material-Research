import { SssDemo } from "./skin_materials_demo.js";
import * as DataBinding from "./databinding.js";

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