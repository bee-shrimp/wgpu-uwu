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

- draw an image instead of a rect.  


