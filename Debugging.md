# Debugging
## Introduction
Debugging is vital to successfully reverse engineering the rendering techniques for a game like Smash Ultimate. The goal is not simply to find out *what* a model or visual effect will look like in game. This question is easily answered by booting up the game or using an emulator. The primary goal of rendering research is to figure out *how* a particular effect is achieved and even *why* the game developers may have chosen to implement it in a particular way. Investigating answers to these questions helps understand what visual effects are possible in game and what steps should be taken by mod authors to achieve them.

It's easy to see the result of animations by playing the game. It's not obvious that *how* this is achieved involves a compute shader pass at the start of the frame rather than traditional techniques like vertex shader skinning. Debugging tools make it straightforward to discover this by inspecting the rendering API calls in Ryujinx or Yuzu emulator. 

Understanding *how* an effect is achieved allows for recreating it in code. Recreating rendering in code can be beneficial for testing understanding of the rendering techniques by comparing with the game itself. Having a shared code implementation of the game's rendering enables the development of applications and tools for modding. The rendering code implemented in [ssbh_wgpu](https://github.com/ScanMountGoat/ssbh_wgpu) is used to create the 3D viewport for applications like [SSBH Editor](https://github.com/ScanMountGoat/ssbh_editor). The ssbh_wgpu code also serves as a point of reference when developing and fixing the Blender addon [Smash-Ultimate-Blender](https://github.com/ssbucarlos/smash-ultimate-blender).

This helps answer the question of *why* the game uses compute shader skinning instead of vertex shader skinning. One likely theory is that the developers wanted to avoid applying the vertex skinning in the vertex shader since this would run the code in both the depth only pass for shadow mapping and the main color pass. A compute shader ensures each vertex is transformed only once and simplifies the vertex shader code.

