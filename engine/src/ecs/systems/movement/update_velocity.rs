use crate::{
  component_lib::{Destination, Position, UnitSpeed, Velocity},
  ecs::World,
};

//Refactor
// -Can just query for velocity?

///Updates the [`Velocity`] component of all entities in the [`World`].
/// Sets calculates the `Velocity` as the vector betwenn an entity's [`Position`] and [`Destination`].
pub fn update_velocity(world: &World) {
  let mut query = world.query();
  let entities = query
    .with_component::<Position>()
    .unwrap()
    .with_component::<Destination>()
    .unwrap()
    .with_component::<UnitSpeed>()
    .unwrap()
    .with_component::<Velocity>()
    .unwrap()
    .run();

  for entity in entities {
    let destination = entity.get_component_mut::<Destination>().unwrap();
    let position = entity.get_component_mut::<Position>().unwrap();
    let speed = entity.get_component::<UnitSpeed>().unwrap();
    let mut velocity = entity.get_component_mut::<Velocity>().unwrap();
    *velocity = Velocity::new(&position, &destination, &speed.total());
  }
}
