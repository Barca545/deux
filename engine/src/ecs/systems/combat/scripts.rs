use mlua::Lua;
use crate::component_lib::{AutoAttack, AutoAttackScript, Owner, Target};
use crate::scripting::LuaEntity;
use crate::ecs::{query::ComponentRef, World};

//System that runs damage scripts.
//add this to the creation step in the combat system to test
//might actually keep attack creation in pure rust instead of scripting until no other choices
pub fn run_scripts(world: &World) {
  //this works but any script that creates new entities *will* need to mutate world and be structured differently
  let lua = world.immut_get_resource::<Lua>().unwrap();  
  
  let mut query = world.query();

  //Search for all auto attack entities 
  let entities = query.with_component::<AutoAttack>().unwrap().run
();

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
    }).unwrap();
  }
}