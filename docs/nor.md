---
---
# NOR Maps (Texture4)
Similar to PRM maps, NOR maps are actually composed of a few different textures.
The red and green channels correspond to the X+ and Y+ direction of the normal map. 
The blue channel is a transition blend map for material transitions. 
The alpha channel is a cavity map. 

## NOR Color Channels
NOR maps do not contain color data despite being stored in a texture.
Avoid saving NOR maps as DDS or Nutexb with
srgb formats. Srgb formats have names that end in _SRGB. When rendering in programs such as Maya or Blender, set the NOR
maps to raw, non color data, or linear to ensure
the values aren't gamma corrected.

### Normal Map X+, Y+ (<span style="color:red">Red</span>, <span style="color:green">Green</span>)
<figure class="figure">
<img src="{{ "/assets/images/nor/mario_normal.jpg" | relative_url }}" height="auto" width="auto">
    <figcaption class="figure-caption text-center">The model with a blank NOR map (left), the XY channels from the NOR map and calculated z channel (center), and the model with the NOR map (right)</figcaption>
</figure>
The red channel contains the x component of the tangent space normal. A value of 1.0 is positive x, 0.0 is negative x, and 0.5 has no effect. 

The green channel contains the y component of the tangent space normal. 
A value of 1.0 is positive y, 0.0 is negative y, and 0.5 has no effect. 
This is the standard channel layout for OpenGL normal maps, which may also be called "Y+" in some applications. The green channel should be flipped when using normal maps from DirectX or other applications that use Y- for the green channel.

The normal map z component is calculated in the shader using the following calculation.
```glsl
// Remap the 0 to 1 range of the normal map to -1 to 1.
float x = 2 * norColor.x - 1.0;
float y = 2 * norColor.y - 1.0;
// x*x + y*y + z*z = 1.0
float z = sqrt(1 - (x * x) + (y * y));
// Map back to 0 to 1 range to get the equivalent texture color.
float normalMapZ = z * 0.5 + 0.5;
```
### Transition Blend Map (<span style="color:blue">Blue</span>)
The blue channel contains the transition blend map used for transitioning between materials. 
See [Material Transitions](material_transitions) for details and an interactive demo.

Some stage models store the z component of the normal map in the NOR blue channel instead of a transition blend map.

### Cavity Map (Alpha)
<img src="{{ "/assets/images/nor/mario_cavity.jpg" | relative_url }}" height="auto" width="auto">
The alpha channel contains the cavity map, which occludes specular reflections and rim lighting. 
Unlike the ambient occlusion map in the PRM blue channel, cavity maps do not affect diffuse ambient lighting.
The shaders for some models don't access the blue channel of the NOR map, so changing the cavity map may have no effect.