use std::{
  alloc::Layout,
  any::{type_name, Any, TypeId},
  hash::Hash,
};

// Refactor:
// -Use TypeInfo everywhere instead of TypeId
// -TypeInfo in its own file

#[derive(Debug, Eq, Clone, Copy)]
pub struct TypeInfo {
  typeid: TypeId,
  layout: Layout,
  type_name: &'static str,
}

impl Hash for TypeInfo {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.typeid.hash(state);
  }
}

impl PartialEq for TypeInfo {
  fn eq(&self, other: &Self) -> bool {
    self.typeid == other.typeid
  }
}

impl TypeInfo {
  pub fn new<T: Any>(value: &T) -> Self {
    TypeInfo {
      typeid: value.type_id(),
      layout: Layout::for_value(value),
      type_name: type_name::<T>(),
    }
  }

  pub fn of<T: 'static>() -> Self {
    TypeInfo {
      typeid: TypeId::of::<T>(),
      layout: Layout::new::<T>(),
      type_name: type_name::<T>(),
    }
  }

  ///Access the `Layout` of this component type.
  pub fn layout(&self) -> Layout {
    self.layout
  }

  ///Returns the size of the underlying `Layout`.
  pub fn size(&self) -> usize {
    self.layout.size()
  }

  ///Access the `TypeId` of this component type.
  pub fn id(&self) -> TypeId {
    self.typeid
  }

  pub fn type_name(&self) -> &str {
    self.type_name
  }
}

pub trait Bundle {
  ///Takes a callback that moves components out of the bundle one-by-one.
  fn safe_put(self, f: impl FnMut(TypeInfo, Box<dyn Any>));
}

macro_rules! impl_tuple {
  ($($name:ident),*) => {
    impl<$($name:'static),*> Bundle for ($($name,)*) {
      #[allow(unused_variables, unused_mut)]
      fn safe_put(self, mut f: impl FnMut(TypeInfo, Box<dyn Any>)) {
        #[allow(non_snake_case)]
        let ($(mut $name,)*) = self;
        $(
          f(
            TypeInfo::of::<$name>(),
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
mod test {
  use std::any::TypeId;

  use crate::{
    component_lib::{Health, UnitSpeed},
    ecs::{
      bundle::{Bundle, TypeInfo},
      World,
    },
  };

  #[test]
  fn component_impls_bundle() {
    let health = Health::new(10);

    let bundle = (health, health);

    takes_bundle(bundle);

    fn takes_bundle(bundle: impl Bundle) {
      bundle.safe_put(|ty, _component| {
        assert_eq!(ty.id(), TypeId::of::<Health>());
        dbg!(ty);
      })
    }
  }

  #[test]
  fn create_entity_from_bundle() {
    let mut world = World::default();
    world.register_component::<Health>().register_component::<UnitSpeed>();

    let id = world.reserve_entity();

    let health = Health::new(453);
    let speed = UnitSpeed::new(5.0);

    let healthid = TypeInfo::of::<Health>().id();
    assert_eq!(healthid, TypeId::of::<Health>());

    let bundle = (health, speed);

    world.add_components(id, bundle);

    let queried_health = world.get_component::<Health>(id).unwrap();
    assert_eq!(queried_health.max(), health.max());
  }
}
