---
---
# Texture Gamma Encoding
The shader programs used to draw the character models, stages, and even the UI elements perform almost all of their 
calculations in floating point. Floating point numbers can store numbers with a range of intensities at high precision such as 
0.5, 1.0003, etc. 

Image textures, on the other hand, typically use 8 bit unsigned integers for each color channel (similar to HTML/Hex colors) to save space compared to the 32 bits required to store a 
floating point number. The integer values in the texture 
are converted to floating point values based on the texture's format.

## Recommended Formats 
The texture format must match the type of data stored in the texture for the texture to render correctly in game.
<table class="table table-striped">
    <thead>
        <tr>
            <th scope="col" class="w-25">Texture</th>
            <th scope="col">Format</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td>Col (Texture0,Texture1)</td>
            <td>sRGB</td>
        </tr>
        <tr>
            <td>NOR (Texture4)</td>
            <td>Unorm</td>
        </tr>
        <tr>
            <td>PRM (Texture6)</td>
            <td>Unorm</td>
        </tr>
        <tr>
            <td>Diffuse Cube Map (Texture8)</td>
            <td>sRGB</td>
        </tr>
        <tr>
            <td>Diffuse Map (Texture10,Texture11,Texture12)</td>
            <td>sRGB</td>
        </tr>
        <tr>
            <td>EMI (Texture5/Texture14)</td>
            <td>sRGB</td>
        </tr>
        <tr>
            <td>Chara UI</td>
            <td>sRGB<sup><a href="#fn1" id="ref1">1</a></sup></td>
        </tr>
    </tbody>
</table>
<sup id="fn1">1. The in game chara UI files use a linear format and save the texture with manually darkened pixel values.</sup>

### Manual Gamma Correction Hacks
<figure class="figure">
    <img src="{{ "/assets/images/gamma/palu_comparison.png" | relative_url }}" height="auto" width="auto">
    <figcaption class="figure-caption text-center">The final rendered color for naive gamma fixing in an image editor (left) compared to saving the image as sRGB (right). 
        Note the artifacts in the shadows on the left image.</figcaption>
</figure>
The in game UI texture files use the unorm (linear) format and store already gamma corrected pixel intensities. This results in noticeable visual quality loss compared to using an sRGB format. The blocky regions of identical pixel values are more easily compressed, so the final compressed file size is smaller. There is no change in the image file size before compression. UI textures should be saved using SRGB unless the compressed size needs to match the original file's compressed size. 

Avoid trying to "fix" the texture gamma manually in an image editor. The inverse sRGB adjustment applied before the final color is displayed on screen is not the same as 
a gamma adjustment of (1.0 / 2.2), so the final color will not be correct if the texture was darkened using a gamma of 2.2 and saved as unorm. 
The result after applying a gamma or levels adjustment is still only stored using 8 bits per color channel, which produces noticeable banding artifacts in game such as in the above image. The artifacts caused by manually gamma correcting textures but saving as a linear format is most noticeable in darker tones on compressed textures.
Saving the texture using an sRGB format performs the correct gamma conversion using floating point and won't introduce any noticeable quality loss.  

## Texture Formats 
The texture's format tells the game how to interpret the texture's data for use in the shaders. The format should match the type of data stored in the texture and it's intended usage. 
Changing the format modifier at the end of the format name such as "BC7_SRGB" vs "BC7_UNORM" just indicates how to convert the texture values to float and does not change the texture data or file size.
The process of converting the integer values to floating point is called *normalization*. 
See the OpenGL wiki's <a href="https://www.khronos.org/opengl/wiki/Normalized_Integer" target="_blank">normalized integer page</a> for technical details. 

Textures need to be saved with the correct format to appear correctly in game. Saving with the wrong format will result in the texture appearing 
too bright or too dark in game. The full format depends on the compression type being used and if the texture has alpha. 

### sRGB
<div class="row">
    <div class="col-md-5 d-flex align-items-center justify-content-center">
        <img class="img-fluid" src="{{ "/assets/images/gamma/srgb_srgb.jpg" | relative_url }}">
    </div>
    <div class="col-md-7 d-flex align-items-center justify-content-center">
        <img class="img-fluid" src="{{ "/assets/images/gamma/srgb_to_float.png" | relative_url }}">
    </div>
</div>
Textures with sRGB formats store nonlinear color data. The texture values are converted to linear floating point values in the range 0.0 to 1.0 using the 
<a href="https://en.wikipedia.org/wiki/SRGB#The_sRGB_transfer_function_(%22gamma%22)" target="_blank">sRGB transfer function</a>. 
The conversion from the sRGB texture values directly to floating point doesn't introduce any of the artifacts caused by applying the adjustment manually in an image editor.

Smash Ultimate uses the sRGB format for storing the final color to be displayed on screen, 
so textures storing color data should also use a format with "sRGB" as part of the format name.

### Unorm (Unsigned Normalized / Linear / Linear Unsigned)
<div class="row">
    <div class="col-md-5 d-flex align-items-center justify-content-center">
        <img class="img-fluid" src="{{ "/assets/images/gamma/unorm_srgb.jpg" | relative_url }}">
    </div>
    <div class="col-md-7 d-flex align-items-center justify-content-center">
        <img class="img-fluid" src="{{ "/assets/images/gamma/unorm_to_float.png" | relative_url }}">
    </div>
</div>
Textures with unorm formats store linear data and are converted to floating point by simply dividing by the type's max value. 
8 bit values are divided by 255, 16 bit values are divided by 16355, etc. This converts unsigned integer values to floating point values in the range 0.0 to 1.0. 
Textures that don't store color data, such as NOR maps and PRM maps, must be saved as unorm to render correctly in game.

### Snorm (Signed Normalized / Linear Signed)
<div class="row">
    <div class="col-md-5 d-flex align-items-center justify-content-center">
        <img class="img-fluid" src="{{ "/assets/images/gamma/snorm_srgb.jpg" | relative_url }}">
    </div>
    <div class="col-md-7 d-flex align-items-center justify-content-center">
        <img class="img-fluid" src="{{ "/assets/images/gamma/snorm_to_float.png" | relative_url }}">
    </div>
</div>
Textures with snorm formats are converted to floating point values in the range -1.0 to 1.0. These formats aren't as common as unorm or sRGB. 


