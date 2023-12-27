use crate::custom_errors::CustomErrors;
use eyre::Result;
use std::{
  any::{Any, TypeId},
  collections::HashMap,
  env::current_exe,
  ffi::CString,
  fs::File,
  io::Read,
  path::{Path, PathBuf}
};

#[derive(Default, Debug)]
pub struct Resource {
  data:HashMap<TypeId, Box<dyn Any>>
}

//add documentation
impl Resource {
  pub fn add(&mut self) -> &mut Self {
    self
  }

  pub fn from_user_defined_data(&mut self, data:impl Any) {
    let typeid:TypeId = data.type_id();
    self.data.insert(typeid, Box::new(data));
  }

  //this is for loading the asset folder
  //currently adding a new one would overwrite other ones so possibly find a way
  // around that
  pub fn path_to_asset_folder_from_relative_exe_path(&mut self, rel_path:&str) {
    let typeid:TypeId = TypeId::of::<std::path::PathBuf>();
    //this line makes it hard to test because this exe is not the one that would
    // eventually be used in final build use current_exe() instead of
    // current_dir()
    let exe_file_name = current_exe().unwrap();

    let exe_path = exe_file_name.parent().unwrap();
    let root_path = exe_path.join(rel_path);
    self.data.insert(typeid, Box::new(root_path));
  }

  //This is for loading a model from the assets folder
  //also not sure this should be here instead of being in world or the entities
  // or even a separate system since the main goal is to
  pub fn load_resource_from_cstring(&self, resource_name:&str) -> Result<CString> {
    let root_path:&PathBuf = self.get_ref::<PathBuf>().unwrap();

    let mut file = File::open(resource_name_to_path(root_path, resource_name))?;

    let mut buffer:Vec<u8> = Vec::with_capacity(file.metadata()?.len() as usize + 1);

    file.read_to_end(&mut buffer)?;

    if buffer.iter().find(|i| **i == 0).is_some() {
      return Err(CustomErrors::FileContainsNil.into());
    }
    Ok(unsafe { CString::from_vec_unchecked(buffer) })
  }

  pub fn get_ref<T:Any>(&self) -> Option<&T> {
    let typeid:TypeId = TypeId::of::<T>();
    if let Some(data) = self.data.get(&typeid) {
      data.downcast_ref()
    } else {
      None
    }
  }

  pub fn get_mut<T:Any>(&mut self) -> Option<&mut T> {
    let typeid:TypeId = TypeId::of::<T>();
    if let Some(data) = self.data.get_mut(&typeid) {
      data.downcast_mut()
    } else {
      None
    }
  }

  pub fn remove<T:Any>(&mut self) {
    let typeid:TypeId = TypeId::of::<T>();
    self.data.remove(&typeid);
  }
}

fn resource_name_to_path(root_dir:&Path, location:&str) -> PathBuf {
  let mut path:PathBuf = root_dir.into();

  for part in location.split("/") {
    path = path.join(part)
  }
  path
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
  use super::*;
  #[test]
  fn add_resource() {
    let resources:Resource = init_resource();
    let stored_resource:&Box<dyn Any> = resources.data.get(&TypeId::of::<WorldWidth>()).unwrap();

    let extracted_world_width:&WorldWidth = stored_resource.downcast_ref::<WorldWidth>().unwrap();
    assert_eq!(extracted_world_width.0, 100.0)
  }
  #[test]
  fn immut_get_resource() {
    let resources:Resource = init_resource();

    if let Some(extracted_world_width) = resources.get_ref::<WorldWidth>() {
      assert_eq!(extracted_world_width.0, 100.0)
    }
  }

  #[test]
  fn mut_get_resource() {
    let mut resources:Resource = init_resource();
    {
      let world_width:&mut WorldWidth = resources.get_mut::<WorldWidth>().unwrap();
      world_width.0 += 1.0
    }
    let world_width:&WorldWidth = resources.get_ref::<WorldWidth>().unwrap();
    assert_eq!(world_width.0, 101.0)
  }

  #[test]
  fn remove_resource() {
    let mut resources = init_resource();

    resources.remove::<WorldWidth>();
    let world_width_typeid:TypeId = TypeId::of::<WorldWidth>();
    assert!(!resources.data.contains_key(&world_width_typeid));
  }

  fn init_resource() -> Resource {
    let mut resources:Resource = Resource::default();
    let world_width:WorldWidth = WorldWidth(100.0);

    resources.add().from_user_defined_data(world_width);

    return resources;
  }

  #[test]
  fn load_cstring() {
    let mut resources = Resource::default();
    resources.add().path_to_asset_folder_from_relative_exe_path("ecs\\src\\");
    let root_path:&PathBuf = resources.get_ref::<PathBuf>().unwrap();
    let test_relative_path = resource_name_to_path(root_path, "triangle.frag");

    dbg!(&test_relative_path);

    let mut file = File::open(test_relative_path).unwrap();

    let mut buffer:Vec<u8> = Vec::with_capacity(file.metadata().unwrap().len() as usize + 1);

    file.read_to_end(&mut buffer).unwrap();

    if buffer.iter().find(|i| **i == 0).is_some() {
      dbg!(buffer);
    }
  }

  struct WorldWidth(pub f32);
}
