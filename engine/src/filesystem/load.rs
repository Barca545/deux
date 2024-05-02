use super::champion::Champion;
use crate::{
  arena::Grid,
  component_lib::AbilityMap,
  errors::FilesystemErrors,
  view::{
    render_gl::{
      buffer::{IndexBuffer, VertexBuffer},
      ModelVertex, Texture,
    },
    Material, Mesh, Model,
  },
};
use config::{Config, File as ConfigFile};
use eyre::Result;
use image::io::Reader;
use std::{
  collections::HashMap,
  env::{current_dir, var},
  ffi::CString,
  fs::{self, File},
  io::Read,
  path::Path,
};
use tobj::LoadOptions;
use wgpu::{BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindingResource, Device, Queue};

//Refactor
// -Use a config file to load each PC will need a unique one so do gitignore
// -Should the unwrap or else be some other form of unwrap
// -Pull useful portions from the level editor
// -have errors print the file that failed
// -Each game object will need it's own named asset folder
// -Move the path generation into its own function find/replace lowercase path
// -Make load image not panic
// -Loading in the grid might require flipping since I use Y as up but blender uses Z as up
// -Replace the load image with the stuff in renderer
// -Replace load shader with the stuff in renderer

///Load a [`Model`] and its [`Texture`]s.
pub fn load_model(name: &str, device: &Device, queue: &Queue, layout: &BindGroupLayout) -> Model {
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
    let diffuse_texture = load_texture(&material.diffuse_texture.unwrap(), device, queue).unwrap();

    //Create the Texture and Sampler bindgroup
    let bind_group = device.create_bind_group(&BindGroupDescriptor {
      label: Some(diffuse_texture.label.as_str()),
      layout,
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
pub fn load_texture(name: &str, device: &Device, queue: &Queue) -> Result<Texture> {
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
  let path = var("champion_folder")? + "/" + name + "." + "json";
  let champion_string = fs::read_to_string(path)?;

  let champion = serde_json::from_str::<Champion>(&champion_string)?;
  Ok(champion)
}

///Load an entity's [`AbilityMap`].
pub fn load_scripts(name: &str) -> Result<AbilityMap> {
  let path = var("champion_folder")? + name + "/scripts";

  // Ok(path)
  todo!()
}

pub fn load_shader(path: &str) -> Result<CString> {
  let path = var("shader_folder")? + "/" + path;
  let shader = load_cstring(&path)?;
  Ok(shader)
}

#[deprecated(since = "0.0.0", note = "Please use `load_model` instead.")]
///Loads an object's vertices and indices from a file name.
pub fn load_object(name: &str) -> Result<(Vec<ModelVertex>, Vec<u32>)> {
  let path = var("model_folder")? + "/" + name + "." + "obj";
  let path = Path::new(&path);

  let load_options = &tobj::LoadOptions {
    single_index: true,
    triangulate: true,
    ..Default::default()
  };

  let mut vertices = vec![];
  let mut indices = vec![];
  let mut lowest_y = 0.0;
  let mut unique_vertices = HashMap::new();

  let (models, _) = tobj::load_obj(path, load_options)?;

  for model in &models {
    let mesh = &model.mesh;

    for index in &mesh.indices {
      let position_offset = (index * 3) as usize;
      let texture_offset = (index * 2) as usize;

      let position = [
        mesh.positions[position_offset],
        mesh.positions[position_offset + 1],
        mesh.positions[position_offset + 2],
      ];
      if position[1] < lowest_y {
        lowest_y = position[1];
      }

      let texture = [mesh.texcoords[texture_offset], mesh.texcoords[texture_offset + 1]];

      let vertex = ModelVertex::new(position, texture);

      if let Some(index) = unique_vertices.get(&vertex) {
        indices.push(*index as u32)
      } else {
        let index = vertices.len();
        unique_vertices.insert(vertex, index);
        vertices.push(vertex);
        indices.push(index as u32);
      }
    }
  }

  for vertex in vertices.iter_mut() {
    vertex.pos[1] += lowest_y.abs();
  }

  Ok((vertices, indices))
}

pub fn load_cstring(path: &str) -> Result<CString> {
  let mut file = File::open(path)?;
  let mut buffer: Vec<u8> = Vec::with_capacity(file.metadata()?.len() as usize + 1);

  file.read_to_end(&mut buffer)?;

  if buffer.iter().find(|i| **i == 0).is_some() {
    return Err(FilesystemErrors::FileContainsNil.into());
  }
  Ok(unsafe { CString::from_vec_unchecked(buffer) })
}

///Loads the a [`Grid`]'s information.
pub fn load_grid(name: &str, extension: &str) -> Result<Grid> {
  let path = var("grid_folder")? + "/" + name + "." + extension;
  let grid_path = fs::read_to_string(path)?;
  let grid: Grid = serde_json::from_str(&grid_path)?;
  Ok(grid)
}

///Loads a configuration file from a given path.
pub fn load_config() -> Result<Config> {
  let root_directory = load_root_directory()?;
  let config_path = &(root_directory + "config");

  let config_path = "C:/Users/jamar/Documents/Hobbies/Coding/deux/config";

  let config = Config::builder().add_source(ConfigFile::with_name(config_path)).build().unwrap();
  Ok(config)
}

///Returns the path of the working directory as a string (the path "./").
fn load_root_directory() -> Result<String> {
  //For testing in the engine
  // let directory = current_dir()?.parent().unwrap().to_str().unwrap().to_owned() + "/";

  //For production
  let directory = current_dir()?.to_str().unwrap().to_owned() + "/";
  Ok(directory)
}

#[cfg(test)]
mod test {
  use super::{load_champion_json, load_shader};
  use crate::{
    errors::FilesystemErrors,
    filesystem::load::{load_config, load_root_directory},
    view::render_gl::ModelVertex,
  };
  use eyre::Result;
  use image::io::Reader;
  use std::{
    env::{set_var, var},
    path::Path,
  };
  use tobj;

  #[test]
  fn get_config() -> Result<()> {
    let config = load_config()?;
    let shader_path: String = config.get("shader_path")?;

    set_var("shader_path", shader_path);
    let test = var("shader_path")?;
    dbg!(test);
    Ok(())
  }

  #[test]
  fn get_working_directory() -> Result<()> {
    let directory = load_root_directory()?;
    dbg!(directory);
    Ok(())
  }

  #[test]
  fn load_image() -> Result<()> {
    let name = "cube";
    let path = format!("C:/Users/jamar/Documents/Hobbies/Coding/deux/assets/textures/{name}-diffuse.jpg");

    //Check whether the image file loaded
    match Reader::open(&path) {
      Ok(img) => match img.decode() {
        Ok(_) => {}
        Err(err) => return Err(FilesystemErrors::FailedToDecodeImage(err).into()),
      },
      Err(_) => return Err(FilesystemErrors::FailedToLoadImage { name: name.to_string(), path }.into()),
    }
    Ok(())
  }

  #[test]
  fn load_test_shader() -> Result<()> {
    let config = load_config()?;
    let shader_path = config.get::<String>("shader_path")?;
    set_var("shader_path", shader_path);

    let shader = load_shader("textured.vert")?;
    dbg!(shader);
    Ok(())
  }

  #[test]
  fn load_champion_components_from_json() -> Result<()> {
    let champion = load_champion_json("test_champion")?;
    let health = champion.health;
    dbg!(health);
    let speed = champion.unit_speed;
    dbg!(speed);
    let selection_radius = champion.selection_radius;
    dbg!(selection_radius);
    let pathing_radius = champion.pathing_radius;
    dbg!(pathing_radius);
    let auto_attack_missle_speed = champion.auto_attack_missle_speed;
    dbg!(auto_attack_missle_speed);
    let auto_attack_cooldown = champion.auto_attack_cooldown;
    dbg!(auto_attack_cooldown);
    let attack_damage = champion.attack_damage;
    dbg!(attack_damage);

    Ok(())
  }

  #[test]
  fn load_obj() -> Result<()> {
    let name = "C:/Users/Jamari/Documents/Hobbies/Coding/deux/target/debug/assets/box.obj";
    let path = Path::new(name);
    // let load_options = tobj::LoadOptions { single_index: (), triangulate: (), ignore_points: (), ignore_lines: () }
    let (models, _materials) = tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS)?;
    let mesh = &models[0].mesh;
    //unsure cloning is the way but I want to own the data not reference it
    let indices = &mesh.indices;

    let mut vertices = vec![];

    for index in indices {
      let position_offset = (index * 3) as usize;
      let texture_offset = (index * 2) as usize;
      let position = [
        mesh.positions[position_offset],
        mesh.positions[position_offset + 1],
        mesh.positions[position_offset + 2],
      ];
      let texture = [mesh.positions[texture_offset], mesh.positions[texture_offset + 1]];

      let vertex = ModelVertex::new(position, texture);
      vertices.push(vertex)
    }
    Ok(())
  }
}
