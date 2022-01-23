// Cube map loading adapted from code by https://github.com/AngusLang.
// https://threejs.org/examples/webgl_materials_cubemap_mipmaps.htmlexport

import * as THREE from "./three.module.js"

/**
 * Loads cube map faces with mipmaps from directory. 
 * Example: "directory/cube_f2_m0.jpg" is the 2nd face and the first mip level.
 * @param {String} directory the source directory for the cube map faces
 * @param {String} extension the image file extension
 * @param {Number} maxLevel the maximum mip level
 */
export async function loadCubeMapWithMipmapsAsync(directory, extension, maxLevel) {
    const mipmaps = [];

    async function loadCubeTexture(urls) {
        return new Promise(function (resolve) {
            new THREE.CubeTextureLoader().load(urls, function (cubeTexture) {
                resolve(cubeTexture);
            });
        });
    }

    // load mipmaps
    const pendings = [];

    for (let level = 0; level <= maxLevel; ++level) {
        const urls = [];

        for (let face = 0; face < 6; ++face) {
            urls.push(directory + "f" + face + "_m" + level + extension);
        }

        const mipmapLevel = level;
        pendings.push(loadCubeTexture(urls).then(function (cubeTexture) {
            mipmaps[mipmapLevel] = cubeTexture;
        }));
    }

    await Promise.all(pendings);

    const cubeMap = mipmaps.shift();
    cubeMap.mipmaps = mipmaps;
    // TODO: Add mipmaps.
    cubeMap.minFilter = THREE.LinearMipMapLinearFilter;
    cubeMap.magFilter = THREE.LinearFilter;
    cubeMap.generateMipmaps = false;
    cubeMap.needsUpdate = true;

    return cubeMap;
}