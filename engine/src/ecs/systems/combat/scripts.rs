use crate::component_lib::AutoAttackScript;
use crate::ecs::World;
use crate::scripting::LuaEntity;
use mlua::Lua;

//System that runs damage scripts.
//add this to the creation step in the combat system to test
//might actually keep attack creation in pure rust instead of scripting until no other choices
///Run all [`AutoAttackScript`]s.
pub fn run_scripts(world: &mut World) {
  // let mut query = world.query();

  // //Search for all auto attack entities
  // let entities = query.with_component::<Player>().unwrap().with_component::<Controllable>().unwrap().run();

  let script = world.get_component::<AutoAttackScript>(1).unwrap().clone();
  // let lua = world.get_resource::<Lua>().unwrap();
  let lua = Lua::new();
  let owner_id = LuaEntity::from(1);

  lua
    .scope(|scope| {
      //Set the ids for the attack's owner
      lua.globals().set("owner", scope.create_userdata_ref(&owner_id)?)?;

      //Add the world
      lua.globals().set("world", scope.create_userdata_ref_mut(world)?)?;

      //Run the script
      lua.load(script.script()).exec()?;
      Ok(())
    })
    .unwrap();
}
