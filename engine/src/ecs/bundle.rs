use std::{alloc::Layout, any::Any};

// git message: Added component bundle trait and command buffer

pub struct TypeInfo{
  layout:Layout
}

impl TypeInfo{
  ///Access the `Layout` of this component type.
  pub fn layout(&self) -> Layout{
    self.layout
  }
}

pub trait Bundle{
  //Allow a callback that moves components out of the bundle
  unsafe fn put(self, f: impl FnMut(*mut u8, TypeInfo));
}

// impl Bundle for (){}
// impl<A:Any,B:Any> Bundle for (A,B,){}  
// impl<A:Any,B:Any,C:Any> Bundle for (A,B,C,){}  
// impl<A:Any,B:Any,C:Any,D:Any> Bundle for (A,B,C,D){}  
// impl<A:Any,B,C,D,E> Bundle for (A,B,C,D,E){}  

// smaller_tuples_too!(tuple_impl, O, N, M, L, K, J, I, H, G, F, E, D, C, B, A);

#[cfg(test)]
mod test{
  use crate::{component_lib::{Health, UnitSpeed}, ecs::{bundle::Bundle, World}};

  #[test]
  fn component_impls_bundle(){
    let mut world = World::default();
    world.register_component::<Health>();

    let health = Health::new(10);
    let speed = UnitSpeed(5.0);

    let bundle = (health,speed);

    // takes_bundle(bundle);
  }

  #[test]
  fn create_entity_from_bundle(){}

  fn takes_bundle(_bundle:impl Bundle){}
}