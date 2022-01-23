+++
title = "Col (Texture0, Texture1)"
weight = 0
aliases = ["col"]
+++
Col maps control the albedo of a model. Albedo is the overall color of an object. Surfaces with higher albedo reflect more light and appear brighter than surfaces with 
low albedo. This corresponds to the base color input of
<a href="https://docs.blender.org/manual/en/latest/render/shader_nodes/shader/principled.html" target="_blank">Blender's
    Principled Shader</a>.
This is the primary texture to edit when recoloring a model. PRM and NOR maps can greatly improve the quality of a
model,
but they aren't strictly necessary.

## Col Map Channels
Col maps contain color data, so they should be saved with sRGB formats. sRGB formats have names that end in _SRGB.
When rendering in programs such as Maya or Blender, set the Col maps to Color, sRGB, etc to ensure they are properly
gamma corrected. Failing to use an sRGB format will result in the textures being too bright and looking washed out.

### Albedo/Base Color (<span style="color:red">R</span><span style="color:green">G</span><span style="color:blue">B</span>)
<img src="mario_albedo.jpg" height="auto" width="auto">
Unlike diffuse maps, albedo maps don't contain any baked lighting or shadows. This means most models in game 
will have albedo maps that are mostly solid colors. Details are typically baked into the NOR and PRM maps. 

Avoid using col map values close to pure white (255,255,255) or pure black (0,0,0). Use col map values below 
0.72 (180 RGB) to avoid overly bright models and unwanted glow. A col map set to (180,180,180) will look grey in Photoshop but appear 
almost completely white in game. The [Post Processing Passes](postprocessing) page contains more details on why this happens. 

The [Albedo Recoloring](albedo_recoloring), [PRM](prm), and [Skin Materials](skin_materials) pages all have demos that demonstrate the effects of editing the model's albedo color.

### Opacity (Alpha)
The alpha channel of the col map controls the opacity of the model. A value of 0.0 is completely transparent, and a value of 1.0 is completely opaque. Values in between 0.0 and 1.0 create a partially transparent effect. Not all materials have alpha blending enabled. See the 
[Alpha Blending](alpha_blending) page for details.

## Col Map Naming Conventions
Col maps tend to follow certain naming conventions.
The texture name itself has no impact on how the texture is used.

| Col Texture Name | Usage |
| --- | --- |
| _b | Default Iris |
| _l | World of Light Enemy Iris (Red) |
| _g |Final Smash Iris (Yellow) |
| _w | Default Eye White |
| _d | World of LIght Enemy Dark Iris (Purple) |
| _wd | World of Light Enemy Dark Eye White |
| alp_ | Hair Color |
| def_ | Main Color |