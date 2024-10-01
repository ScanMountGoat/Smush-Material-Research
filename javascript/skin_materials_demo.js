import * as THREE from "./three.module.js";
import { PerspectiveScene } from "./perspectivescene.js";

class SssDemo {
    constructor(window, canvas, albedo, customVector11, customVector30x, customVector30y, metalness) {
        this.material = new THREE.ShaderMaterial({
            vertexShader: `
                varying vec2 vUv;
                varying vec3 vNormal;
                
                void main() {
                    vUv = uv;
                    vNormal = normal;
                    gl_Position = projectionMatrix * modelViewMatrix * vec4( position, 1.0 );  
                }`,
            fragmentShader: `
                varying vec2 vUv;
                varying vec3 vNormal;

                // ZW are unused.
                uniform vec3 CustomVector11;
                uniform float CustomVector30x;
                uniform float CustomVector30y;
                uniform vec3 albedo;
                uniform float metalness;

                void main() {
                    vec3 normal = normalize(vNormal);
                    float nDotL = dot(normal, normalize(vec3(-0.5,1,0.5)));

                    float sssBlend = metalness * CustomVector30x;

                    // Adjust diffuse shading.
                    float skinShading = nDotL * CustomVector30y;
                    skinShading = clamp(skinShading * 0.5 + 0.5, 0.0, 1.0);
                    vec3 skinFinal = CustomVector11.rgb * sssBlend * skinShading;

                    vec3 direct = mix(nDotL * albedo.rgb, skinFinal, sssBlend);

                    // Adjust albedo.
                    vec3 albedoFinal = albedo.rgb;
                    albedoFinal = mix(albedoFinal, CustomVector11, sssBlend);
                    vec3 ambient = albedoFinal;

                    vec3 result = mix(direct, ambient, 0.5);
                    
                    // Gamma correction.
                    result = pow(result, vec3(1.0 / 2.2));
                    result.rgb *= 1.5;
                    gl_FragColor = vec4(result,1.0);
                }`,
            uniforms: {
                albedo: { value: new THREE.Color(albedo) },
                CustomVector11: { value: new THREE.Color(customVector11) },
                CustomVector30x: { value: customVector30x },
                CustomVector30y: { value: customVector30y },
                metalness: { value: metalness }
            }
        });

        // Draw a sphere.
        this.scene = new PerspectiveScene(window, canvas, this.material);
        this.scene.sphere.visible = true;

        this.scene.render();
    }

    updateAlbedo(value) {
        this.material.uniforms.albedo.value = new THREE.Color(value).convertSRGBToLinear();
        this.scene.render();
    }

    updateCustomVector11(value) {
        this.material.uniforms.CustomVector11.value = new THREE.Color(value).convertSRGBToLinear();
        this.scene.render();
    }

    updateMetalness(value) {
        this.material.uniforms.metalness.value = value;
        this.scene.render();
    }

    updateCustomVector30x(value) {
        this.material.uniforms.CustomVector30x.value = value;
        this.scene.render();
    }

    updateCustomVector30y(value) {
        this.material.uniforms.CustomVector30y.value = value;
        this.scene.render();
    }
}

export { SssDemo };
