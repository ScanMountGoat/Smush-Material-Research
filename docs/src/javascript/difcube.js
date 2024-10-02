import { DifCubeDemo } from "./difcube_demo.js";

const imgCanvas = document.getElementById("imgCanvas");
const demo = new DifCubeDemo(window, imgCanvas, "");

document.getElementById("sphere").addEventListener("click", function () { demo.setSphere(); }, false);
document.getElementById("cube").addEventListener("click", function () { demo.setCube(); }, false);