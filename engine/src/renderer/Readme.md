# Overview

Rendering infrastructure for the engine.

- Core: Contains the building blocks of creating a renderer and rendering information to the screen.
- Scene: Contains the information needed to draw a 3D in-game scene.
- UI: Contains the components needed to integrate [yakui](https://docs.rs/yakui/latest/yakui/) into the rest of the renderer.

# Module Layout

```
View
├─────Core
│      ├──GraphicsContext
│      ├──Vertex
│      ├──Buffer
│      ├──Texture
│      ├──RenderPass
│      └──DrawCommand
├──────Scene
│       ├──Camera
│       ├──Transforms
│       ├──Instance
│       ├──Model
│       └──Material
├──────Ui
│      ├──UiVertex
│      ├──UiTexture
│      └──UiRenderPass
└──────Renderer
```
