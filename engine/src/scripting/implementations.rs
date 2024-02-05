use std::ops::Deref;
use mlua::{FromLua, Lua, Result, UserData, UserDataMethods, Value};
use crate::ecs::{component_lib::{Health, Target}, World};

impl Deref for Health{
    type Target = Health;

  fn deref(&self) -> &Self::Target {
    &&self
  }
}

impl<'lua> FromLua<'lua> for Health{
  fn from_lua(value: Value<'lua>, lua: &'lua Lua) -> Result<Self> {
    match value {
      Value::UserData(data) => Ok(*data.borrow::<Self>()?),
      _ => unreachable!()
    }
  }
}

//this could go in scripting
//unsure I want to keep this since I moved stuff to the world struct
impl UserData for Health {
  fn add_fields<'lua, F: mlua::prelude::LuaUserDataFields<'lua, Self>>(fields: &mut F) {
    fields.add_field_method_get("max", |_,this | Ok(this.max));
    fields.add_field_method_get("remaining", |_,this | Ok(this.remaining));
  }
  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_method_mut("add", |_,this, amount:i32| {
      this.remaining += amount;
      Ok(())
    });
  }
}

impl UserData for World {
  //get the component data should basically just be numbers 
  //introduce it to th
  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    //get target method
    methods.add_method("add_health", |_,world, (target,value):(i32,i32)| {
      let mut health = world.mut_get_component_by_entity_id::<Health>(target as usize).unwrap();
      health.remaining += value;
      Ok(())
    });
    methods.add_method("remove_health", |_,world, (target,value):(i32,i32)| {
      let mut health = world.mut_get_component_by_entity_id::<Health>(target as usize).unwrap();
      health.remaining -= value;
      Ok(())
    });
  }
}