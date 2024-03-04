use crate::{component_lib::Owner, ecs::World, scripting::LuaEntity};
use mlua::{FromLua, Lua};
use std::rc::Rc;

// Refactor:
// -Should these take in the actual scripts and handle them there?
// -Why does u8 not bring up a bunch of the errors T does?

///Run scripts with an [`Owner`].
pub fn run_scripts(world: &mut World, owner: &Owner, script: &String) {
  let owner_id = LuaEntity::from(owner.0);
  let lua = world.get_resource::<Rc<Lua>>().unwrap().clone();
  lua
    .scope(|scope| {
      //Set the ids for the attack's owner
      lua.globals().set("owner", scope.create_userdata_ref(&owner_id)?)?;

      //Add the world
      lua.globals().set("world", scope.create_userdata_ref_mut(world)?)?;

      //Run the script
      lua.load(script).exec()?;
      Ok(())
    })
    .unwrap();
}

///Returns the result of running a [`Script`].
pub fn eval_scripts<'lua, T: for<'scope> FromLua<'scope> + Clone>(world: &mut World, owner: &Owner, script: &String) -> Option<T> {
  let owner_id = LuaEntity::from(owner.0);
  let lua = world.get_resource::<Rc<Lua>>().unwrap().clone();

  lua
    .scope(|scope| {
      //Set the ids for the attack's owner
      lua.globals().set("owner", scope.create_userdata_ref(&owner_id)?)?;

      //Add the world
      lua.globals().set("world", scope.create_userdata_ref_mut(world)?)?;

      //Run the script
      let value = lua.load(script).eval::<T>()?;

      Ok(Some(value))
    })
    .unwrap()
}
