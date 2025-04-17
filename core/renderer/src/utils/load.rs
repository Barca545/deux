use crate::{
  core::{
    buffer::{IndexBuffer, VertexBuffer},
    gpu_context::GpuContext,
    texture::Texture,
    vertex::ModelVertex,
  },
  errors::RendererErrors,
  renderer::Renderer,
  scene::{material::Material, mesh::Mesh, model::Model},
};
use eyre::Result;
use image::io::Reader;
use std::{fs, path::PathBuf};
use tobj::LoadOptions;
use wgpu::{ShaderModule, ShaderModuleDescriptor, ShaderSource};

/// Load a [`Model`](crate::scene::model::Model) and its
/// [`Texture`](crate::core::texture::Texture)s.into the scene and store it in
/// [`RenderResources`](crate::utils::resources::RenderResources).
pub fn load_model(renderer: &mut Renderer, name: &str,) -> Model {
  let path = PathBuf::from(format!("assets/models/{name}.obj"),);

  let load_options = &LoadOptions {
    single_index: true,
    triangulate: true,
    ..Default::default()
  };

  let (models, obj_materials,) = tobj::load_obj(path, load_options,).unwrap();

  //Create the materials
  let mut materials = Vec::new();
  for material in obj_materials.unwrap() {
    let diffuse_texture = match &material.diffuse_texture {
      Some(texture,) => load_texture(&renderer.ctx, texture,).unwrap(),
      // TODO: Eventually this needs to actually import the correct texture based on params
      None => load_texture(&renderer.ctx, "red.jpg",).unwrap(),
    };

    // Create the Texture bindgroup and cache it
    let texture_bindgroup = renderer
      .resources
      .insert_bindgroup(renderer.create_texture_bindgroup(&diffuse_texture,),);

    // Cache the material and return the key
    let material =
      renderer
        .resources
        .insert_material(Material::new(name, diffuse_texture, texture_bindgroup,),);
    materials.push(material,);
  }

  // Iterate over the model's meshes and a Mesh for each one
  let meshes = models
    .into_iter()
    .map(|model| {
      //Create the model's verticies by iterating over the mesh's indices
      let vertices = (0..model.mesh.positions.len() / 3)
        .map(|i| {
          let position_offset = (i * 3) as usize;
          let texture_offset = (i * 2) as usize;

          // Calculate the position coords
          let position = [
            model.mesh.positions[position_offset],
            model.mesh.positions[position_offset + 1],
            model.mesh.positions[position_offset + 2],
          ];

          // Get the texture coords
          let texture = [
            model.mesh.texcoords[texture_offset],
            1.0 - model.mesh.texcoords[texture_offset + 1],
          ];

          // Create the vertex
          ModelVertex::new(position, texture,)
        },)
        .collect::<Vec<_,>>();

      // Create the vertex and index buffers
      let vertex_buffer = VertexBuffer::new(&renderer.ctx.device, &vertices,);
      let index_buffer = IndexBuffer::new(&renderer.ctx.device, &model.mesh.indices,);

      Mesh::new(
        name,
        vertex_buffer,
        index_buffer,
        materials[model.mesh.material_id.unwrap()],
      )
    },)
    .collect::<Vec<_,>>();
  Model::new(meshes, materials,)
}

/// Load a [`Texture`].
fn load_texture(ctx: &GpuContext, name: &str,) -> Result<Texture,> {
  let path = format!("assets/textures/{name}");

  match Reader::open(&path,) {
    Ok(img,) => match img.decode() {
      Ok(img,) => Ok(Texture::from_image(&ctx, img, name,),),
      Err(err,) => return Err(RendererErrors::FailedToDecodeImage(err,).into(),),
    },
    Err(_,) => {
      return Err(
        RendererErrors::FailedToLoadImage {
          name: name.to_string(),
          path,
        }
        .into(),
      )
    }
  }
}

// TODO: Load the shader function here too
pub fn load_shader(ctx: &GpuContext, name: &str,) -> Result<ShaderModule,> {
  let path = format!("assets/shaders/{name}.wgsl");

  match fs::read_to_string(&path,) {
    Ok(shader,) => Ok(ctx.device.create_shader_module(ShaderModuleDescriptor {
      label: Some("Opaque Shader",),
      source: ShaderSource::Wgsl(shader.into(),),
    },),),
    Err(err,) => {
      return Err(
        RendererErrors::ShaderDoesNotExist {
          name: name.to_string(),
          path: path.to_string(),
          err,
        }
        .into(),
      )
    }
  }
}
