use std::{alloc::Layout, any::{Any, TypeId}, hash::Hash};
use core::mem;

// git message: Added component bundle trait and command buffer
// Refactor:
// -Use TypeInfo everywhere instead of TypeId
// -Worst case scenario entities can just also box the data might cause performance issues in the future but meh can fix then if so
// -Can also look in

//not sure why the type id stored in type info is distinct from other type ids for the type
#[derive(Debug, Eq)]
pub struct TypeInfo{
  typeid: TypeId,
  layout:Layout
}

impl Hash for TypeInfo{
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.typeid.hash(state);
  }
}

impl PartialEq for TypeInfo {
  fn eq(&self, other: &Self) -> bool {
    self.typeid == other.typeid
  }
}

impl TypeInfo{
  pub fn new(value:&impl Any) -> Self {
    TypeInfo { 
      typeid: value.type_id(),
      layout: Layout::for_value(value)
    }
  }

  pub fn of<T:'static>()->Self{
    TypeInfo { 
      typeid: TypeId::of::<T>(),
      layout: Layout::new::<T>()
    }
  }
  
  ///Access the `Layout` of this component type.
  pub fn layout(&self) -> Layout{
    self.layout
  }

  ///Access the `TypeId` of this component type.
  pub fn id(&self) -> TypeId{
    self.typeid
  }
}


pub trait Bundle{
  ///Takes a callback that moves components out of the bundle one-by-one.
  unsafe fn put(self, f: impl FnMut(TypeInfo,*mut u8, ));
  fn safe_put(self, f: impl FnMut(TypeInfo, Box<dyn Any>));
}

// use std::any::{Any, TypeId};
// use std::collections::HashMap;
// trait Components {
//     fn move_into_vecs(self, map: &mut HashMap<TypeId, Vec<Box<dyn Any>>>);
// }

///tbh if I want to be lazy it can just take in workd

// // in the 3-element impl, for example
// impl<T1: 'static, T2: 'static, T3: 'static> Components for (T1, T2, T3) {
//     fn move_into_vecs(self, map: &mut HashMap<TypeId, Vec<Box<dyn Any>>>) {
//         let (v1, v2, v3) = self;
//         map.entry(TypeId::of::<T1>()).or_default().push(Box::new(v1));
//         map.entry(TypeId::of::<T1>()).or_default().push(Box::new(v2));
//         map.entry(TypeId::of::<T1>()).or_default().push(Box::new(v3));
//     }
// }

macro_rules! impl_tuple {
  ($($name:ident),*) => {
    impl<$($name:'static),*> Bundle for ($($name,)*) {
      #[allow(unused_variables, unused_mut)]
      //originally cast to u8
      unsafe fn put(self, mut f: impl FnMut(TypeInfo,*mut u8)) {
        #[allow(non_snake_case)]
        let ($(mut $name,)*) = self;
        $(
          f(
            TypeInfo::of::<$name>(),
            (&mut $name as *mut $name).cast::<u8>()
          );
          mem::forget($name);
        )*
      }

      #[allow(unused_variables, unused_mut)]
      fn safe_put(self, mut f: impl FnMut(TypeInfo, Box<dyn Any>)) {
        #[allow(non_snake_case)]
        let ($(mut $name,)*) = self;
        $(
          f(
            TypeInfo::of::<$name>(),
            // (&mut $name), 
            Box::new($name),
          );
        )*
      }
    }
  };
}

macro_rules! smaller_tuples_too {
  ($m: ident, $next: tt) => {
    $m!{}
    $m!{$next}
  };
  ($m: ident, $next: tt, $($rest: tt),*) => {
    smaller_tuples_too!{$m, $($rest),*}
    reverse_apply!{$m [$next $($rest)*]}
  };
}

macro_rules! reverse_apply {
  ($m: ident [] $($reversed:tt)*) => {
    $m!{$($reversed),*}  // base case
  };
  ($m: ident [$first:tt $($rest:tt)*] $($reversed:tt)*) => {
    reverse_apply!{$m [$($rest)*] $first $($reversed)*}
  };
}
smaller_tuples_too!(impl_tuple, O, N, M, L, K, J, I, H, G, F, E, D, C, B, A);

#[cfg(test)]
mod test{
  use std::any::TypeId;
  use crate::{component_lib::{Health, UnitSpeed}, ecs::{bundle::{Bundle, TypeInfo}, World}};

  #[test]
  fn component_impls_bundle(){
    let health = Health::new(10);

    let bundle = (health,health);

    takes_bundle(bundle);

    fn takes_bundle(bundle:impl Bundle){
      bundle.safe_put(|ty, _component|{
        assert_eq!(ty.id(),TypeId::of::<Health>());
        dbg!(ty);
      })
    }
  }

  #[test]
  fn create_entity_from_bundle(){
    let mut world = World::default();
    world.register_component::<Health>().register_component::<UnitSpeed>();

    let id = world.reserve_entity();

    let health = Health::new(453);
    let speed = UnitSpeed(5.0);

    let healthid = TypeInfo::of::<Health>().id();
    assert_eq!(healthid,TypeId::of::<Health>());

    let bundle = (health,speed);

    world.add_components(id, bundle);

    let queried_health = world.immut_get_component_by_entity_id::<Health>(id).unwrap();
    assert_eq!(queried_health.max,health.max);
  }
}