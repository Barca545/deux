use std::fs;

use crate::{
  core::{
    buffer::{IndexBuffer, VertexBuffer},
    texture::Texture,
    vertex::ModelVertex,
  },
  errors::RendererErrors,
  renderer::Renderer,
  scene::{material::Material, mesh::Mesh, model::Model},
};
use eyre::Result;
use image::io::Reader;
use tobj::LoadOptions;
use wgpu::{
  BindGroupDescriptor, BindGroupEntry, BindingResource, Device, Queue, ShaderModule,
  ShaderModuleDescriptor, ShaderSource,
};

///Load a [`Model`] and its [`Texture`]s.
pub fn load_model(renderer:&mut Renderer, name:&str,) -> Model {
  let path = format!("assets/models/{name}.obj");
  let load_options = &LoadOptions {
    single_index:true,
    triangulate:true,
    ..Default::default()
  };

  let (models, obj_materials,) = tobj::load_obj(path, load_options,).unwrap();

  //Create the materials
  let mut materials = Vec::new();
  for material in obj_materials.unwrap() {
    let diffuse_texture = match &material.diffuse_texture {
      Some(texture,) => load_texture(texture, &renderer.ctx.device, &renderer.ctx.queue,).unwrap(),
      // TODO: Eventually this needs to actually import the correct texture based on params
      None => load_texture("red.jpg", &renderer.ctx.device, &renderer.ctx.queue,).unwrap(),
    };

    // Create the Texture and Sampler bindgroup and cache it
    let bind_group =
      renderer
        .render_resources
        .insert_bindgroup(renderer.ctx.device.create_bind_group(&BindGroupDescriptor {
          label:Some(diffuse_texture.label.as_str(),),
          layout:&renderer.texture_bind_group_layout(),
          entries:&[
            BindGroupEntry {
              binding:0,
              resource:BindingResource::TextureView(&diffuse_texture.view,),
            },
            BindGroupEntry {
              binding:1,
              resource:BindingResource::Sampler(&diffuse_texture.sampler,),
            },
          ],
        },),);

    //Create and add the material
    let material = Material::new(name, diffuse_texture, bind_group,);
    materials.push(material,);
  }

  //Iterate over the model's meshes to generate a Mesh
  let meshes = models
    .into_iter()
    .map(|model| {
      //Create the model's verticies by iterating over the mesh's indices
      let vertices = (0..model.mesh.positions.len() / 3)
        .map(|i| {
          let position_offset = (i * 3) as usize;
          let texture_offset = (i * 2) as usize;

          //Calculate the position coords
          let position = [
            model.mesh.positions[position_offset],
            model.mesh.positions[position_offset + 1],
            model.mesh.positions[position_offset + 2],
          ];

          //Get the texture coords
          let texture = [
            model.mesh.texcoords[texture_offset],
            1.0 - model.mesh.texcoords[texture_offset + 1],
          ];

          //Create the vertex
          ModelVertex::new(position, texture,)
        },)
        .collect::<Vec<_,>>();

      //Create the vertex and index buffers
      let vertex_buffer = VertexBuffer::new(&renderer.ctx.device, &vertices,);
      let index_buffer = IndexBuffer::new(&renderer.ctx.device, &model.mesh.indices,);

      Mesh::new(
        name,
        vertex_buffer,
        index_buffer,
        model.mesh.material_id.unwrap_or(0,),
      )
    },)
    .collect::<Vec<_,>>();
  Model::new(meshes, materials,)
}

///Load a [`Texture`].
fn load_texture(name:&str, device:&Device, queue:&Queue,) -> Result<Texture,> {
  let path = format!("assets/textures/{name}");

  match Reader::open(&path,) {
    Ok(img,) => match img.decode() {
      Ok(img,) => Ok(Texture::from_image(device, queue, img, name,),),
      Err(err,) => return Err(RendererErrors::FailedToDecodeImage(err,).into(),),
    },
    Err(_,) => {
      return Err(
        RendererErrors::FailedToLoadImage {
          name:name.to_string(),
          path,
        }
        .into(),
      )
    }
  }
}

// TODO: Load the shader function here too
pub fn load_shader(device:&Device, name:&str,) -> Result<ShaderModule,> {
  let path = format!("assets/shaders/{name}.wgsl");

  match fs::read_to_string(&path,) {
    Ok(shader,) => Ok(device.create_shader_module(ShaderModuleDescriptor {
      label:Some("Opaque Shader",),
      source:ShaderSource::Wgsl(shader.into(),),
    },),),
    Err(err,) => {
      return Err(
        RendererErrors::ShaderDoesNotExist {
          name:name.to_string(),
          path,
          err,
        }
        .into(),
      )
    }
  }
}
