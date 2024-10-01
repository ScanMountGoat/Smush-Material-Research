/**
 * Calls updateCallBack with the new value for the "input" event of textInput and rangeInput.
 * Updates to textInput update rangeInput and vice versa.
 * @param {*} textInput the text input for the value
 * @param {*} rangeInput the range input for the value
 * @param {*} updateCallBack the callback function to call on updates with the new value 
 */
export function oneWayBindFloat(textInput, rangeInput, updateCallBack) {
    textInput.addEventListener("input", function () {
        rangeInput.value = textInput.value;
        updateCallBack(parseFloat(textInput.value));
    });
    rangeInput.addEventListener("input", function () {
        updateCallBack(parseFloat(rangeInput.value));
        textInput.value = rangeInput.value;
    });
};

/**
 * Calls updateCallBack with the new value for the "input" event of colorInput.
 * @param {*} colorInput the color input for the value
 * @param {*} updateCallBack the callback function to call on updates with the new value 
 */
export function oneWayBindColor(colorInput, updateCallBack) {
    colorInput.addEventListener("input", function () {
        updateCallBack(colorInput.value);
    });
};

export function twoWayBindInputsToColor(inputR, inputG, inputB, colorInput) {
    oneWayBindInputsToColor(inputR, inputG, inputB, colorInput);
    oneWayBindColorToInputs(colorInput, inputR, inputG, inputB);
}

/**
 * 
 * @param {*} inputR 
 * @param {*} inputG 
 * @param {*} inputB 
 * @param {*} colorInput 
 */
export function oneWayBindInputsToColor(inputR, inputG, inputB, colorInput) {
    inputR.addEventListener("input", function () {
        updateColorFromInputs(inputR, inputG, inputB, colorInput);
    });
    inputG.addEventListener("input", function () {
        updateColorFromInputs(inputR, inputG, inputB, colorInput);
    });
    inputB.addEventListener("input", function () {
        updateColorFromInputs(inputR, inputG, inputB, colorInput);
    });
}

/**
 * 
 * @param {*} colorInput 
 * @param {*} inputR 
 * @param {*} inputG 
 * @param {*} inputB 
 */
export function oneWayBindColorToInputs(colorInput, inputR, inputG, inputB) {
    colorInput.addEventListener("input", function () {
        const rgb = hexColorToRgb(colorInput.value);
        inputR.value = rgb[0].toString();
        inputG.value = rgb[1].toString();
        inputB.value = rgb[2].toString();
    });
}

function floatToHex(f) {
    // two digit hexidecimal
    const fCLamp = Math.max(Math.min(f, 1.0), 0.0);
    return ("0" + Math.floor(fCLamp * 255).toString(16)).slice(-2);
}

function hexToFloat(h) {
    return Math.max(Math.min(parseInt(h,16) / 255, 1.0), 0.0);
}

export function rgbToHexColor(r, g, b) {
    return "#" + floatToHex(r) + floatToHex(g) + floatToHex(b);
}

export function hexColorToRgb(color) {
    // Assume colors are of the form #rrggbb.
    const r = color.substring(1,3);
    const g = color.substring(3,5);
    const b = color.substring(5,7);

    return [hexToFloat(r), hexToFloat(g), hexToFloat(b)];
}

function updateColorFromInputs(inputR, inputG, inputB, colorInput) {
    colorInput.value = rgbToHexColor(parseFloat(inputR.value), parseFloat(inputG.value), parseFloat(inputB.value));
}
