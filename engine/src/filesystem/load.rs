use super::champion::Champion;
use crate::{
  arena::Grid,
  component_lib::AbilityMap,
  errors::FilesystemErrors,
  view::{IndexBuffer, Material, Mesh, Model, ModelVertex, Renderer, Texture, VertexBuffer},
};
use config::{Config, File as ConfigFile};
use eyre::Result;
use image::io::Reader;
use std::{
  env::{current_dir, var},
  fs,
};
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
  use super::load_champion_json;
  use crate::{
    errors::FilesystemErrors,
    filesystem::load::{load_config, load_root_directory},
    view::ModelVertex,
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
