# Debugging
## Introduction
Debugging is vital to successfully reverse engineering the rendering techniques for a game like Smash Ultimate. 
The goal is not simply to find out *what* a model or visual effect will look like in game. This question is easily answered by booting up the game or using an emulator.
The primary goal of rendering research is to figure out *how* a particular effect is achieved and even *why* the game developers may have chosen to implement it in a particular way.
For example, it's easy to see the result of animations by playing the game. It's not obvious that *how* this is achieved involves a compute shader pass at the start of the frame 
rather than traditional techniques like vertex shader skinning. Debugging tools make it straightforward to discover this by inspecting the rendering API calls in Ryujinx or Yuzu emulator.
This helps answer the question of *why* the game uses compute shader skinning instead of vertex shader skinning. One likely theory is that the developers wanted to avoid applying the vertex skinning in the 
vertex shader since this would run the code in both the depth only pass for shadowmapping and the main color pass. A compute shader ensures each vertex is transformed only once and simplifies 
the vertex shader code. Investigating answers to these questions helps understand what visual effects are possible in game and what steps should be taken by mod authors to achieve them.

## Debugging Process
The basic process when investigating a particular visual effect or technique in game is to investigate the rendering calls in an unmodified scene or a scene with known behavior. 
This serves as the "control" for comparing with the modified scene later. The next step is to modify the scene such as changing the value of CustomVector13 for a particular material 
and then record the rendering calls again. Ideally, only one value in game should be changed in game and the scene should be easy to reproduce like a specific frame of an animation 
in traning mode with the same fighters. This helps avoid ambiguous cases where it is hard to tell which modified file caused which rendering call change. With experience, it's possible to 
perform multiple edits at a time and still confidently analyze the scene. This speeds up testing but can easily lead to incorrect or inconclusive results. The final step is to anayze the changes in the 
rendering commands to determine what a particular value in the input files does. For example, checking "Alpha Sample to Coverage" in the material sets the setting with the same name 
in the pipeline's fragment state. 

It's important to note that some settings in the rendering pipeline state are affected by multiple values or only change in certain circumstances.
This can make it seem like a value does nothing or does something unexpected. For example, not all material values are actually referenced anywhere in the shader code. 
It can be helpful to test multiple different models or scenes that use a particular value. Dumping all possible values for a parameter from the game files can help predict if a value is used or not. 
It's not always the case that values that only use a single value in game are unused or values with many unique values actually do something in game, however. 

It's generally not feasible or even valuable for modding to completely reverse engineer all of the game's rendering code or test every possible parameter combination. 
An efficient way to produce reasonably good understanding is to start with a basic guess of how something works, find cases that don't match up with in game, and try to find 
the value or values responsible for the difference via debugging. Many improvements in material and rendering research have been found by analyzing why the output of the renderer 
in applications like SSBH Editor does not match in game. It's important to discover why these discrepencies exist even if perfectly recreating them in applications is deemed not worth the effort.

## Testing
Testing is a vital part of validating not only application code but also understanding of how the in game rendering works. Without access to the original code or specification, it's unlikely that 
implementations or research will ever be 100% accurate. Exhaustively comparing an application code with the output from in game would work in theory but take far too long in practice. 

### Image Output Comparison
The most straightforward way to test rendering code is by comparing the final rendered output from a renderer like ssbh_wgpu with in game. 
This would ideally use screenshots from the Nintendo Switch itself, but emulators can still do a good job at creating accurate output.
This technique is an easy way to find obvious visual errors but has a number of flaws. Testing like this can be slow since the process of launching the game and recording images takes time. 
The results are often unhelpful since spotting visual differences doesn't tell you *why* an issue is occurring. It can also be hard to isolate issues to a single part of the rendering process.
If the lighting is known to not be accurate yet, it can be hard to isolate the errors in material coloring.

### Graphical Debugger Captures
Another method for testing is to compare the application and an emulator using a debugger like RenderDoc. 
The advantage of using a debugger is that it's possible to easily spot differencces in API calls that may not be possible from just screenshots. 
Discovering which values affect certain state like alpha blending is trivial using this technique by simplying comparing the pipeline state by two meshes.
This technique also has the advantage of being able to edit shaders in RenderDoc and obvserve the changes to the rendered image. 
The sequence of API calls and shader code in the emulator can be used as a reference for the application. 

It can even be beneficial in some cases to translate sections of shader code or recreate sequences of API calls. Blindly copying API calls and shader code can quickly create an unmaintainable mess in the application code.
A perfect 1:1 creation of the in game code might produce accurate output, but it won't help modders or application developers make informed decisions if it can't be understood.
There will always be a tradeoff between rendering accuracy and development effort. 

