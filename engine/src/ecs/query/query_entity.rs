use crate::{ecs::entities::Entities, errors::EcsErrors};
use eyre::Result;
use std::{
  any::{Any, TypeId},
  cell::{Ref, RefCell, RefMut},
  rc::Rc
};

use super::component_ref::ComponentRef;

type ExtractedComponents<'a> = Result<&'a Vec<Option<Rc<RefCell<dyn Any>>>>>;

pub struct QueryEntity<'a> {
  pub id:usize,
  entities:&'a Entities
}

impl<'a> QueryEntity<'a> {
  pub fn new(id:usize, entities:&'a Entities) -> Self {
    Self { id, entities }
  }

  fn extract_components<T:Any>(&self) -> ExtractedComponents {
    let typid = TypeId::of::<T>();
    let components = self.entities.components.get(&typid).ok_or(EcsErrors::ComponentNotRegistered)?;
    Ok(components)
  }

  pub fn immut_get_component<T:Any>(&self) -> Result<Ref<T>> {
    let components = self.extract_components::<T>()?;

    let borrowed_component = components[self.id].as_ref().ok_or(EcsErrors::ComponentDataDoesNotExist)?.borrow();
    Ok(Ref::map(borrowed_component, |any| any.downcast_ref::<T>().unwrap()))
  }

  pub fn mut_get_component<T:Any>(&self) -> Result<RefMut<T>> {
    let components = self.extract_components::<T>()?;

    let borrowed_component = components[self.id].as_ref().ok_or(EcsErrors::ComponentDataDoesNotExist)?.borrow_mut();
    Ok(RefMut::map(borrowed_component, |any| any.downcast_mut::<T>().unwrap()))
  }

  ///Returns an `Rc` smart pointer to the component.
  pub fn get_commonent_ref<T:Any>(&self) -> Result<ComponentRef<T>>{
    let components = self.extract_components::<T>()?;   
    let component = components[self.id].as_ref();
    
    match component {
      Some(component_ref) => {
        let component_ref = ComponentRef::new::<T>(component_ref.clone());
        Ok(component_ref)
      }
      None => {
        Err(EcsErrors::ComponentDataDoesNotExist.into())
      }
    }
  }
}
