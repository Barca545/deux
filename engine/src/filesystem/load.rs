use super::champion::Champion;
use crate::{
  arena::Grid,
  component_lib::AbilityMap,
  errors::FilesystemErrors,
  view::{IndexBuffer, Material, Mesh, Model, ModelVertex, Renderer, Texture, VertexBuffer},
};
use eyre::Result;
use image::io::Reader;
use std::fs;
use tobj::LoadOptions;
use wgpu::{BindGroupDescriptor, BindGroupEntry, BindingResource, Device, Queue, ShaderModule, ShaderModuleDescriptor, ShaderSource};

//Refactor
// -Pull useful portions from the level editor
// -Move the path generation into its own function find/replace lowercase path
// -Loading in the grid might require flipping since I use Y as up but blender uses Z as up
// -Use lazy static to define the paths

///Load a [`Model`] and its [`Texture`]s.
pub fn load_model(name: &str, device: &Device, queue: &Queue) -> Model {
  let path = format!("C:/Users/jamar/Documents/Hobbies/Coding/deux/assets/models/{name}.obj");

  let load_options = &LoadOptions {
    single_index: true,
    triangulate: true,
    ..Default::default()
  };

  let (models, obj_materials) = tobj::load_obj(path, load_options).unwrap();

  //Create the materials
  let mut materials = Vec::new();
  for material in obj_materials.unwrap() {
    let diffuse_texture = match &material.diffuse_texture {
      Some(texture) => load_texture(texture, device, queue).unwrap(),
      None => load_texture("red.jpg", device, queue).unwrap(),
    };

    //Create the Texture and Sampler bindgroup
    let bind_group = device.create_bind_group(&BindGroupDescriptor {
      label: Some(diffuse_texture.label.as_str()),
      layout: &&Renderer::texture_bind_group_layout(&device),
      entries: &[
        BindGroupEntry {
          binding: 0,
          resource: BindingResource::TextureView(&diffuse_texture.view),
        },
        BindGroupEntry {
          binding: 1,
          resource: BindingResource::Sampler(&diffuse_texture.sampler),
        },
      ],
    });

    //Create and add the material
    let material = Material::new(name, diffuse_texture, bind_group);
    materials.push(material);
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
          let texture = [model.mesh.texcoords[texture_offset], 1.0 - model.mesh.texcoords[texture_offset + 1]];

          //Create the vertex
          ModelVertex::new(position, texture)
        })
        .collect::<Vec<_>>();

      //Create the vertex and index buffers
      let vertex_buffer = VertexBuffer::new(&device, &vertices);
      let index_buffer = IndexBuffer::new(&device, &model.mesh.indices);

      Mesh::new(name, vertex_buffer, index_buffer, model.mesh.material_id.unwrap_or(0))
    })
    .collect::<Vec<_>>();
  Model::new(meshes, materials)
}

///Load a [`Texture`].
fn load_texture(name: &str, device: &Device, queue: &Queue) -> Result<Texture> {
  let path = format!("C:/Users/jamar/Documents/Hobbies/Coding/deux/assets/textures/{name}");

  match Reader::open(&path) {
    Ok(img) => match img.decode() {
      Ok(img) => Ok(Texture::from_image(device, queue, img, name)),
      Err(err) => return Err(FilesystemErrors::FailedToDecodeImage(err).into()),
    },
    Err(_) => return Err(FilesystemErrors::FailedToLoadImage { name: name.to_string(), path }.into()),
  }
}

pub fn load_champion_json(name: &str) -> Result<Champion> {
  let path = format!("C:/Users/jamar/Documents/Hobbies/Coding/deux/assets/champions/{name}.json");

  let champion_string = match fs::read_to_string(&path) {
    Ok(str) => str,
    Err(err) => {
      return Err(
        FilesystemErrors::ChampDataDoesNotExist {
          name: name.to_string(),
          path,
          err,
        }
        .into(),
      )
    }
  };

  let champion = serde_json::from_str::<Champion>(&champion_string)?;
  Ok(champion)
}

///Load an entity's [`AbilityMap`].
pub fn load_scripts(name: &str) -> Result<AbilityMap> {
  todo!()
}

pub fn load_shader(device: &Device, name: &str) -> Result<ShaderModule> {
  let path = format!("C:/Users/jamar/Documents/Hobbies/Coding/deux/assets/shaders/{name}.wgsl");

  match fs::read_to_string(&path) {
    Ok(shader) => Ok(device.create_shader_module(ShaderModuleDescriptor {
      label: Some("model shader"),
      source: ShaderSource::Wgsl(shader.into()),
    })),
    Err(err) => {
      return Err(
        FilesystemErrors::ShaderDoesNotExist {
          name: name.to_string(),
          path,
          err,
        }
        .into(),
      )
    }
  }
}

///Loads the a [`Grid`]'s information.
pub fn load_grid(name: &str, extension: &str) -> Result<Grid> {
  // let path = var("grid_folder")? + "/" + name + "." + extension;
  // let grid_path = fs::read_to_string(path)?;
  // let grid: Grid = serde_json::from_str(&grid_path)?;
  // Ok(grid)
  todo!()
}
