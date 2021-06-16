import * as THREE from "./three.module.js";
import { TextureScene } from "./texturescene.js";

class AlbedoRecoloringDemo {
    /**
     * Initializes the albedo coloring demo. 
     * A single frame is rendered once all textures are loaded.
     * @param {*} window the window used for drawing
     * @param {*} canvas the canvas used for drawing
     * @param {*} renderImgPath the path of the render to recolor
     * @param {*} maskImgPath the path of the mask for the region to recolor
     * @param {DOMString} albedoColor the initial hex color for albedo
     * @param {DOMString} newAlbedoColor the initial hex color for new albedo
     */
    constructor(window, canvas, renderImgPath, maskImgPath, albedoColor, newAlbedoColor) {
        const manager = new THREE.LoadingManager();

        const texture = new THREE.TextureLoader(manager).load(renderImgPath);
        const mask = new THREE.TextureLoader(manager).load(maskImgPath);

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

                    uniform sampler2D image;
                    uniform sampler2D mask;

                    uniform vec3 albedo;
                    uniform vec3 newAlbedo;

                    void main() {
                        vec4 renderColor = texture(image, vUv);
                        vec4 maskColor = texture(mask, vUv);

                        // Clamp albedo to prevent potential divide by 0.
                        vec3 lighting = renderColor.rgb / max(albedo, 0.001);
                        vec3 recolored = lighting * newAlbedo;
                        vec3 composite = mix(renderColor.rgb, recolored, maskColor.r);

                        // Premultiplied alpha.
                        gl_FragColor.rgb = composite * renderColor.a;
                        gl_FragColor.a = renderColor.a;
                    }`,
                uniforms: {
                    image: { value: texture },
                    mask: { value: mask },
                    albedo: { value: new THREE.Color(albedoColor) },
                    newAlbedo: { value: new THREE.Color(newAlbedoColor) }
                }
            });

            that.textureScene = new TextureScene(window, canvas, that.material);

            that.textureScene.render();

            that.isLoaded = true;
        };
    }

    /**
     * Update the albedo color and rerender the scene.
     * @param {DOMString } value a hex string for the albedo color in the form #rrggbb.
     */
    updateAlbedo(value) {
        if (this.isLoaded) {
            this.material.uniforms.albedo.value = new THREE.Color(value);
            this.textureScene.render();
        }
    }

    /**
     * Update the new albedo color and rerender the scene.
     * @param {DOMString } value a hex string for the new albedo color in the form #rrggbb.
     */
    updateNewAlbedo(value) {
        if (this.isLoaded) {
            this.material.uniforms.newAlbedo.value = new THREE.Color(value);
            this.textureScene.render();
        }
    }
}

export { AlbedoRecoloringDemo };
