# wgpu-uwu  

my delusional attempt to learn wgpu.  
the focus is pixel art rendering and post processing.  
inspired by [parasyte/pixels](https://github.com/parasyte/pixels).  

## 00. blue  

what it is:  

- [hello-window](https://github.com/gfx-rs/wgpu/tree/trunk/examples/standalone/02_hello_window) example from the official wgpu repo lightly modified.  

what i did:  

- replaced an outdated method.  
  - `queue.present(surface_texture)` -> `surface_texture.present()`  
- changed clear colour from green to blue.  
- took notes in comments to understand.  

## 01_triangle  

what it does:  

- clear the window with black.  
- draw a big triangle.  
- the colour of the triangle slowly changes between pink and blue.  

what i did:  

- integrated the shader and the render pipeline from [hello-triangle](https://github.com/gfx-rs/wgpu/tree/trunk/examples/features/src/hello_triangle).  
- integrated uniform buffer from [uniform-buffer example](https://github.com/gfx-rs/wgpu/tree/trunk/examples/features/src/uniform_values).  
- separated State as a module and renamed it Renderer.  
- took more notes.  

what i learnt:  

- how to make/update/use a uniform buffer.  

## 02_square  

what it does:  

- clear the window with black.  
- draw a small white square.  
- keyboard inputs control the square.  

how it works:  

- Rect struct in main.rs has position and speed of the square.  
- Uniforms struct in renderer.rs has 4x4 matrix.  
- glam provides tools to manipulate the matrix (`Mat4::from_translation()`).  
- bytemuck converts matrix to suitable form for shader.  

what i learnt:  

- how to pass information between main/renderer/shader.  
- how to handle input without WinitInputHelper.  

reference:  

- [Uniform buffers and a 3d camera | Learn Wgpu](https://sotrh.github.io/learn-wgpu/beginner/tutorial6-uniforms/)  
- [Proper way to get input in game with winit :r/rust_gamedev](https://www.reddit.com/r/rust_gamedev/comments/1b1oxtr/proper_way_to_get_input_in_game_with_winit/?solution=31b6dcf698621a6b31b6dcf698621a6b&js_challenge=1&token=bbbe4bf1c9a2b5160829c4be34da5861d3917b178f888e044fcbc2dfcd4e7f69&jsc_orig_r=)  

## 03_colour  

what it does:  

- based on 02_square.  
- the colour of the square gradates according to uv coordinate.  

how it works:  

- Vertex struct has uv field instead of colour field.  
- each vertex has a uv coordinate. top left = (0.0, 0.0), bottom right = (1.0, 1.0) etc.  
- VertexOutput has uv field. vertex shader just passes uv as is.  
- fragment shader calculates colour using uv.  

what i learnt:  

- how to pass information between shaders.  
- what uv is and how to use it.  

## 04_many  

what it does:  

- based on 03_colour.  
- draw many rects.  
- rects move in a circle.  

how it works:  

- instance buffer has the xy of instances.  
- rect.update() calculates new pos using sin() and cos().  
- renderer.update() applies `Mat4::from_translation()` using the new pos.  

what i learnt:  

- how to make/use instance buffer.  

## 05_rotate  

what it does:  

- based on 04_many.  
- draw one big rect.  
- rotate and scale the rect.  

how it works:  

- renderer.update() applies scale and rotation to matrix.  
- uniform buffer has the matrix.  
- vertex shader applies the matrix to the rect.  

what i learnt:  

- how to apply multiple things to a 4x4 matrix.  
- the order matters(scale -> rotation -> translation).  
- the order flips. `(model *= translation * rotation * scale)`  
- (applied from right to left)  

## 06_image  

what it does:  

- based on 05_rotate.  
- draw an image instead of a rect.  

how it works:  

- image crate converts the image to vec data.  
- renderer has diffuse_texture (image data written) and sampler.  
- diffuse_bind_group passes texture and sampler to fragment shader.  

what i learnt:  

- how to handle image files with image crate.  
- how to handle multiple bind groups.  
- what texture and sampler are and how they work.  

## 07_big  

what it does:  

- draw a pixel art image scaled up.  

how it works:  

- sampler uses Nearest instead of Linear to show a crisp pixel art.  

what i learnt:  

- filter type differences.  

## 08_grey  

what it does:  

- draw an image in grayscale.  

how it works:  

- fragment shader changes rgb to the avg of rgb.  

what i learnt:  

- how to manipulate pixel colour in shader.  

## 09_blur  

what it does:  

- draw an image with gaussian blur.  

how it works:  

- fragment shader calculates blur using colours of nearby pixels.  

what i learnt:  

- the basics of gaussian blur.  

## 10_mid  

what it does:  

- clear the window with black.  
- draw a white rectangle inside a blue rectangle.  
- the white rect moves with keyboard inputs (just like 02_square).  
- resizing the window does not break the aspect ratio of the blue rect.  

how it works:  

- has 2 renderpasses.  
  - mid (smaller texture)  
    - create a smaller mid texture.  
    - clear the mid texture with blue.  
    - draw a white rectangle to the mid texture.  
  - scaler (surface)  
    - clear with black.  
    - sample and draw the mid texture.  
    - viewport is set to keep the aspect ratio intact.  

what i learnt:  

- how to make and use multiple bind groups/pipelines/renderpasses.  
- how to keep the aspect ratio intact when resizing.  

## 11_scaler  

what it does:  

- clear the window with blue.  
- draw a pixel art image on a black rectangle.  
- the pixel art moves with keyboard inputs (just like 10_mid).  

how it works:  

- load a png file using image.  
- create a diffuse texture to draw the image onto.  
- use queue.write_texture to draw the image on diffuse_texture.  
- mid renderpass: sample diffuse_texture and draw onto mid.  
- scaler renderpass: sample mid_texture and draw onto the surface.  

what i learnt:  

- how to make and use even more bind groups/pipelines/renderpasses.  

## 12_layer  

what it does:  

- clear the window with black.  
- draw a purplish gradient triangle.  
- draw a small half translucent white square.  
- small square moves with keyboard inputs.  

how it works:  

- has 4 textures and 4 pipelines.  
  - base: clear with black and draw a triangle (alpha=1.0).  
  - mid: clear with transparent and draw a white square (alpha=0.5).  
  - blend: sample base and mid texture, blend them with alpha considered.  
  - scaler: sample blend texture and make it bigger.  

what i learnt:  

- how to chain multiple renderpasses.  

## 13_illuminate  

what it does:  

- clear the window with black.  
- draw an image.  
- the brightness of the image changes with keyboard inputs.  

how it works:  

- App has brightness field (set 0.0).  
- keyboard inputs (arrow up/down) add/sub 0.01 from the brightness.  
- uniform buffer contains the brightness.  
- shader uses the content of uniform buffer as brightness offset.  

what i learnt:  

- idea of post processing.  

(also updated keyboard input handling and renderer.resize())  

## 14_hue  

what it does:  

- clear the window with black.  
- draw an image.  
- the hue of the image changes over time.  

how it works:  

- App has time field (starts 0.0).  
- about_to_wait increases time by 0.002.  
- uniform_buffer contains the time.  
- has 3 pipelines.  
  - mid  
    - draw an image.  
  - scaler  
    - sample mid texture and draw bigger.
    - (not necessary, effect pipeline can do this)  
    - (too lazy to fix)
  - effect  
    - sample scaler texture.  
    - convert rgb of the texture colour to hsv.  
    - rotate hue a little using time from uniform buffer.  
    - convert hsv back to rgb.  
    - draw the rotated colour.  

## 15_glitch  

what it does:  

- draw a fish with watery background.  
- with keyboard input (arrow up) fish glitches.  
- keyboard inputs (arrow up/down) control the intensity of the glitch.  

how it works:  

- has 4 pipelines.  
  - base
    - draw background.  
  - mid  
    - draw fish.  
  - effect  
    - sample mid texture.  
    - make a pseudo random number using uv.y.  
    - the whole line glitches when the rand is higher than a threshold.  
    - shift uv.x and its colour.  
  - scaler  
    - sample and blend base/mid.  
    - draw bigger on the surface.  

what i learnt:  

- how to manipulate pixel placement in shader.  
- how to generate pseudo random number in shader.  

## 16_water  

what it does:  

- draw a lakeside image.  
- the lake has a wavy reflection of the lakeside view.  

how it works:  

- base  
  - draw the image as is.  
- effect  
  - map uv.y 0.0-0.5 (upper half) of base to uv.y 0.0-1.0 (entire texture).  
  - add different sin waves to uv with time and coord.  
- scaler  
  - sample base with nearest sampler.  
  - sample effect with linear sampler.  
  - blend them.  

what i learnt:  

- how difficult water shader is.  

## 17_illuminate  

what it does:  

- draw an image (colourful rectangles on black).  
- the rects has bloom effect.  

how it works:  

- extract bright parts of the image.  
- make the bright image blur by redrawing on smaller and smaller textures.  
- blend (additive) smaller textures to bigger textures to make it glow.  

what i learnt:  

- how to handle tons of textures/bind groups/renderpasses.  
- making helper functions is convenient.  

## 18. wallpaper  

what it does:  

- draw an image of sky and sea.  
- the cloud of the sky glows, the sea has a wavy reflection of clouds.  
- the wave intensifies with keyboard inputs (arrow up/down).  

how it works:  

- has 16_water and 17_illuminate functions combined.  

what i learnt:  

- it's my wallpaper now.  
