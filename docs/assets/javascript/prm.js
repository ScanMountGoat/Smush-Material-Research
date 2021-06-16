import * as THREE from "./three.module.js";
import { loadCubeMapWithMipmapsAsync } from "./cubemap.js";
import { PerspectiveScene } from "./perspectivescene.js";

class PrmDemo {
    /**
     * 
     * @param {*} window 
     * @param {*} canvas 
     * @param {DOMString} albedo the initial albedo color in the form #rrggbb
     * @param {Number} mtl the initial metalness
     * @param {Number} rgh the initial roughness
     * @param {Number} ao the initial ambient occlusion
     * @param {Number} spc the initial specular
     * @param {String} envDirectory the directory for the cube map faces
     * @param {Number} envMaxLevel the max mip level for the cube map faces
     * @param {String} envExtension the extension of the cube map face images
     */
    constructor(window, canvas, albedo, mtl, rgh, ao, spc, envDirectory, envMaxLevel, envExtension) {
        this.isLoaded = false;

        const that = this;
        loadCubeMapWithMipmapsAsync(envDirectory, envExtension, envMaxLevel).then(function (cubeTexture) {
            that.material = new THREE.ShaderMaterial({
                vertexShader: `
                    varying vec2 vUv;
                    varying vec3 vNormal;
                    varying vec3 vPosition;
                    
                    void main() {
                        vUv = uv;
                        vNormal = normal;
                        vPosition = position;
                        gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);  
                    }`,
                fragmentShader: `
                    varying vec2 vUv;
                    varying vec3 vNormal;
                    varying vec3 vPosition;
    
                    uniform vec3 albedo;

                    uniform samplerCube env;
    
                    // PRM Channels
                    uniform float metalness;
                    uniform float roughness;
                    uniform float ambientOcclusion;
                    uniform float specular;
    
                    // Shader code ported from Cross Mod GUI
                    // https://github.com/Ploaj/SSBHLib/blob/master/CrossModGui/Shaders/RModel.frag
    
                    // Schlick fresnel approximation.
                    vec3 FresnelSchlick(float cosTheta, vec3 F0)
                    {
                        return F0 + (1.0 - F0) * pow(1.0 - cosTheta, 5.0);
                    } 
    
                    // Ultimate shaders use a schlick geometry masking term.
                    // http://cwyman.org/code/dxrTutors/tutors/Tutor14/tutorial14.md.html
                    float SchlickMaskingTerm(float nDotL, float nDotV, float a2) 
                    {
                        // TODO: Double check this masking term.
                        float k = a2 * 0.5;
                        float gV = nDotV / (nDotV * (1.0 - k) + k);
                        float gL = nDotL / (nDotL * (1.0 - k) + k);
                        return gV * gL;
                    }
    
                    // Ultimate shaders use a standard GGX BRDF for specular.
                    // http://graphicrants.blogspot.com/2013/08/specular-brdf-reference.html
                    float Ggx(float nDotH, float nDotL, float nDotV, float roughness)
                    {
                        // Clamp to 0.01 to prevent divide by 0.
                        float a = max(roughness, 0.01) * max(roughness, 0.01);
                        float a2 = a*a;
                        const float PI = 3.14159;
                        float nDotH2 = nDotH * nDotH;
    
                        float denominator = ((nDotH2) * (a2 - 1.0) + 1.0);
                        float specular = a2 / (PI * denominator * denominator);
                        float shadowing = SchlickMaskingTerm(nDotL, nDotV, a2);
                        // TODO: missing 4.0*nDotL*nDotV in denominator?
                        return specular * shadowing;
                    }

                    float RoughnessToLod(float roughness)
                    {
                        // Adapted from decompiled shader source.
                        // Applies a curves adjustment to roughness.
                        // Clamp roughness to avoid divide by 0.
                        float roughnessClamped = max(roughness, 0.01);
                        float a = (roughnessClamped * roughnessClamped);
                        return log2((1.0 / a) * 2.0 - 2.0) * -0.4545 + 4.0;
                    }
    
                    vec3 GetSpecularWeight(float f0, vec3 diffusePass, float metalness, float nDotV, float roughness)
                    {
                        // Metals use albedo instead of the specular color/tint.
                        vec3 specularReflectionF0 = vec3(f0);
                        vec3 f0Final = mix(specularReflectionF0, diffusePass, metalness);
                        return FresnelSchlick(nDotV, f0Final);
                    }
    
                    void main() {
                        vec3 viewVector = normalize(cameraPosition - vPosition);
    
                        vec3 normal = normalize(vNormal);
                        vec3 lightDirection = normalize(vec3(-0.383022249,0.866025,0.32139495));
                        float nDotL = max(dot(normal, lightDirection), 0.0);
    
                        vec3 halfAngle = normalize(lightDirection + viewVector);
                        float nDotV = max(dot(normal, viewVector), 0.0);
                        float nDotH = max(dot(normal, halfAngle), 0.0);

                        vec3 reflectionVector = reflect(viewVector, normal);
                        reflectionVector.y *= -1.0;

                        float specularLod = RoughnessToLod(roughness);
                        vec3 specularIbl = textureLod(env, reflectionVector, specularLod).rgb;
                        vec3 diffuseIbl = textureLod(env, reflectionVector, 5.0).rgb;
    
                        vec3 kSpecular = GetSpecularWeight(specular * 0.2, albedo, metalness, nDotV, roughness);
                        float specularBrdf = Ggx(nDotH, nDotL, nDotV, roughness);
    
                        float directLightIntensity = 4.0;
    
                        vec3 kDiffuse = max((vec3(1.0) - kSpecular) * (1.0 - metalness), 0.0);
                        vec3 diffuseLighting = (nDotL * directLightIntensity) + (diffuseIbl * ambientOcclusion);
    
                        vec3 result = kDiffuse * albedo * diffuseLighting / 3.14159;

                        vec3 specularLighting = (specularBrdf * directLightIntensity) + specularIbl;
                        result += kSpecular * specularLighting * ambientOcclusion;
    
                        // Gamma correction.
                        result = pow(result, vec3(1.0 / 2.2));
                        gl_FragColor = vec4(result,1.0);
                    }`,
                uniforms: {
                    albedo: { value: new THREE.Color(albedo).convertSRGBToLinear() },
                    metalness: { value: mtl },
                    roughness: { value: rgh },
                    ambientOcclusion: { value: ao },
                    specular: { value: spc },
                    env: { value: cubeTexture }
                }
            });

            // Draw a sphere.
            that.scene = new PerspectiveScene(window, canvas, that.material);
            that.scene.sphere.visible = true;

            that.scene.render();
            that.isLoaded = true;
        });
    }

    updateAlbedo(value) {
        if (this.isLoaded) {
            this.material.uniforms.albedo.value = new THREE.Color(value).convertSRGBToLinear();
            this.scene.render();
        }
    }

    updateMetalness(value) {
        if (this.isLoaded) {
            this.material.uniforms.metalness.value = value;
            this.scene.render();
        }
    }

    updateRoughness(value) {
        if (this.isLoaded) {
            this.material.uniforms.roughness.value = value;
            this.scene.render();
        }
    }

    updateAmbientOcclusion(value) {
        if (this.isLoaded) {
            this.material.uniforms.ambientOcclusion.value = value;
            this.scene.render();
        }
    }

    updateSpecular(value) {
        if (this.isLoaded) {
            this.material.uniforms.specular.value = value;
            this.scene.render();
        }
    }
}

export { PrmDemo };