One approach is to try and come up with the simplest possible solution that explains the 
rendering results seen in game. This simple, readable solution can be tested by comparing the application outputs with the "reference" solution of the emulator. RenderDoc allows for editing shader code, which can 
be an easy way to quickly test different parameter values and isolate sections of the code. This makes it easy to test the same scenario for both the application code and the game. 
It's important to establish the relationships between values even if the math isn't obvious right away. 
For example, it's easy to see that colorSet1 makes the model brighter or darker by changing vertex attribute values in the shader in RenderDoc. A value of 0 makes the model black and a value of 1 makes the model bright. 
This sounds like multiplication, so an application might express this as `color = color * colorSet1.rgb`. 
More careful analysis by analyzing the game's vertex shader code in RenderDoc will reveal that the actual code should be `color = color * colorSet1.rgb * 2.0`.

### Software Code Testing
It's also beneficial to test the code itself using traditional software testing techniques. This includes tests like unit testing, fuzz testing, integration tests, etc.
Testing shader and rendering code tends using traditonal tests like unit tests tends to be not very straightforward since rendering calls typically modify memory or state on the GPU and don't return meaningful values.
It's not impossible to adapt testing techniques to work with rendering code, but it can be difficult to maintain due to the complexities in moving data to and from the CPU and GPU and configuring rendering state.

Code that just runs on the CPU can easily be tested using traditional testing techniques. For example, the ssbh_lib repository for loading rendering related file formats uses unit tests as well as fuzz testing for checking how invalid inputs are handled. 

ssbh_wgpu itself uses unit testing for the skeletal animation code since it runs on the CPU. 
Fuzz tests check for basic handling of missing or invalid files to check for potential crashes. The shader and rendering code itself is not tested due to complexities in writing and maintaining tests. This saves time writing tests but has the side effect of 
the rendering code being easy to break accidentally while modifying the code. ssbh_wgpu instead uses batch rendering for quickly rendering screenshots of multiple models to check for any obvious visual errors. This also helps check for any API errors or crashes while loading and rendering models. 100% accuracy to in game rendering is not a goal of ssbh_wgpu, so minor visual errors are generally ok as long as the application is stable and gives a mostly accurate representation of the scene.

## Debugging Tools
A great tool for graphical debugging is [RenderDoc](https://renderdoc.org/). RenderDoc can debug OpenGL, Vulkan, and DirectX applications. 
Some hardware manufactures provide their own dedicated debugging tools like AMD, Nvidia, or Apple. 
These tools should also work for debugging but tend to be more focused towards application developers wanting to optimize their apps.
It's not currently possible to debug the game itself with RenderDoc. Thankfully, emulators like Ryujinx and Yuzu run on a PC and can be easily debugged with RenderDoc. The Tegra X1 uses a modern architecture, and most graphics APIs use 
very similar concepts with different names like "pixel shaders" instead of "fragment shaders". While emulators don't perfectly emulate the Switch and have the occasional visual bug, access to 
powerful and convenient debugging tools on a PC for outweighs any potential inaccuracies. 

It's recommended to use Ryujinx with OpenGL instead of Vulkan. 
The Vulkan renderer has had minor issues like incorrect alpha testing for shadow mapping. These issues tend to get resolved as the emulator evolves. 
The main reason to use OpenGL is that the decompiled shaders can be easily edited in plaintext in RenderDoc and match the output of the decompiled shader dump produced by Ryujinx's shader tools.
Yuzu also can be made to work, but the rendering commands and shaders in Yuzu are structured differently than Ryujinx and won't match up with the research on this repository.
Ryujinx has also the benefit of currently working with [Arcropolis](https://github.com/Raytwo/ARCropolis) for loading mods.

Ryujinx's OpenGL implementation won't work with RenderDoc out of the box and requires a few tweaks. The steps are listed below.
1. Install the version of .NET used by Ryujinx. Visual Studio is recommended for editing but any text editor will work.
2. Clone the repository from https://github.com/Ryujinx/Ryujinx
3. Remove the calls to alpha testing by deleting or commenting out the lines in `Ryujinx.Graphics.OpenGL.Pipeline.SetAlphaTest`.
4. Build Ryujinx.
5. Point RenderDoc to this new executable in the Launch Application tab of RenderDoc.

The basic process for debugging is as follows.
1. Launch the emulator from RenderDoc
2. Set the graphics API to OpenGL if it isn't already.
3. Open Smash Ultimate and find an scene to analyze.
4. Hit the capture hotkey (F12) to record an API capture. It's recommended to save this to disk to save time later.
5. Open the capture and investigate the the textures, meshes, shaders, API calls, etc. See the [RenderDoc guide](https://renderdoc.org/docs/getting_started/quick_start.html) for more information. 
