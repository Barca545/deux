// use std::collections::HashMap;

// use crate::view::render_gl::{Shader, ShaderProgram, Texture};
// use eyre::Result;
// use gl::{types::GLenum, Gl, FRAGMENT_SHADER, GEOMETRY_SHADER, VERTEX_SHADER};

// // Refactor:
// // -Add support for loading material files, the XML things
// // -Figure out how to avoid wasted passes where multiple materials use some of the same shader
// // -For some reason some examples seem to have multiple techniques?
// // -Instead of storing the texture in the Sampler, store it in a texture atlas?
// // -Check footer
// // -Add different maps to the sampler as needed, right now I have none so we're good

// #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
// pub enum RenderStageName {
//   Fog = 0,
//   Shadows = 1,
//   StaticMesh = 2,
//   SkinnedMesh = 3,
//   Outlines = 4,
//   Plants = 5,
//   Water = 6,
//   Decals = 7,
//   SpecialOutlines = 8,
//   Particles = 9,
//   Billboards = 10,
//   UI = 11,
// }

// #[derive(Debug, Clone)]
// pub struct Material {
//   pub stages: HashMap<RenderStageName, RenderStage>,
//   pub samplers: Vec<Sampler>,
// }

// impl Material {
//   pub fn new() -> Self {
//     Material {
//       stages: HashMap::new(),
//       samplers: Vec::new(),
//     }
//   }

//   ///Adds a new [`RenderStage`] to the material.
//   pub fn add_stage(&mut self, stage: RenderStage) -> &mut Self {
//     self.stages.insert(stage.name, stage);
//     self
//   }

//   pub fn add_sampler(&mut self, gl: &Gl, name: &str) -> Result<&mut Self> {
//     //Load the texture
//     let texture = Texture::new(gl, name)?;
//     let name = name.to_string();

//     //Create the Sampler
//     Sampler { name, texture };
//     Ok(self)
//   }
// }

// #[derive(Debug, Clone)]
// ///Struct containing a [`Texture`] and any information needed to render it.
// /// Abstraction for a [GLSL sampler](https://www.khronos.org/opengl/wiki/Sampler_(GLSL)).
// pub struct Sampler {
//   pub name: String,
//   pub texture: Texture,
// }

// impl Sampler {
//   ///Create a new [`Sampler`].
//   pub fn new(gl: &Gl, name: &str) -> Result<Self> {
//     let texture = Texture::new(gl, name)?;
//     let name = name.to_string();
//     Ok(Sampler { name, texture })
//   }
// }

// #[derive(Debug, Clone)]
// pub struct RenderStage {
//   name: RenderStageName,
//   pub(crate) passes: Vec<RenderPass>,
// }

// impl RenderStage {
//   ///Create a new [`RenderStage`].
//   pub fn new(name: RenderStageName) -> RenderStage {
//     RenderStage { name, passes: Vec::new() }
//   }

//   ///Add a [`RenderPass`] to the [`RenderStage`].
//   pub fn add_pass(&mut self, pass: RenderPass) -> &mut Self {
//     self.passes.push(pass);
//     self
//   }
// }

// #[derive(Debug, Clone)]
// pub struct RenderPass {
//   shader_program: ShaderProgram,
//   ///Array of gl render capabilities, i.e. DEPTH_TEST, CULL_FACE
//   render_state: Vec<GLenum>,
// }

// impl RenderPass {
//   ///Creates a new [`RenderPassBuilder`].
//   /// Add [`Shader`]s using the `with_vert`, `with_frag`, and `with_geom` methods.
//   /// Enable render state capabilities with the `enable` method.
//   /// Initialize the [`RenderPass`] using the `build` method.
//   pub fn new(gl: &Gl) -> RenderPassBuilder {
//     RenderPassBuilder::new(gl)
//   }

//   ///Method to prepare a [`RenderPass`] to render.
//   /// Installs the [`ShaderProgram`] and enables the capabilities.
//   pub fn init(&self, gl: &Gl) {
//     unsafe {
//       //Install the shader program
//       self.shader_program.use_program(gl);

//       //Set the uniforms' values

//       //Enable the capabilities
//       for capability in &self.render_state {
//         gl.Enable(*capability);
//       }
//     }
//   }
// }

// pub struct RenderPassBuilder<'b> {
//   gl: &'b Gl,
//   shaders: Vec<Shader>,
//   render_state: Vec<GLenum>,
// }

// impl<'b> RenderPassBuilder<'b> {
//   fn new(gl: &'b Gl) -> Self {
//     RenderPassBuilder {
//       gl,
//       shaders: Vec::new(),
//       render_state: Vec::new(),
//     }
//   }

//   ///Add a vertex [`Shader`] to the [`RenderPass`]'s [`ShaderProgram`].
//   pub fn with_vert(&mut self, name: &str) -> Result<&mut Self> {
//     let vert = name.to_owned() + ".vert";
//     let vert = Shader::new(self.gl, &vert, VERTEX_SHADER)?;
//     self.shaders.push(vert);
//     Ok(self)
//   }

//   ///Add a fragment [`Shader`] to the [`RenderPass`]'s [`ShaderProgram`].
//   pub fn with_frag(&mut self, name: &str) -> Result<&mut Self> {
//     let frag = name.to_owned() + ".frag";
//     let frag = Shader::new(self.gl, &frag, FRAGMENT_SHADER)?;
//     self.shaders.push(frag);
//     Ok(self)
//   }

//   ///Add a geometry [`Shader`] to the [`RenderPass`]'s [`ShaderProgram`].
//   pub fn with_geom(&mut self, name: &str) -> Result<&mut Self> {
//     let geom = name.to_owned() + ".geom";
//     let geom = Shader::new(self.gl, &geom, GEOMETRY_SHADER)?;
//     self.shaders.push(geom);
//     Ok(self)
//   }

//   ///Add [`GLenum`]s which characterize the pass's capabilities.
//   pub fn enable(&mut self, capabilities: &[GLenum]) -> &mut Self {
//     self.render_state = Vec::from(capabilities);
//     self
//   }

//   ///Create the [`RenderPass`].
//   pub fn build(&self) -> Result<RenderPass> {
//     let program = ShaderProgram::new(self.gl, self.shaders.clone())?.build()?;

//     Ok(RenderPass {
//       shader_program: program,
//       render_state: self.render_state.clone(),
//     })
//   }
// }

// // So Submeshes have a Material type

// // When a mesh wants to render, it gets sent to some drawbuffer, sorted by material type
// // To render a material iterate over its passes

// // need to solve the problem with setting shader uniforms

// // pub struct Material {
// //   //Contains information about the texture to use
// //   //Information about the shader to use
// //   //Information about which pass it belongs to
// // }

// //Render stage
// // -Render stages dictate the shader program and render state
// //    -i.e. is Depth Testing/Stencil Testing enabled)
// // -Shader program for a stage takes in the possible parameters as a material
// //    -i.e. A shader in the skinned meshes stage might allow a mesh to be
// //     with anything from a normal texture to just as a color + reflection
// // -Meshes can be sent to multiple render stage
// //    -i.e. A skinned mesh might go to the shadow, skinned mesh, and outline stages

// //This is how riot handled it but it is outdated:
// // Render phases
// // shadows
// // static meshes
// // skinned meshes
// // outlines
// // plants
// // water
// // decals
// // particles
// // anti aliasing
// // billboards
// // HUD
