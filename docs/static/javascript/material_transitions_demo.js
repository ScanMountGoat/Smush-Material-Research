import * as THREE from "./three.module.js";
import { TextureScene } from "./texturescene.js";

class MaterialTransitionsDemo {
    /**
     * Initializes the material transitions demo. 
     * A single frame is rendered once all textures are loaded.
     * @param {*} window the window used for drawing
     * @param {*} canvas the canvas used for drawing
     * @param {Number} threshold the initial material transition threshold
     * @param {String} baseImgPath the path of the untransitioned image
     * @param {String} effectImgPath the path of the fully transitioned image
     * @param {String} effectNorBluePath the path of the NOR map blue channel image
     */
    constructor(window, canvas, threshold, baseImgPath, effectImgPath, effectNorBluePath, maskImgPath) {
        const manager = new THREE.LoadingManager();

        const base = new THREE.TextureLoader(manager).load(baseImgPath);
        const effect = new THREE.TextureLoader(manager).load(effectImgPath);
        const norB = new THREE.TextureLoader(manager).load(effectNorBluePath);

        this.isLoaded = false;

        const that = this;
        // Use a loading manager to render the first frame once all textures are loaded.
        manager.onLoad = function () {
            that.material = new THREE.ShaderMaterial({
                vertexShader: `
                    varying vec2 vUv;
                    
                    void main() {
                        vUv = uv;
                        gl_Position = vec4( position, 1.0 );    
                    }`,
                fragmentShader: `
                    varying vec2 vUv;

                    uniform sampler2D base;
                    uniform sampler2D effect;
                    uniform sampler2D norB;

                    uniform int renderMode;
                    uniform float threshold;

                    void main() {
                        vec4 baseColor = texture(base, vUv);
                        vec4 effectColor = texture(effect, vUv);
                        vec4 norColor = texture(norB, vUv);

                        // Blend between the base and transitioned colors.
                        // This is simpler and more efficient for demos than how it works in game.
                        vec3 composite = baseColor.rgb;
                        float transitionBlend = 0.0;
                        if (norColor.b >= (1.0 - threshold)) {
                            composite = effectColor.rgb;
                            transitionBlend = 1.0;
                        }

                        if (renderMode == 0) {
                            gl_FragColor.rgb = composite;
                        } else if (renderMode == 1) {
                            gl_FragColor.rgb = vec3(norColor.b);
                        } else {
                            gl_FragColor.rgb = vec3(transitionBlend);
                        }

                        gl_FragColor.rgb = mix(baseColor.rgb, gl_FragColor.rgb, norColor.a);

                        gl_FragColor.a = 1.0;
                    }`,
                uniforms: {
                    renderMode: { value: 0 },
                    threshold: { value: threshold },
                    base: { value: base },
                    effect: { value: effect },
                    norB: { value: norB },
                }
            });

            that.textureScene = new TextureScene(window, canvas, that.material);

            that.textureScene.render();

            that.isLoaded = true;
        };
    }

    /**
     * 0 = Shaded, 1 = NOR.b, 2 = Transition Blend
     * Update the renderMode and rerender the scene.
     * @param {Number} value the render mode 
     */
    updateRenderMode(value) {
        if (this.isLoaded) {
            this.material.uniforms.renderMode.value = value;
            this.textureScene.render();
        }
    }

    /**
     * Update the transition threshold and rerender the scene.
     * @param {Number} value a floating point value
     */
    updateThreshold(value) {
        if (this.isLoaded) {
            this.material.uniforms.threshold.value = value;
            this.textureScene.render();
        }
    }
}

export { MaterialTransitionsDemo };
