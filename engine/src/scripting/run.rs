//this should probably be renamed tests

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