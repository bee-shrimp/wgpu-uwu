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

what i did:  

- make Rect struct in main.rs to hold position and speed of the square.  
- make Uniforms struct in renderer.rs to hold 4x4 matrix.  
- use glam to manipulate the matrix.  
- use bytemuck to pass the matrix to shader.  

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

what i did:  

- add uv field to Vertex struct(delete colour field).  
- add uv to vertices. top left = (0.0, 0.0), bottom right = (1.0, 1.0) etc.  
- add uv field to vertex shader output. it passes uv as is.  
- fragment shader calculates colour using uv.  

what i learnt:  

- how to pass information between shaders.  
- what uv is and how to use it.  

## nombreux

what it does:  

- based on coleur.  
- draw many rects.  
- rects moves in circle.  

what i did:  

- make instance buffer to hold the xy of instances.  
- move rects using sin() and cos().  

what i learnt:  

- how to make/use instance buffer.  

## tourner

what it does:  

- based on nombreux.  
- rotate and scale the rects.  


