use crate::{errors::{EcsErrors, FilesystemErrors}, view::render_gl::Vertex};
use eyre::Result;

use image::{io::Reader, DynamicImage};
use serde_json::Value;
use std::{
  ffi::CString,
  fs::{File, self},
  io::Read,
  path::{PathBuf, Path}, collections::HashMap
};

pub fn load_image(name:&str, extension:&str) -> Result<DynamicImage> {
  let path = name_to_pathbuff(name, extension);
  let image = Reader::open(path)
    .unwrap_or_else(|_| panic!("{}", { FilesystemErrors::FailedToLoadImage }))
    .decode()
    .unwrap_or_else(|_| panic!("{}", { FilesystemErrors::FailedToDecodeImage }));
  Ok(image)
}

pub fn load_components_json(name:&str)->Result<Value>{
  let path = name_to_pathbuff(name, "json");
  let json_string = fs::read_to_string(path)?;
  
  let json_value = serde_json::from_str(&json_string)?;
  Ok(json_value)
}

pub fn load_cstring(name:&str, extension:&str) -> Result<CString> {
  let mut file = File::open(name_to_pathbuff(name, extension))?;

  let mut buffer:Vec<u8> = Vec::with_capacity(file.metadata()?.len() as usize + 1);

  file.read_to_end(&mut buffer)?;

  if buffer.iter().find(|i| **i == 0).is_some() {
    return Err(EcsErrors::FileContainsNil.into());
  }
  Ok(unsafe { CString::from_vec_unchecked(buffer) })
}

pub fn load_shader(name:&str, extension:&str) -> Result<CString> {
  let shader = load_cstring(name, extension)?;
  Ok(shader)
}

///Loads an object's vertices and indices from a file name.
pub fn load_object(name:&str) -> Result<(Vec<Vertex>,Vec<u32>)> {
  let path_string = name_to_path_string(name, "obj");
  let path = Path::new(&path_string);
  
  let load_options = &tobj::LoadOptions {
    single_index: true,
    triangulate: true,
    ..Default::default()
  };

  let mut vertices = vec![];
  let mut indices = vec![];
  let mut lowest_y = 0.0;
  let mut unique_vertices = HashMap::new();
  //this eventually is where the materials come from (the second part of the tuple)

  //maybe just make the thing a solid color and don't worry about fighting textures so much rn
  //Fix textures:
  //--load in texture from the mtl file or wherever or figure out how to export a texture as a jpg
  //--fix wrapping
  //----UV wrapping?
  //----first thing to check is if the loaded textures match the textures the gpu is getting 
  
  let (models,_) = tobj::load_obj(path, load_options)?;
  
  for model in &models{
    let mesh = &model.mesh;

    for index in &mesh.indices{
      let position_offset = (index * 3) as usize;
      let texture_offset = (index * 2) as usize;
      
      let position = [
        mesh.positions[position_offset],
        mesh.positions[position_offset + 1],
        mesh.positions[position_offset + 2],
      ];
      if position[1] < lowest_y{
        lowest_y = position[1];
      }

      let texture = [
        mesh.texcoords[texture_offset],
        mesh.texcoords[texture_offset + 1]
      ];

      let vertex = Vertex::new(position, texture);
      
      if let Some(index) = unique_vertices.get(&vertex){
        indices.push(*index as u32)
      }
      else {
        let index = vertices.len();
        unique_vertices.insert(vertex, index);
        vertices.push(vertex);
        indices.push(index as u32);
      }
    }
  }
  
  for vertex in vertices.iter_mut(){
    vertex.pos[1] += lowest_y.abs();
  }

  Ok((vertices,indices))
}

fn name_to_pathbuff(name:&str, extension:&str) -> PathBuf {
  let root_dir = "C:/Users/Jamari/Documents/Hobbies/Coding/deux/target/debug/assets/".to_owned();
  let path:PathBuf = PathBuf::from(root_dir + name + "." + extension);
  path
}

fn name_to_path_string(name:&str, extension:&str) -> String {
  let root_dir = "C:/Users/Jamari/Documents/Hobbies/Coding/deux/target/debug/assets/".to_owned();
  let path_string = root_dir + name + "." + extension;
  path_string
}

#[cfg(test)]
mod test {
  use crate::{errors::FilesystemErrors, view::render_gl::Vertex};
  use eyre::Result;
  use image::io::Reader;
  use tobj;
  use std::path::Path;

  use super::{load_shader, load_components_json};
  #[test]
  fn load_image() -> Result<()> {
    let name = "C:/Users/Jamari/Documents/Hobbies/Coding/deux/target/debug/assets/wall.jpg";

    let path = Path::new(name);
    let _image = Reader::open(path)
      .unwrap_or_else(|_| panic!("{}", { FilesystemErrors::FailedToLoadImage }))
      .decode()
      .unwrap_or_else(|_| panic!("{}", { FilesystemErrors::FailedToDecodeImage }));

    Ok(())
  }

  #[test]
  fn load_test_shader() -> Result<()> {
    let shader = load_shader("textured", "vert")?;
    dbg!(shader);
    Ok(())
  }

  #[test]
  fn load_champion_components_from_json() -> Result<()> {
    let json = load_components_json("test_champion")?;
    dbg!(&json["Health"]);

    Ok(())
  }

  #[test]
  fn load_obj() -> Result<()> {
    let name = "C:/Users/Jamari/Documents/Hobbies/Coding/deux/target/debug/assets/box.obj";
    let path = Path::new(name);
    // let load_options = tobj::LoadOptions { single_index: (), triangulate: (), ignore_points: (), ignore_lines: () }
    let (models,_materials) = tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS)?;
    let mesh = &models[0].mesh;
    //unsure cloning is the way but I want to own the data not reference it
    let indices = &mesh.indices;
    
    let mut vertices = vec![];

    for index  in indices{
      let position_offset = (index * 3) as usize;
      let texture_offset = (index * 2) as usize;
      let position = [
        mesh.positions[position_offset],
        mesh.positions[position_offset + 1],
        mesh.positions[position_offset + 2],
      ];
      let texture = [
        mesh.positions[texture_offset],
        mesh.positions[texture_offset + 1]
      ];

      let vertex = Vertex::new(position, texture);
      vertices.push(vertex)
    }
    Ok(())
  }
}
