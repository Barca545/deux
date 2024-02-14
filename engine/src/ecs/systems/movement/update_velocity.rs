use crate::{
  component_lib::{Destination, Position, UnitSpeed, Velocity}, 
  ecs::World
};

//Refactor
// -Can just query for velocity?

///Updates the velocities of all entities in the world.
pub fn update_velocity(world:&World) {
  let mut query = world.query();
  let entities = query
    .with_component::<Position>().unwrap()
    .with_component::<Destination>().unwrap()
    .with_component::<UnitSpeed>().unwrap()
    .with_component::<Velocity>().unwrap()
    .run
();

  for entity in entities {
    let destination = entity.mut_get_component::<Destination>().unwrap();
    let position = entity.mut_get_component::<Position>().unwrap();
    let speed = entity.immut_get_component::<UnitSpeed>().unwrap();
    let mut velocity = entity.mut_get_component::<Velocity>().unwrap();
    *velocity = Velocity::new(&position, &destination, &speed.0);
  }  
}