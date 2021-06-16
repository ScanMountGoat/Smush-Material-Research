import * as THREE from "./three.module.js";
import { PerspectiveScene } from "./perspectivescene.js";
import { OrbitControls } from "./orbitcontrols.js";

class DifCubeDemo {
    constructor(window, canvas, difCubeDirectory) {
        this.isLoaded = false;

        const manager = new THREE.LoadingManager();

        const cubeTexture = new THREE.CubeTextureLoader(manager)
            .setPath(difCubeDirectory)
            .load([
                'posx.png',
                'negx.png',
                'posy.png',
                'negy.png',
                'posz.png',
                'negz.png'
            ]);

        const that = this;
        manager.onLoad = function () {
            that.material = new THREE.ShaderMaterial({
                vertexShader: `
                    varying vec2 vUv;
                    varying vec3 vNormal;
                    varying vec3 vPosition;
                    
                    void main() {
                        vUv = uv;
                        vNormal = normal;
                        vPosition = position.xyz;
                        gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);  
                    }`,
                fragmentShader: `
                    varying vec2 vUv;
                    varying vec3 vNormal;
                    varying vec3 vPosition;
    
                    uniform samplerCube difCube;
    
                    void main() {
                        vec3 normal = normalize(vNormal);
                        vec3 viewVector = normalize(cameraPosition - vPosition);
    
                        vec3 reflectionVector = reflect(viewVector, normal);
                        reflectionVector.y *= -1.0;
    
                        vec3 result = texture(difCube, reflectionVector).rgb;
                        gl_FragColor = vec4(result,1.0);
                    }`,
                uniforms: {
                    difCube: { value: cubeTexture }
                }
            });

            that.scene = new PerspectiveScene(window, canvas, that.material);
            that.scene.sphere.visible = true;
            
            const controls = new OrbitControls(that.scene.camera, canvas);
            controls.enablePan = false;
            controls.enableZoom = false;
            controls.autoRotate = true;
            controls.update();

            that.isLoaded = true;

            const animate = function () {
                controls.update();
                that.scene.render();
                requestAnimationFrame(animate);
            }

            animate();
        };
    }

    setSphere() {
        if (this.isLoaded) {
            this.scene.cube.visible = false;
            this.scene.sphere.visible = true;
        }
    }

    setCube() {
        if (this.isLoaded) {
            this.scene.cube.visible = true;
            this.scene.sphere.visible = false;
        }
    }
}

export { DifCubeDemo };
