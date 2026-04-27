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
- draw a square.  
- keyboard inputs controll the square.  
