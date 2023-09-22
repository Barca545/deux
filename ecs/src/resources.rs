use std::any::{Any, TypeId};
use std::collections::HashMap;

#[derive(Default,Debug)]
pub struct Resource{
  data:HashMap<TypeId,Box<dyn Any>>
}

//add documentation
impl Resource{
  pub fn add(&mut self, data:impl Any){
    let typeid: TypeId = data.type_id();
    self.data.insert(typeid, Box::new(data));
  }

  pub fn get_ref<T:Any>(&self) -> Option<&T>{
    let typeid: TypeId = TypeId::of::<T>();
    if let Some(data) = self.data.get(&typeid){
      data.downcast_ref()
    } 
    else {
      None
    }
  }

  pub fn get_mut<T:Any>(&mut self) -> Option<&mut T>{
    let typeid: TypeId = TypeId::of::<T>();
    if let Some(data) = self.data.get_mut(&typeid){
      data.downcast_mut()
    } 
    else {
      None
    }
  }

  pub fn remove<T:Any>(&mut self){
    let typeid:TypeId = TypeId::of::<T>();
    self.data.remove(&typeid);
  }

}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
  use super::*;
  #[test]
  fn add_resource(){
    let resources: Resource = init_resource();
    let stored_resource: &Box<dyn Any> = resources.data.get(&TypeId::of::<WorldWidth>()).unwrap();
    
    let extracted_world_width: &WorldWidth = stored_resource.downcast_ref::<WorldWidth>().unwrap();
    assert_eq!(extracted_world_width.0,100.0)
  }
  #[test]
  fn immut_get_resource(){
    let resources: Resource = init_resource();

    if let Some(extracted_world_width) = resources.get_ref::<WorldWidth>(){
      assert_eq!(extracted_world_width.0, 100.0)
    }
  }

  #[test]
  fn mut_get_resource(){
    let mut resources: Resource = init_resource(); 
    {
      let world_width: &mut WorldWidth = resources.get_mut::<WorldWidth>().unwrap();
      world_width.0 += 1.0 
    }
    let world_width: &WorldWidth = resources.get_ref::<WorldWidth>().unwrap();
    assert_eq!(world_width.0,101.0)
  }
  
  #[test]
  fn remove_resource(){
    let mut resources = init_resource();
    
    resources.remove::<WorldWidth>();
    let world_width_typeid:TypeId = TypeId::of::<WorldWidth>();
    assert!(!resources.data.contains_key(&world_width_typeid));
  }

  fn init_resource() -> Resource {
    let mut resources: Resource = Resource::default();
    let world_width: WorldWidth = WorldWidth(100.0);
    
    resources.add(world_width);

    return resources
  } 

  struct WorldWidth(pub f32);
}