## Overview of GPU Rendering
GPU rendering consists of multiple processing stages known as the [rendering pipeline](https://www.khronos.org/opengl/wiki/Rendering_Pipeline_Overview). Data is loaded into buffers and uploaded to the GPU for rendering. The data moves through multiple stages before finally being rendered on screen. Graphics API like OpenGL, Vulkan, NVN, etc allow programmers to configure how processing is done in each of these stages by calling API functions. Some stages allow for compiling and running custom programs called shaders. Once all the state is configured, the programmer issues a "draw call" to actually draw the model. It's also worth noting that modern GPU architectures like the Tegra X1 also support compute shaders, which run a single processing stage using a programmable shader often called a "kernel". Submitting work for compute shaders is typically called "dispatching" since compute shaders are often used for tasks other than drawing like animating model geometry.

## Why Debug an Emulator
For reverse engineering in game rendering, testing on actual hardware seems like the obvious choice. Changes to rendering related files will always be reflected accurately in  copies of the game running on actual hardware. This treats the game's rendering engine like a black box where only the inputs are controlled and only the final outputs can be observed. Testing in this way can also be time consuming due to repeatedly loading the game. This also assumes the existence of file modding tools. Limitations in mod loading early in the modding scene for Ultimate made this approach quite limiting in terms of what could be analyzed. Thankfully, there are some useful properties of how modern GPU rendering is done that allows for a far more powerful and efficient workflow.

Programmers very rarely issue hardware specific commands directly, so all rendering commands are issued through a graphics API. This provides an abstraction over the GPU hardware's internal state and rendering capabilities. The configured state of the GPU and the specified input data determine the final rendered result. **If all the graphics API calls and their inputs are known, it's possible to perfectly recreate the in game rendering of a single frame without needing to analyze any of the game's code.** GPU debugging programs like RenderDoc work by recording the API calls and their inputs and can even save them to disk to play back and analyze later. This is similar to analyzing the final HTML and CSS using developer tools in a web browser. This also allows investigating the rendering "in progress" by monitoring the GPU state at various events in the captured frame. This makes analyzing changes to rendering related input files far more accurate and repeatible compared to analyzing screenshots.

It's not possible to debug the game running on the Switch itself with RenderDoc to inspect the actual API calls. Thankfully, emulators like Ryujinx and Yuzu run on a PC and can be easily debugged with RenderDoc. 

The Tegra X1 uses a modern GPU architecture similar to desktop GPUs. Most graphics APIs use very similar concepts with different names like "pixel shaders" instead of "fragment shaders". It's generally safe to assume that each OpenGL or Vulkan API call in the emulator has a corresponding call or set of calls in the game itself. This assumption does not hold for older consoles with non modern hardware like the Wii or Gameboy or consoles with hard to emulate features. While emulators don't perfectly emulate the Switch and have the occasional visual bug, the benefits of debugging tools on a PC far outweigh any minor potential inaccuracies.

## Debugging Process
The basic process when investigating a particular visual effect or technique in game is to investigate the rendering calls in an unmodified scene or a scene with known behavior. This serves as the "control group" for comparing with the modified scene later.  

The next step is to modify the scene such as changing the value of CustomVector13 for a particular material 
and then record the rendering calls again. Ideally, only one file value in should be changed at a time. The scene should be easy to reproduce like a specific frame of an animation in training mode with the same set of fighters. This helps avoid ambiguous cases where it is hard to tell which modified file caused which rendering call change. With experience, it's possible to perform multiple edits at a time and still confidently analyze the scene. This speeds up testing but can easily lead to incorrect or inconclusive results.  

The final step is to anayze the changes in the rendering commands to determine what a particular value in the input files does. For example, checking "Alpha Sample to Coverage" in the material sets the setting with the same name in the pipeline's fragment state. Many of the researched field names for rendering related files come directly from these tests.

### Limitations
It's important to note that some settings in the rendering pipeline state are affected by multiple values or only change in certain circumstances. This can make it seem like a value does nothing or does something unexpected. For example, not all material values required in the nufxlb are actually referenced anywhere in the shader code. It can be helpful to test multiple different models or scenes that use a particular value. Dumping all possible values for a parameter from the game files can help predict if a value is used or not since parameters with many unique values are likely important. It's not always the case that values that only use a single value in game are unused or values with many unique values actually do something in game, however. 

It's generally not feasible or even valuable for modding to completely reverse engineer all of the game's rendering code or test every possible parameter combination. An efficient way to produce reasonably good understanding is to start with a basic guess of how something works, find cases that don't match up with in game, and try to find the value or values responsible for the difference via debugging. The initial guess can come from anywhere like a previous game or research paper that would have been available during the game's development. Many improvements in material and rendering research have been found by analyzing why the output of the renderer in applications like SSBH Editor does not match in game. It's important to discover why these discrepencies exist even if fixing them in the application code is deemed not worth the effort. It's necessary in some cases to sacrifice rendering accuracy to achieve better performance or compatibility on PC hardware.

## Testing
Testing is a vital part of validating not only application code but also understanding of how the in game rendering works. Without access to the original code or specification, it's unlikely that code implementations or research will ever be 100% accurate. Exhaustively comparing an application code with the output from in game works in theory but would take far too long in practice. 

### Image Output Comparison
The most straightforward way to test rendering code is by comparing the final rendered output from a renderer like ssbh_wgpu with in game. 
This would ideally use screenshots from the Nintendo Switch itself, but emulators can still do a good job at creating accurate output.
This technique is an easy way to find obvious visual errors but has a number of flaws. Testing like this can be slow since the process of launching the game and recording images takes time. The results are often unhelpful since spotting visual differences doesn't tell you *why* an issue is occurring. It can also be hard to isolate issues to a single part of the rendering process. If the application's lighting code is known to not be accurate yet, it can be hard to isolate the errors in material coloring.

### Graphical Debugger Captures
![image](https://user-images.githubusercontent.com/23301691/226055125-44453c88-af82-4d1a-bc8c-65fb2c11bfde.png)

Another method for testing is to compare the application and an emulator using a debugger like RenderDoc. RenderDoc is similar to devtools available in modern browsers, and both are usually accessed by hitting F12. The advantage of using a debugger is that it's possible to easily spot differences in API calls that may not be discernible in screenshots. 
Discovering which values affect certain state like alpha blending is trivial using this technique by simplying comparing the pipeline state for two meshes. RenderDoc also has the ability to edit shaders and observe the changes to the rendered image, which can be far faster than editing material files. The sequence of API calls and shader code in the emulator can be used as a reference for the application. 

It can even be beneficial in some cases to translate sections of shader code or recreate sequences of API calls. Blindly copying API calls and shader code from RenderDoc can quickly create an unmaintainable mess in the application code. A perfect 1:1 creation of the in game code might produce accurate output, but it won't help modders or application developers make informed decisions if it can't be understood. There will always be tradeoffs between rendering accuracy, readability, and development effort.

One approach is to try and come up with the simplest possible solution that explains the 
rendering results seen in game. This simple, readable solution can be tested by comparing the application outputs with the "reference" solution of the emulator. This is made easier by the shader editing capabilities in RenderDoc that enable fast iteration. This makes it easy to test the same scenario for both the application code and the game and compare the output.

It's important to establish the relationships between values even if the math isn't obvious right away. 
For example, it's easy to see that colorSet1 makes the model brighter or darker by changing vertex attribute values in the shader in RenderDoc. A value of 0 makes the model black and a value of 1 makes the model bright. 
This sounds like multiplication, so an application might express this as `color = color * colorSet1.rgb;`. 
More careful analysis by analyzing the game's vertex shader code in RenderDoc will reveal that the actual code should be `color = color * colorSet1.rgb * 2.0;`.

### Software Code Testing
It's also beneficial to test the code itself using traditional software testing techniques. This includes tests like unit testing, fuzz testing, integration tests, etc.
Testing shader and rendering code tends using traditional tests like unit tests tends to be not very straightforward since rendering calls typically modify memory or state on the GPU and don't return meaningful values.
It's not impossible to adapt testing techniques to work with rendering code, but it can be difficult to maintain due to the complexities in moving data to and from the CPU and GPU and configuring rendering state.

Code that just runs on the CPU can easily be tested using traditional testing techniques. For example, the ssbh_lib repository for loading rendering related file formats uses unit tests as well as fuzz testing for checking how invalid binary files are handled. 

ssbh_wgpu itself uses unit testing for the skeletal animation code since it runs on the CPU. 
Fuzz tests check for basic handling of missing or invalid files to check for potential crashes. The shader and rendering code itself does not use automated testing due to complexities in writing and maintaining tests. This saves time writing tests but has the side effect of 
the rendering code being easy to break accidentally while modifying the code. ssbh_wgpu instead uses batch rendering for quickly rendering screenshots of multiple models to check for any obvious visual errors. This also helps check for any API errors or crashes while loading and rendering models. 100% accuracy to in game rendering is not a goal of ssbh_wgpu, so minor visual errors are generally ok as long as the application is stable and gives a mostly accurate representation of the scene.

## Debugging Tools
A great tool for graphical debugging is [RenderDoc](https://renderdoc.org/). RenderDoc can debug OpenGL, Vulkan, and DirectX applications. 
Some hardware manufactures provide their own dedicated debugging tools like AMD, Nvidia, or Apple. 
These tools also work for debugging but tend to be more focused towards application developers wanting to optimize their apps.

It's recommended to use Ryujinx with Vulkan. Ryujinx has the benefit of currently working with [Arcropolis](https://github.com/Raytwo/ARCropolis) for loading mods. The Vulkan renderer has good rendering accuracy and performance and doesn't require modifying the emulator. **Compile the Ryujinx.Gtk3 from source for use with RenderDoc to avoid issues attaching RenderDoc to the default Avalonia application.**

The main reason to use OpenGL is that the shaders in RenderDoc match the output of the decompiled shader dump produced by Ryujinx.ShaderTools. Shader editing with Vulkan is still possible but has slightly different formatting in RenderDoc.

Ryujinx's OpenGL implementation won't work with RenderDoc out of the box and requires a few tweaks. The steps are listed below.
1. Install the version of .NET used by Ryujinx. Visual Studio is recommended for editing but any text editor will work.
2. Clone the original repository from a trusted source.
3. Remove the calls to alpha testing by deleting or commenting out the lines in `Ryujinx.Graphics.OpenGL.Pipeline.SetAlphaTest`.
4. Remove the calls to `GL.Begin` and `GL.End` in `Ryujinx.Graphics.OpenGL.HwCapabilities`.
5. Build Ryujinx.
6. Point RenderDoc to the new Ryujinx.Gtk3 executable in the Launch Application tab of RenderDoc.

The basic process for debugging is as follows.
1. Launch the emulator from RenderDoc
2. Set the graphics API to OpenGL if it isn't already.
3. Open Smash Ultimate and find an scene to analyze.
4. Hit the capture hotkey (F12) to record an API capture. It's recommended to save this to disk to save time later.
5. Open the capture and investigate the the textures, meshes, shaders, API calls, etc. See the [RenderDoc guide](https://renderdoc.org/docs/getting_started/quick_start.html) for more information. 

## Learning Resources
The following links cover important compute graphics concepts in a more approachable manner for people with some level of programming experience.
* https://learnopengl.com/
* https://sotrh.github.io/learn-wgpu/

Modern graphics APIs all use similar abstractions, so knowledge from one API tends to carry over to another. For example, a tutorial on shadowmapping may still be helpful for reverse engineering shadows in a game even if the tutorial uses MacOS's Metal API. Knowledge of at least one modern graphics API is critical for understanding the output of RenderDoc.

The WGPU tutorial is helpful for people wanting to contribute to renderers like ssbh_wgpu written in Rust. LearnOpenGL is far more in depth but doesn't cover some more recent concepts. Vulkan tutorials tend to be more modern but often include more complex topics like memory management and synchronization that aren't always relevant to reverse engineering.
