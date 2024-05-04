use crate::{component_lib::Target, ecs::World, math::MouseRay, scripting::LuaEntity};
use mlua::{FromLua, Lua};
use std::rc::Rc;

// Refactor:
// -Should these take in the actual scripts and handle them there?
// -Why does u8 not bring up a bunch of the errors T does?

///Run scripts with an [`Owner`].
pub fn run_scripts(world: &mut World, owner: &usize, ability_id: &usize, script: &String) {
  let owner_id = LuaEntity::from(owner);
  let ability_id = LuaEntity::from(ability_id);

  let lua = world.get_resource::<Rc<Lua>>().unwrap().clone();
  lua
    .scope(|scope| {
      //Set the ids for the attack's owner
      lua.globals().set("owner", scope.create_userdata_ref(&owner_id)?)?;

      //Set the id of the ability
      lua.globals().set("ability", scope.create_userdata_ref(&ability_id)?)?;

      //Add the world
      lua.globals().set("world", scope.create_userdata_ref_mut(world)?)?;

      //Run the script
      lua.load(script).exec()?;
      Ok(())
    })
    .unwrap();
}

///Returns the result of running a [`Script`].
pub fn eval_scripts_mouse<'lua, T: for<'scope> FromLua<'scope>>(
  world: &mut World,
  owner: &usize,
  target: &Target,
  mouse: &MouseRay,
  script: &String,
) -> Option<T> {
  let owner_id = LuaEntity::from(owner);
  let target_id = match target.0 {
    Some(id) => LuaEntity::from(id),
    None => LuaEntity::from(0),
  };

  let lua = world.get_resource::<Rc<Lua>>().unwrap().clone();

  lua
    .scope(|scope| {
      //Set the id of the scripts' owner
      lua.globals().set("owner", scope.create_userdata_ref(&owner_id)?)?;

      //Set the id of the casts' target
      if let Some(_) = target.0 {
        lua.globals().set("target", scope.create_userdata_ref(&target_id)?)?;
      }

      //Add the mouse ray
      lua.globals().set("mouse", scope.create_userdata_ref(mouse)?)?;

      //Add the world
      lua.globals().set("world", scope.create_userdata_ref_mut(world)?)?;

      //Run the script
      let value = lua.load(script).eval::<T>()?;

      Ok(Some(value))
    })
    .unwrap()
}

///Returns the result of running a [`Script`].
pub fn eval_scripts<'lua, T: for<'scope> FromLua<'scope>>(world: &mut World, id: &usize, owner: &usize, script: &String) -> Option<T> {
  //The id of the owner of the sr
  let owner_id = LuaEntity::from(owner);
  //The id of the entity holding the script
  let entity_id = LuaEntity::from(id);
  let lua = world.get_resource::<Rc<Lua>>().unwrap().clone();

  lua
    .scope(|scope| {
      //Set the ids for the scripts's owner
      lua.globals().set("owner", scope.create_userdata_ref(&owner_id)?)?;

      //Set the id for the script entity
      lua.globals().set("entity", scope.create_userdata_ref(&entity_id)?)?;

      //Add the world
      lua.globals().set("world", scope.create_userdata_ref_mut(world)?)?;

      //Run the script
      let value = lua.load(script).eval::<T>()?;

      Ok(Some(value))
    })
    .unwrap()
}
