use crate::data_lib::{Destination, Path, Position};
use mlua::UserData;

//Need to add the destinations in reverse order because they will be fetched by
// poping the vector one way to do this is to set the entity's current position
// as the goal and the destination as the start point

impl UserData for Destination {
  fn add_fields<'lua, F:mlua::prelude::LuaUserDataFields<'lua, Self>>(fields:&mut F) {
    fields.add_field_method_get("val", |_, destination| {
      let val = [destination.0.x, destination.0.y, destination.0.z];
      Ok(val)
    })
  }
}

impl UserData for Position {
  fn add_fields<'lua, F:mlua::prelude::LuaUserDataFields<'lua, Self>>(fields:&mut F) {
    fields.add_field_method_get("val", |_, position| {
      let val = [position.0.x, position.0.y, position.0.z];
      Ok(val)
    })
  }
}

impl UserData for Path {
  fn add_methods<'lua, M:mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods:&mut M) {
    methods.add_method_mut("add", |_, path, node:[f32; 3]| {
      let node = Destination::from(node);
      path.push(node);
      Ok(())
    });
  }
}
