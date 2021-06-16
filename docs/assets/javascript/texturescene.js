import * as THREE from "./three.module.js"

/**
 * Stores the state necessary for drawing a textured quad.
 */
class TextureScene {
    constructor(window, canvas, material) {
        this.renderer = new THREE.WebGLRenderer({
            canvas: canvas,
            alpha: true
        });
        this.updateRenderDimensions(window);

        const textureScene = this;
        window.addEventListener('resize', function () {
            textureScene.updateRenderDimensions(window);
            textureScene.render();
        });
    
        this.scene = new THREE.Scene();
        this.camera = new THREE.OrthographicCamera(-1, 1, 1, -1, 0, 1);

        const quad = new THREE.Mesh(new THREE.PlaneBufferGeometry(2, 2, 1, 1), material);
        this.scene.add(quad);
    }

    render() {
        this.renderer.render(this.scene, this.camera);
    }

    /**
     * Set the renderer dimensions to the max dimension of the canvas.
     * This assumes a 1:1 aspect ratio but improves the output resolution.
     */
    updateRenderDimensions(window) {
        const maxDimension = Math.max(this.renderer.domElement.clientWidth, this.renderer.domElement.clientHeight);

        // Set the pixel ratio to set the correct resolution for high PPI displays.
        this.renderer.setPixelRatio(window.devicePixelRatio);
        this.renderer.setSize(maxDimension, maxDimension, false);
    };
}

export { TextureScene };