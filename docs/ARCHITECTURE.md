> [!WARNING]
> Deux is still under heavy development. This documentation is **not** final.

> [!Note] 
> This document is a work in progress. Some or most sections are currently unfilled.

# Table of Contents

- [Core](#core)
  - [Storage](#storage)
  - [Events](#events)
  - [Inputs](#inputs)
  - [Math](#math)
  - [Renderer](#renderer)
  - [Time](#time)
  - [Game Data](#game-data)
  - [Utility](#utility)
- [App](#app)
  - [Gameplay](#gameplay)
  - [Lib](#lib)
  - [Main](#main)
- [Assets](#assets)

# Module Tree
Crates containing only a `lib.rs` are not expanded

```text
├─App
│  ├─Gameplay
│  ├─Lib
│  └─Main
├─Core
│  ├─Storage
│  │   ├─Cache
│  │   └─Memory Arena
│  ├─Event
│  ├─Math
│  │   └─Number 
│  ├─Renderer
│  │   ├─Core
│  │   │   ├─Buffer
│  │   │   ├─Color
│  │   │   ├─GPU Context
│  │   │   ├─Texture
│  │   │   ├─Instance
│  │   │   └─Vertex
│  │   ├─Scene
│  │   │   ├─Camera
│  │   │   ├─Material
│  │   │   ├─Mesh
│  │   │   └─Model
│  │   ├─RenderPass
│  │   ├─Render Resources
│  │   ├─Renderer
│  │   └─Errors
│  ├─Time
│  │   ├─Server Time
│  │   └─Timer
│  ├─Windowing
│  │   ├─Window
│  │   └─SDL2 Utils
│  ├─Input Handling
│  │   ├─Mouse Ray
│  │   ├─Player Inputs
│  │   └─Errors
│  └─Game Data
│      ├─Components
│      ├─Resources 
│      │    └─World Map
│      └─Utility
└─Assets
```

# Core
## Storage
Data structures used for storing information the engine needs. 

**TODO:** Discuss the memory Arena in more detail.

## Events

## Filesystem

## Math & Physics
### Math
Exposes wrappers around [glm-rs](https://docs.rs/glm/latest/glm/) functions and types. Designed so replacing the game's math library is easy if necessary.

### Physics
Provides basic functionality for calculating object collisions and raycasting. As the game this engine is designed for has no substantive physics interactions this module is deliberately lightweight.

## Renderer
The Renderer is one of the more complex crates in the engine. Deux's renderer is built on top of [wgpu](https://wgpu.rs/). It consists of a [`core`](https://github.com/Barca545/deux/tree/wgpu-switch/engine%2Fsrc%2Frenderer%2Fcore) module and a [`scene`](https://github.com/Barca545/deux/tree/wgpu-switch/engine%2Fsrc%2Frenderer%2Fscene) module.

### Core
`Core` contains the main functionality of the render. It holds the [`GpuContext`](https://github.com/Barca545/deux/blob/wgpu-switch/engine%2Fsrc%2Frenderer%2Fcore%2Fgpu_context.rs), the abstraction used for communicating with the GPU. Also contains various primatives; primarily [`Vertex`](https://github.com/Barca545/deux/blob/wgpu-switch/engine%2Fsrc%2Frenderer%2Fcore%2Fvertex.rs)es and [`Texture`](https://github.com/Barca545/deux/blob/wgpu-switch/engine%2Fsrc%2Frenderer%2Fcore%2Ftexture.rs)s.

### Scene
`Scene` controls how the game is rendered. The central module is the [`Camera`](https://github.com/Barca545/deux/blob/wgpu-switch/engine%2Fsrc%2Frenderer%2Fscene%2Fcamera.rs) which provides the logic to control how vertices in a scene are manipulated. The other modules in `scene` control rendering the meshes which make up the elements within the scene.

### Renderer
Both subsection are tied together by the [`Renderer`](https://github.com/Barca545/deux/blob/wgpu-switch/engine%2Fsrc%2Frenderer%2Frenderer.rs) which handles building the `RenderPiplines` and dispatching `DrawCall`s to the GPU.

## Scripting

## Time
Struct driving the game's time. Uses Rust's [`Instant`](https://doc.rust-lang.org/std/time/struct.Instant.html) as a base for tracking the passage of time. All other timers in the game \[should\] reference the [`ServerTime`](https://github.com/Barca545/deux/blob/wgpu-switch/engine%2Fsrc%2Ftime%2Fserver_time.rs#L22) defined in this crate.

## Windowing
SDL2 helpers and window context management.

## Game Data

### Utility

## Inputs
Provides tools for processing and evaluating input events from SLD2's [`EventPump`](https://docs.rs/sdl2/latest/sdl2/struct.EventPump.html). Exports the [`KeyBinds`](https://github.com/Barca545/deux/blob/wgpu-switch/core%2Finput%2Fsrc%2Fkeybinds.rs#L9), [`FrameInputs`](https://github.com/Barca545/deux/blob/wgpu-switch/core%2Finput%2Fsrc%2Fframe_inputs.rs#L8), and [`MouseRay`](https://github.com/Barca545/deux/blob/wgpu-switch/core%2Finput%2Fsrc%2Fmouseray.rs#L4) structs.

## Macros
Procedural Macros for the engine.

# App 
## Gameplay 
The gameplay folder contains game-specific components and systems that define the behavior and rules of the game. While `core` provides generic engine functionality, `gameplay` builds on it to implement specific game features. Gameplay's main export is the `update` function which wraps all system functionality. 

## Main
Root of the binary. Very little logic lives here. See the [library module](#Lib) for ==more logic==.

## Lib

## ECS

### Components

### Resources

### Systems

## Assets
