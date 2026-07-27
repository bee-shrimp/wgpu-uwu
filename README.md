# wgpu-uwu

my delusional attempt to learn wgpu.  

## bleu

what it is:  

- examples/standalone/hello-window from official wgpu repo lightly modified.  

what i did:  

- replaced outdated method.  
  - queue.present(surface_texture) -> surface_texture.present()  
- changed clear colour from green to blue.  
- took notes in comments to understand.  

## triangle

what it does:  

- clear the window with black.  
- draw a big triangle.  
- the colour of the triangle slowly changes between pink and blue.  

what i did:  

- integrated the shader and render pipeline from hello-triangle.  
- integrated uniform buffer from uniform-buffer example.  
- separated State as a module and renamed it Renderer.  
- took more notes.  

what i learnt:  

- how to make/update/use an uniform buffer.  

## carre

what it does:  

- clear the window with black.  
- draw a small white square.  
- keyboard inputs controll the square.  

how it works:  

- Rect struct in main.rs holds position and speed of the square.  
- Uniforms struct in renderer.rs holds 4x4 matrix.  
- glam provides tools to manipulate the matrix(Mat4::from_translation()).  
- bytemuck converts matrix to suitable form for shader.  

what i learnt:  

- how to pass information between main/renderer/shader.  
- how to handle input without WinitInputHelper.  

reference:

- [Uniform buffers and a 3d camera | Learn Wgpu](https://sotrh.github.io/learn-wgpu/beginner/tutorial6-uniforms/)
- [Proper way to get input in game with winit :r/rust_gamedev](https://www.reddit.com/r/rust_gamedev/comments/1b1oxtr/proper_way_to_get_input_in_game_with_winit/?solution=31b6dcf698621a6b31b6dcf698621a6b&js_challenge=1&token=bbbe4bf1c9a2b5160829c4be34da5861d3917b178f888e044fcbc2dfcd4e7f69&jsc_orig_r=)

## coleur

what it does:  

- based on carre.  
- the colour gradates according to the uv coord.  

how it works:  

- Vertex struct has uv field instead of colour field.  
- each vertices has uv. top left = (0.0, 0.0), bottom right = (1.0, 1.0) etc.  
- vertex shader output has uv fields. vertex shader just passes uv as is.  
- fragment shader calculates colour using uv.  

what i learnt:  

- how to pass information between shaders.  
- what uv is and how to use it.  

## nombreux

what it does:  

- based on coleur.  
- draw many rects.  
- rects moves in circle.  

how it works:  

- instance buffer holds the xy of instances.  
- rect.update() calculates new pos using sin() and cos().  
- renderer.update() applies Mat4::from_translation() using the new pos.  

what i learnt:  

- how to make/use instance buffer.  

## tourner

what it does:  

- based on nombreux.  
- draw one big rect.  
- rotate and scale the rect.  

how it works:  

- renderer.update() applies scale and rotation to matrix.  
- uniform buffer holds the matrix.  
- vertex shader applies the matrix to the rect.  

what i learnt:  

- how to apply multiple things to a 4x4matrix.  
- the order matters(scale -> rotation -> translation).  
- the order flips(model \*= translation * rotation * scale)

## image

what it does:  

- based on tourner.  
- draw an image instead of a rect.  

how it works:  

- image crate converts the image to vec data.  
- rederer has texture(with image data) and sampler.  
- diffuse_bind_group passes the texture and sampler to fragment shader.  

what i learnt:  

- how to handle image files with image crate.  
- how to handle multiple bind groups.  
- what texture and sampler are and how they works.  

## grand

what it does:  

- draw a pixel art image scaled up.  

how it works:  

- sampler uses Nearest instead of Linear to show crisp pixel art.  

what i learnt:  

- filter type difference.  

(removed uniform buffer to make sure i understand how it works.)  

## gris

what it does:  

- draw an image in grayscale.  

how it works:  

- fragment shader changes rgb to the avg of rgb.  

what i learnt:  

- how to manipulate the pixels in texture.  

## flou

what it does:  

- draw an image with gaussian blur.  

how it works:  

- fragment shader calc blur using colours of nearby pixels.  

what i learnt:

- the basics of gaussian blur.  

## entre

what it does:  

- clear the window with black.  
- draw a white rectangle inside a blue rectangle.  
- the white rect moves with keaboard inputs(just like carre).  
- resizing the window does not break the aspect ratio of the blue rect.  

how it works:

- create a small mid texture.  
- clear the mid texture with blue.  
- draw a white rectangle to the mid textute.  
- sample and draw the mid texture to the surface.  
- viewport is set to keep the aspect ratio in tact.  

what i learnt:

- how to make and use multiple bind group/pipelime/renderpass.  
- how to keep the aspect ratio in tact.  

## grossir

what it does:  

- clear the window with bleu.  
- draw a pixel art on a black rectangle.  
- the pixel art moves with keaboard inputs(just like entre).  

how it works:

- load a png file using image.  
- create a diffuse texture to draw the image onto.  
- use queue.write_texture to draw the image on diffuse_texture.  
- create and use diffuse_bindgroup to sample diffuse_texture to mid.  

what i learnt:

- how to make and use even more bind group/pipeline/renderpass.  

## plusieurs

what it does:  

- clear the window with black.  
- draw a puplish gradiant triangle.  
- draw a small half translucent white square.  
- small rect moves with keyboard inputs.  

how it works:

- has 4 textures and 4 pipelimes.
        - base: clear with black and draw triangle.  
        - mid: clear with transparent and draw white square(alpha=0.5).  
        - blend: sample base and mid, blend with alpha considered.  
        - scaler: sample blend and make it bigger.  

what i learnt:

- how to chain multipul renderpasses.  

## lumiere

what it does:  

- clear the window with black.  
- draw an image.  
- the brightness of the image changes with keyboard inputs.  

how it works:

- App has brightness field(set 0.0).  
- keyboard inputs adds/subs 0.01 from the brightness.  
- uniform buffer contains the brightness.  
- shader uses the content of uniform buffer as brightness offset.  

what i learnt:

- idea of post proccessing.

(also updated keyboard input handling and resize fn)

## teinte


