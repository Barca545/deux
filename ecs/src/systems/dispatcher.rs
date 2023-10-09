use crate::{systems::system::{System,BoxedSystem}, World, entities::query_entity::QueryEntity};
use std::{any::Any, rc::Rc, cell::RefCell};

// pub struct Systems{
//   /*
//   does this actually needs to be a vec or some other type like I used in the entities
//   refcell/RC because not known at compilation time?
//   or Box or something
//   */
//   pub systems: Vec<BoxedSystem<>>,
//   //unsure this mut is the way to do this
  
// }

// impl Systems{
//   pub fn new() -> Self{
//     Systems { 
//       //new or default?
//       systems: Vec::new(),
//     }
//   }
  
  //takes ownership of the system?
  //can be chained together
  //this also needs to take in a mutable reference to the world and then grab the data the system needs to own

  ///if a system needs to use the same data as another system, add them in the order in which they need the data
//   pub fn insert_system<T:System>(&mut self,system:T) -> &mut Self{
//     self.systems.push(Box::new(system));
//     self
//   }
// }
//ideally add to world eventually
pub struct Dispatcher{
  pub systems: Vec<BoxedSystem>,
}

//I am not 100% sure I need Any here
impl Dispatcher<> {
  pub fn new()-> Self{
    Dispatcher { 
      systems: Vec::new()
    }
  }

  //I kinda want this to error if the system is messed up?
  pub fn register_system<T>(&mut self, system:T) -> &mut Self
  //why does this need to be static? I think this is bad because world is not static
  where T: System + 'static{
    let new_system = Box::new(system);
    self.systems.push(new_system);
    self
  }
  
  /*
  this should take in data from the world each tick
  then iterate over each system in the map and feed it the data it wants
  then return the output from the system to the appropriate part of world
  */
  pub fn run_systems(&mut self,world:&mut World){
    //unsure if a mutable ref needs a mut thing here
    let systems = &mut self.systems;

    for boxed_system in systems{
      //may need to be an as_mut
      let system = boxed_system.as_mut();
      system.run(world);
    }
  }

}

// impl Drop for Dispatcher<'_>{
//   fn drop(&mut self) {
//     std::mem::drop(self)
//   }
// }


#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests{
    use std::{cell::{RefMut, Ref}, ops::SubAssign, borrow::BorrowMut};

    use eyre::Result;

    use crate::{systems::system::System, World, entities::{entities::Component, query_entity::QueryEntity}};
    use super::Dispatcher;

  #[test]
  fn register_system()->Result<()>{
    let mut world = World::new();

    world.register_component::<Health>();

    world
      .create_entity()
      .with_component(Health(100_i32))?;

    let mut query = world.query();
      
    let health_query = query
      .with_component::<Health>()?
      .run_entity();

    let entity_health = *health_query[0].mut_get_component::<Health>()?;

    assert_eq!(entity_health.0,100);

    let damage_system = TakeDamageSystem::new()?;

    let mut dispatcher = Dispatcher::new();

    let dispatcher_system_length = dispatcher.systems.len();

    assert_eq!(dispatcher_system_length,0);

    dispatcher.register_system(damage_system);

    let new_dispatcher_system_length = dispatcher.systems.len();

    assert_eq!(new_dispatcher_system_length,1);
    
    Ok(())
 }

  #[test]
  fn run_independent_systems()-> Result<()>{
    let mut world = World::new();

    world
      .register_component::<Health>()
      .register_component::<Position>();

    world
      .create_entity()
      .with_component(Health(100_i32))?
      .with_component(Position{x:5.0,y:9.0})?;

    let damage_system = TakeDamageSystem::new()?;
    let movement_system = MovementSystem::new()?;
    
    let mut dispatcher = Dispatcher::new();

    dispatcher
      .register_system(damage_system)
      .register_system(movement_system)
      .run_systems(&mut world);

    let mut query = world.query();
      
    let entity_query = query
      .with_component::<Health>()?
      .with_component::<Position>()?
      .run_entity();

    //this should fail if the component given to the system is not the right component
    let entity_health = entity_query[0].immut_get_component::<Health>()?;
    let entity_position = entity_query[0].immut_get_component::<Position>()?;

    assert_eq!(entity_health.0,99);

    assert_eq!(entity_position.x,6.0);
    assert_eq!(entity_position.y,10.0);
    
    Ok(())
  }

  #[test]
  fn run_dependent_systems(){}
  
  #[derive(Clone, Copy)]
  pub struct Health(pub i32);
  impl SubAssign for Health{
    fn sub_assign(&mut self, change: Self) {
      *self = Self(self.0-change.0)
    }
}
  pub struct Position{
    pub x: f32,
    pub y: f32
  }

  pub enum MissileType {
    TargetedAbility,
    UntargetedAbility,
    AutoAttack  
  }
  
  pub struct TakeDamageSystem{}

  impl TakeDamageSystem {
    pub fn new() -> Result<Self> {
      Ok(TakeDamageSystem{})
    }
  }

  impl System for TakeDamageSystem {
    fn run(&mut self,world: &mut World){
      let mut query = world.query();
      
      let entity_query = query
        .with_component::<Health>().unwrap()
        .run_entity();
      
      let mut entity_health = entity_query[0].mut_get_component::<Health>().unwrap();

      *entity_health-=Health(1);
    }


  }

  pub struct MovementSystem{}
  
  impl<> MovementSystem{
    pub fn new() -> Result<Self>{
     Ok(MovementSystem{})
    }
  }

  impl System for MovementSystem{
    fn run(&mut self,world: &mut World) {
      let mut query = world.query();
      
      let entity_query = query
        .with_component::<Position>().unwrap()
        .run_entity();

      let mut entity_position = entity_query[0].mut_get_component::<Position>().unwrap();
      
      *entity_position = Position{
        x: 6.0,
        y: 10.0,
      }
    }
}

  pub struct CollisionSystem{}
}