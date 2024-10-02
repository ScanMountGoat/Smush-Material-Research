# Vertex Attributes
Vertices in Smash Ultimate contain multiple attributes. Every mesh requires Position0, Tangent0, and Normal0. 
Additional attributes are required by some shaders for certain effects like texture mapping or baked lighting.
For a brief overview of all the types of attributes, see the GitHub repository's [Vertex Attributes](https://github.com/ScanMountGoat/Smush-Material-Research/blob/master/Vertex%20Attributes.md)

## Attribute Errors (Yellow Checkerboard)
Smash Ultimate has builtin error handling for cases when a mesh is required attributes for rendering. 
Each object in a mesh should have a material assigned in the model.numdlb. This assigned material has an assigned 
shader based on the shader label attribute. The shader is compiled code that runs on the GPU and expects certain attributes to be present.
If the game detects that any of these attributes are missing, a yellow checkerboard is rendered instead.

Meshes should be checked in [SSBH Editor](https://github.com/ScanMountGoat/ssbh_editor) for potential missing attributes. The application 
displays warnings if attributes are missing and provide an easy way to add default values for missing attributes if necessary. 
Missing attributes can also be fixed by using a shader that doesn't require those attributes or adding the attributes in a model editor like Blender and exporting the model.numshb again.