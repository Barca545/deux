use std::any::{Any, TypeId};

use eyre::Result;

use crate::World;

//work on the dispatcher, 
//I think there is a much fancier 
//implementation for all this I can eventually do but this seems like a solid mvp for now

pub trait System{
  type Data;

  fn run(&mut self,world:&World,data:Self::Data)->Result<()>;
}

pub trait SystemData{
  //don't love this solution but this currently allows me to 
  fn get_ids()->Vec<TypeId>;
}

impl<P1> SystemData for (P1,)
where 
  P1:'static+Any,
{
  fn get_ids()->Vec<TypeId>{
    let mut ids = vec![];
    ids.push(TypeId::of::<P1>());
    ids
  }
}

impl<P1,P2> SystemData for (P1,P2)
where 
  P1:'static+Any,
  P2:'static+Any
{
  fn get_ids()->Vec<TypeId>{
    let mut  ids = vec![];
    ids.push(TypeId::of::<P1>());
    ids.push(TypeId::of::<P2>());
    ids
  }
}

impl<P1,P2,P3> SystemData for (P1,P2,P3)
where 
  P1:'static+Any,
  P2:'static+Any,
  P3:'static+Any,
{
  fn get_ids()->Vec<TypeId>{
    let mut  ids = vec![];
    ids.push(TypeId::of::<P1>());
    ids.push(TypeId::of::<P2>());
    ids.push(TypeId::of::<P3>());
    ids
  }
}

#[cfg(test)]
mod tests{
  use std::any::{TypeId, Any};

  use eyre::Result;

  use crate::{systems::system::{System, SystemData}, World, entities::query::{self, Query}};

  #[test]
  fn test_get_ref()->Result<()>{
    let mut world = build_world()?;
    
    struct System1;

    impl System for System1{     
      type Data = Vec<TypeId>;
      
      fn run(&mut self,world:&World,data:Self::Data)->Result<()>{
        let health_id = data[0];
        let speed_id = data[1];

        let mut query = Query::new(&world.entities);
        
        query
        .with_component_typeid(health_id)?
        .run_entity();
        

        Ok(())
      }
    }

    let mut system = System1;
    
    system.run(&world,vec![Health.type_id(),Speed.type_id()])?;

    Ok(())
  }

  #[derive(Debug)]
  struct Health(i32);
  struct Speed(f32);

  fn build_world()->Result<World>{
    let mut world = World::default();
    
    world
      .register_component::<Health>()
      .register_component::<Speed>();
    
    world
      .create_entity()
      .with_component(Health(100))?
      .with_component(Speed(2.0))?;

    Ok(world)
  }
}