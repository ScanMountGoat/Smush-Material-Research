import * as THREE from "./three.module.js"

/**
 * Stores the state necessary for drawing a cube.
 */
class CubeScene {
    constructor(window, canvas, material) {
        this.renderer = new THREE.WebGLRenderer({
            canvas: canvas,
            alpha: true,
            antialias: true
        });
        this.updateRenderDimensions(window);

        const cubeScene = this;
        window.addEventListener('resize', function () {
            cubeScene.updateRenderDimensions(window);
            cubeScene.render();
        });

        this.scene = new THREE.Scene();
        // Assume a 1:1 aspect ratio.
        this.camera = new THREE.PerspectiveCamera(75, 1.0, 0.1, 1000);
        this.camera.position.z = 2;

        const geometry = new THREE.BoxGeometry(1, 1, 1);
        const cube = new THREE.Mesh(geometry, material);
        this.scene.add(cube);
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

export { CubeScene };