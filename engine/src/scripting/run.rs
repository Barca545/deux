use mlua::Lua;
use eyre::Result;

use crate::ecs::{component_lib::{AutoAttackScript, AutoAttack, Owner, Target}, query::ComponentRef, World};

use super::implementations::LuaEntity;

pub fn run(world: &World)-> Result<()>{
  run_damage_scripts(world)?;
  Ok(())
}

// these functions need to be exported and stuck into the loops for the other systems. i.e. 
// run_damage_scripts should be a step of the combat system

//System that runs damage scripts.
//add this to the creation step in the combat system to test
//might actually keep attack creation in pure rust instead of scripting until no other choices
pub fn run_damage_scripts(world: &World) -> Result<()>{
  //this works but any script that creates new entities *will* need to mutate world and be structured differently
  let lua = world.immut_get_resource::<Lua>().unwrap();  
  
  let mut query = world.query();

  //Search for all auto attack entities 
  let entities = query.with_component::<AutoAttack>().unwrap().run_entity();

  for entity in entities {
    let script_ref = entity.immut_get_component::<ComponentRef<AutoAttackScript>>().unwrap();
    let script = script_ref.get_component();
  
    //Convert the ids into types Lua can use
    let target = *entity.immut_get_component::<Target>().unwrap();
    let owner = *entity.immut_get_component::<Owner>().unwrap();
    
    let entity_id = LuaEntity::from(entity.id);
    let target_id = LuaEntity::from(target);
    let owner_id = LuaEntity::from(owner);

    lua.scope(|scope| {
    //Set the ids for the entity, its target, and its owner 
    lua.globals().set("entity", scope.create_userdata_ref(&entity_id)?)?;
    lua.globals().set("target", scope.create_userdata_ref(&target_id)?)?;
    lua.globals().set("owner", scope.create_userdata_ref(&owner_id)?)?;

    //Add the world 
    lua.globals().set("world", scope.create_userdata_ref(world)?)?;

    //Run the script
    lua.load(script.script()).exec()?; 
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