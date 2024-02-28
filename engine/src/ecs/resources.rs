use std::{
  any::{Any, TypeId}, cell::{Ref, RefCell, RefMut}, collections::HashMap, rc::Rc
};

use crate::errors::EcsErrors;
use eyre::Result;
use super::bundle::TypeInfo;

#[derive(Default, Debug)]
pub struct Resource {
  data:HashMap<TypeId, Rc<RefCell<Box<dyn Any>>>>
}

//add documentation
impl Resource {
  pub fn add_resource(&mut self, data:impl Any) {
    let typeid:TypeId = data.type_id();
    self.data.insert(typeid, Rc::new(RefCell::new(Box::new(data))));
  }

  pub fn get_ref<T:Any>(&self) -> Result<Ref<T>> {
    let ty = TypeInfo::of::<T>();
    let resource_ref = self.data.get(&ty.id()).ok_or(EcsErrors::ResourceDataDoesNotExist{component:ty.type_name().to_string()})?;
    let borrowed_resource = resource_ref.as_ref().borrow();
    Ok(Ref::map(borrowed_resource, |resource| resource.downcast_ref::<T>().unwrap()))
    // if let Some(data) = self.data.get(&typeid) {
    //   data.downcast_ref()
    // } 
    // else {
    //   None
    // }
  }

  pub fn get_mut<T:Any>(&self) -> Result<RefMut<T>> {
    let ty = TypeInfo::of::<T>();
    let resource_ref = self.data.get(&ty.id()).ok_or(EcsErrors::ResourceDataDoesNotExist{component:ty.type_name().to_string()})?;
    let borrowed_resource = resource_ref.as_ref().borrow_mut();
    Ok(RefMut::map(borrowed_resource, |resource| resource.downcast_mut::<T>().unwrap()))
    // if let Some(data) = self.data.get_mut(&typeid) {
    //   data.downcast_mut()
    // } 
    // else {
    //   None
    // }
  }

  pub fn remove<T:Any>(&mut self) {
    let typeid:TypeId = TypeId::of::<T>();
    self.data.remove(&typeid);
  }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
  use super::*;
  #[test]
  fn add_resource() {
    let resources:Resource = init_resource();
    let stored_resource = resources.data.get(&TypeId::of::<WorldWidth>()).unwrap();
    let borrowed_resource = stored_resource.as_ref().borrow();

    let extracted_world_width:&WorldWidth = borrowed_resource.downcast_ref::<WorldWidth>().unwrap();
    assert_eq!(extracted_world_width.0, 100.0)
  }
  #[test]
  fn immut_get_resource() {
    let resources:Resource = init_resource();

    if let Ok(extracted_world_width) = resources.get_ref::<WorldWidth>() {
      assert_eq!(extracted_world_width.0, 100.0)
    };
  }

  #[test]
  fn mut_get_resource() {
    let resources:Resource = init_resource();
    {
      let mut world_width = resources.get_mut::<WorldWidth>().unwrap();
      world_width.0 += 1.0
    }
    let world_width = resources.get_ref::<WorldWidth>().unwrap();
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

    resources.add_resource(world_width);

    return resources;
  }
  struct WorldWidth(pub f32);
}
