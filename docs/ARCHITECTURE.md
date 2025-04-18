> [! Warning] Deux is still under heavy development. This documentation is **not** final.

> [! Note] This document is a work in progress. Some or most sections are currently unfilled.

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
  - [Lib](#lib)
  - [Main](#main)
- [Assets](#assets)

# Module Tree
Crates containing only a `lib.rs` are not expanded

```text
├─App
│    └─Loop
├─Core
│    ├─Storage
│    │   ├─Cache
│    │   └─Memory Arena
│    ├─Event
│    ├─Math
│    │   └─Number 
│    ├─Renderer
│    │   ├─Core
│    │   │   ├─Buffer
│    │   │   ├─Color
│    │   │   ├─GPU Context
│    │   │   ├─Texture
│    │   │   ├─Instance
│    │   │   └─Vertex
│    │   ├─Scene
│    │   │   ├─Camera
│    │   │   ├─Material
│    │   │   ├─Mesh
│    │   │   └─Model
│    │   ├─RenderPass
│    │   ├─Render Resources
│    │   ├─Renderer
│    │   └─Errors
│    ├─Time
│    │   ├─Server Time
│    │   └─Timer
│    ├─Windowing
│    │   ├─Window
│    │   └─SDL2 Utils
│    ├─Input Handling
│    │   ├─Mouse Ray
│    │   ├─Player Inputs
│    │   └─Errors
│    └─Game Data
│        ├─Components
│        ├─Resources 
│        │    └─World Map
│        └─Utility
└─Assets
```

# Core

## Storage

Data structure used for storing information the engine needs. ==**TODO:**== Discuss the Arena in more detail.

## Events

## Filesystem

## Math & Physics

### Math

Exposes wrappers around [glm-rs](https://docs.rs/glm/latest/glm/) functions and types. Designed so replacing the math library the game relies on is easy if necessary.

### Physics

Provides basic functionality for calculating object collisions and raycasting. As the game this engine is designed for has no substantive Physics interactions this module is deliberately lightweight.

## Renderer

The Renderer is one of the more complex crates in the engine. Deux's renderer is built on top of [wgpu](https://wgpu.rs/). It consists of a [`core`](https://github.com/Barca545/deux/tree/wgpu-switch/engine%2Fsrc%2Frenderer%2Fcore) module and a [`scene`](https://github.com/Barca545/deux/tree/wgpu-switch/engine%2Fsrc%2Frenderer%2Fscene) module.

### Core

`Core` contains the main functionality of the render. It holds the [`GpuContext`](https://github.com/Barca545/deux/blob/wgpu-switch/engine%2Fsrc%2Frenderer%2Fcore%2Fgpu_context.rs), the abstraction used for communicating with the GPU. As well as the ==APIs== for various primatives; primarily [Vertex](https://github.com/Barca545/deux/blob/wgpu-switch/engine%2Fsrc%2Frenderer%2Fcore%2Fvertex.rs)es and [`Texture`](https://github.com/Barca545/deux/blob/wgpu-switch/engine%2Fsrc%2Frenderer%2Fcore%2Ftexture.rs)s.

### Scene

==`Scene` controls how the game is rendered.== The central module is the [`camera`](https://github.com/Barca545/deux/blob/wgpu-switch/engine%2Fsrc%2Frenderer%2Fscene%2Fcamera.rs) which provides the logic to control how vertices in a scene are manipulated. The other modules in `scene` control rendering the meshes which make up the elements within the scene.

### Renderer

Both subsection are tied together by the [`renderer`](https://github.com/Barca545/deux/blob/wgpu-switch/engine%2Fsrc%2Frenderer%2Frenderer.rs) which handles building the ==renderpiplines== and dispatching ==DrawCommands== to the GPU.

## Scripting

## Time

Struct driving the game's time. Uses Rust's [`Instant`](https://doc.rust-lang.org/std/time/struct.Instant.html) as a base for tracking the passage of time. All other timers in the game \[should\] reference the [`ServerTime`](https://github.com/Barca545/deux/blob/wgpu-switch/engine%2Fsrc%2Ftime%2Fserver_time.rs#L22) defined in this crate.

## Windowing

==The Window is responsible for receiving inputs==

- SDL2 helpers go here needed for windowing
- Arguably inputs should too.

## Game Data

### ==Utility==

## Inputs

## Macros

**UNIMPLEMENTED** Procedural Macros for the engine.

# App J

<
Convert inputs into game events.

## Main

Root of the binary. Very little logic lives here. See the [library module](#Lib) for ==more logic==.

## Lib

## ==ECS==

### Components

### Resources

### Systems

## Assets
