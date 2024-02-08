use mlua::{Lua, Value::Nil};
use eyre::Result;

use crate::ecs::{component_lib::{AutoAttackScript, AutoAttack, Owner, Target}, query::ComponentRef, World};

pub fn run(world: &World)-> Result<()>{
  run_damage_scripts(world)?;
  Ok(())
}

//alternatively instead of this being a whole system it could just be


//System that runs damage scripts.
//build an event system and have this query the event system for which events happened and require scripts
pub fn run_damage_scripts(world: &World) -> Result<()>{
  let lua = world.immut_get_resource::<Lua>().unwrap();
  //this works but any script that creates new entities *will* need to mutate world and be structured differently
  
  let mut query = world.query();

  //Search for all auto attackentities 
  let entities = query.with_component::<AutoAttack>().unwrap().run_entity();

  for entity in entities {
    //get the autoattack's scripts as a Vec<String>
    // handle the case where something does not have scripts
    let script_ref = entity.immut_get_component::<ComponentRef<AutoAttackScript>>().unwrap();
    let script = script_ref.get_component();
    
    //one alternative to this might be to make query entity something I just pass into the scope
    let entity_id =  entity.id;
    let target_id = entity.immut_get_component::<Target>().unwrap().0.unwrap();
    let owner_id = entity.immut_get_component::<Owner>().unwrap().id;
    
    lua.scope(|scope| {
      //Set the ids for the entity, its target and its owner 
      lua.globals().set("entity_id", entity_id)?;
      lua.globals().set("target_id", target_id)?;
      lua.globals().set("owner_id", owner_id)?;
      
      lua.globals().set("world", scope.create_userdata_ref(world)?)?;

      //add this to the creation step in the combat system to test
      //might actually keep attack creation in pure rust instead of scripting until no other choice

      //Run the script
      lua.load(script.script()).exec()?; 

      //keep in case I add multiple scripts for one event
      //Run the scripts
      // for script in &scripts.0 {
      //   lua.load(script).exec()?; 
      // }

      //Reset the ids for the entity, its target and its owner 
      //might be unnecescary as long as I always make sure a new thing that needs these resets them so they're not reused
      lua.globals().set("entity_id", Nil)?;
      lua.globals().set("target_id", Nil)?;
      lua.globals().set("owner_id", Nil)?;

      Ok(()) 
    })?;
  }
  Ok(())
}

#[cfg(test)]
mod tests {
  use mlua::Lua;
  use crate::ecs::{component_lib::Health, World};
  use eyre::Result;

  #[test]
  fn mutate_with_scope() -> Result<()>{
    let mut world = World::default();
    world.register_component::<Health>();

    world.create_entity().with_component(Health::new(100))?;

    let lua = Lua::new();
    lua.scope(|scope| {
    {
      //make lua function versions of the get component stuff for world and then pass world to the thing
      //could do some api like update health by target id and make it a user data method
      //then pass in world as a global
      //as far as I can tell the best way to do this is to just declare the methods globally and pass in a reference to world
      lua.globals().set("update_data", scope.create_function_mut(|_,(target,value):(usize,i32)|{
        let mut health = world.mut_get_component_by_entity_id::<Health>(target).unwrap();
        health.remaining += value;
        Ok(())
      })?
    )?;

    let script = r#"
    update_data(0,6)
      "#;

      lua.load(script).exec()?; 
    }
    Ok(())
    })?;
   
    let health = world.mut_get_component_by_entity_id::<Health>(0).unwrap();
    assert_eq!(health.remaining, 106);
    
    Ok(())
  }

  #[test]
  fn mutate_world() -> Result<()>{
    let mut world = World::default();
    world.register_component::<Health>();

    world.create_entity().with_component(Health::new(100))?;

    let lua = Lua::new();
    lua.scope(|scope| {
      //might not need to be mut 
      //need some way for a script's owner to set its own id
      lua.globals().set("world", scope.create_userdata_ref_mut(&mut world)?)?;

      let script = r#"
        world:add_health(0,6)
      "#;

      lua.load(script).exec()?; 
    Ok(())
    })?;
   
    let health = world.mut_get_component_by_entity_id::<Health>(0).unwrap();
    assert_eq!(health.remaining, 106);
    Ok(())
  }
}