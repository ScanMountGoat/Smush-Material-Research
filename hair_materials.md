# Hair Materials - WIP
Hair materials have separate parameters to control specular highlights to better recreate the way light interacts with hair. The highlights on hair materials are not symmetrical and appear to wrap around the surface. The anisotropic (not symmetrical) specular can be used for recreating the look strands of hair or brushed metal without having to model or texture the fine surface detail. 

## Anisotropic Specular
CustomFloat10 controls the amount of anisotropy. This works identically to the anisotropy input of Blender's 
<a href="https://docs.blender.org/manual/en/latest/render/shader_nodes/shader/principled.html" target="_blank">Principled BSDF<\a>.
A value of 0.0 produces a round, symmetrical highlight. Higher values produce a more stretched highlight. 

## Anisotropic Rotation
Some materials use the PRM alpha value as anisotropic rotation instead of controlling specular. A value of 0.0 is no rotation. A value of 0.5 rotates the highlight by 90 degrees. A value of 1.0 rotates the highlight by 180 degrees. Anisotropic rotation can be used to fix the highlight direction on surfaces with UV layouts that cause the highlight to appear rotated or distorted. 
If the hair strands run from top to bottom on the texture and the hair planes are UV mapped to match, there is usually no need to correct the rotation of the highlight using the PRM alpha. 