use std::any::TypeId;

use crate::{
  arena::{Grid, Terrain},
  component_lib::{AbilityMap, AttackDamage, Destination, Health, Owner, Path, Position, SpellResource, Target},
  ecs::World,
  math::Vec3,
  utility::{create_ranged_auto_attack, has_resource, is_enemy, is_neutral, is_unoccupied, off_cooldown, target_is_alive},
};
use mlua::{FromLua, Lua, Result as LuaResult, UserData, UserDataMethods, Value};

// Refactor
// -Figure out how to convert ECS errors into LuaErrors
// -Convert terrain to lua. ergonomically, I think I'll do strings until it shows as a performance issue
// -I think the damage calculation might need to be here so I can have stuff like lifesteal etc

impl UserData for World {
  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    //Get an entity's target
    methods.add_method("getTarget", |_, world, entity: usize| {
      let target = world.get_component::<Target>(entity).unwrap();
      Ok(target.0)
    });

    //Pre-cast checks. Read more in the utility functions' casting module.
    methods.add_method("off_cooldown", |_, world, (entity, ability_name)| Ok(off_cooldown(world, entity, ability_name)));
    methods.add_method("has_resource", |_, world, (entity, cost)| Ok(has_resource(world, entity, cost)));
    methods.add_method("is_unoccupied", |_, world, entity| Ok(is_unoccupied(world, entity)));
    methods.add_method("target_is_alive", |_, world, target| Ok(target_is_alive(world, target)));
    methods.add_method("is_enemy", |_, world, (entity, target)| Ok(is_enemy(world, entity, target)));
    methods.add_method("is_neutral", |_, world, target| Ok(is_neutral(world, target)));

    //Update resources
    methods.add_method("add_resource", |_, world, (entity, amount): (usize, i32)| {
      let mut resource = world.get_component_mut::<SpellResource>(entity).unwrap();
      resource.0 += amount;
      Ok(())
    });

    methods.add_method("remove_resource", |_, world, (entity, amount): (usize, i32)| {
      let mut resource = world.get_component_mut::<SpellResource>(entity).unwrap();
      resource.0 -= amount;
      Ok(())
    });

    //Spawn a targeted projectile
    methods.add_method_mut("spawnTargetedProjectile", |_, world, (owner, target): (usize, usize)| {
      let auto_attack = create_ranged_auto_attack(world, Owner(owner), Target(Some(target)));
      world.create_entity().with_components(auto_attack).unwrap();
      Ok(())
    });

    //Runs the script's stop script
    methods.add_method("stop", |_, world, (owner, key): (usize, LuaTypeId)| {
      let map = world.get_component::<AbilityMap>(owner).unwrap();
      let id = key.0;
      let ability = map.get(id);
      let stop = ability.stop().to_owned().unwrap();
      Ok(stop)
    });

    //Increments the health of the queried entity
    methods.add_method("add_health", |_, world, (target, value): (usize, i32)| {
      let mut health = world.get_component_mut::<Health>(target as usize).unwrap();
      health.remaining += value;
      Ok(())
    });

    //Decrements the health of the queried entity
    methods.add_method("remove_health", |_, world, (target, value): (usize, i32)| {
      let mut health = world.get_component_mut::<Health>(target as usize).unwrap();
      health.remaining -= value;
      Ok(())
    });

    //Returns the attack damage of the queried entity
    methods.add_method("get_attack_damage", |_, world, entity_id: usize| {
      let attack_damage = world.get_component::<AttackDamage>(entity_id).unwrap().0;
      Ok(attack_damage)
    });

    //Retrieves an entity's Destination
    methods.add_method("get_destination", |_, world, entity: usize| {
      let destination = world.get_component::<Destination>(entity).unwrap();
      Ok([destination.0.x, destination.0.y, destination.0.z])
    });

    //Retrieves an entity's Position
    methods.add_method("get_position", |_, world, entity: usize| {
      let position = world.get_component::<Position>(entity).unwrap();
      Ok([position.0.x, position.0.y, position.0.z])
    });

    //Add a new node to an entity's Path component
    methods.add_method("add_node_to_path", |_, world, (entity, node): (usize, [f32; 3])| {
      let mut path = world.get_component_mut::<Path>(entity).unwrap();
      let node = Destination::from(node);
      path.push(node);
      Ok(())
    });
  }
}

pub struct LuaEntity(usize);

impl From<Owner> for LuaEntity {
  fn from(value: Owner) -> Self {
    LuaEntity(value.0)
  }
}

impl From<Target> for LuaEntity {
  fn from(value: Target) -> Self {
    LuaEntity(value.0.unwrap())
  }
}

impl From<usize> for LuaEntity {
  fn from(value: usize) -> Self {
    LuaEntity(value)
  }
}

impl From<&usize> for LuaEntity {
  fn from(value: &usize) -> Self {
    LuaEntity(*value)
  }
}

impl UserData for LuaEntity {
  fn add_fields<'lua, F: mlua::prelude::LuaUserDataFields<'lua, Self>>(fields: &mut F) {
    fields.add_field_method_get("id", |_, entity_id| Ok(entity_id.0))
  }
}

#[derive(Debug, Clone, Copy)]
pub struct LuaTypeId(pub TypeId);
impl UserData for LuaTypeId {}
impl<'lua> FromLua<'lua> for LuaTypeId {
  fn from_lua(value: Value<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
    match value {
      Value::UserData(ud) => Ok(*ud.borrow::<Self>()?),
      _ => unreachable!(),
    }
  }
}

impl UserData for Grid {
  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_method("is_passable", |_, grid, position: [f32; 3]| Ok(grid.is_passable(Vec3::from(position))));

    // methods.add_method("test", |_,grid,|)
  }
}

impl UserData for Terrain {}

// impl<'lua, T> IntoLua<'lua> for Grid {
//   fn into_lua(self, lua: &'lua mlua::prelude::Lua) -> mlua::prelude::LuaResult<mlua::prelude::LuaValue<'lua>> {
//     todo!()
//   }
// }
