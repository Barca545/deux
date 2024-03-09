use std::any::{Any, TypeId};

use crate::{
  arena::{Grid, Terrain},
  component_lib::{
    AbilityMap, DamageType, Destination, Health, IncomingDamage, MagicDamage, Owner, Path, PhysicalDamage, Position, SpellResource, Target, UnitSpeed,
  },
  ecs::World,
  event::AbilityThree,
  math::{MouseRay, Vec3},
  utility::{
    create_persistent_script, create_ranged_auto_attack, displacement, has_resource, is_enemy, is_neutral, is_unoccupied, off_cooldown, target_is_alive,
  },
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
      let target = LuaEntity::from(target.0.unwrap());
      Ok(target)
    });

    //Add an instance of magic damage to an entitys Incoming damage tracker
    methods.add_method("dealMagicDamage", |_, world, (owner, target, damage): (usize, usize, i32)| {
      let mut incoming_damage = world.get_component_mut::<IncomingDamage>(target).unwrap();
      let damage = DamageType::Magic { owner, damage };
      incoming_damage.push(damage);
      Ok(())
    });

    //Add an instance of physical damage to an entitys Incoming damage tracker
    methods.add_method("dealPhysicalDamage", |_, world, (owner, target, damage): (usize, usize, i32)| {
      let mut incoming_damage = world.get_component_mut::<IncomingDamage>(target).unwrap();
      let damage = DamageType::Physical { owner, damage };
      incoming_damage.push(damage);
      Ok(())
    });

    //Add an instance of true damage to an entitys Incoming damage tracker
    methods.add_method("dealTrueDamage", |_, world, (owner, target, damage): (usize, usize, i32)| {
      let mut incoming_damage = world.get_component_mut::<IncomingDamage>(target).unwrap();
      let damage = DamageType::True { owner, damage };
      incoming_damage.push(damage);
      Ok(())
    });

    //Update resources
    methods.add_method("addResource", |_, world, (entity, amount): (usize, i32)| {
      let mut resource = world.get_component_mut::<SpellResource>(entity).unwrap();
      resource.add_remaining(amount);
      Ok(())
    });

    methods.add_method("removeResource", |_, world, (entity, amount): (usize, i32)| {
      let mut resource = world.get_component_mut::<SpellResource>(entity).unwrap();
      resource.sub_remaining(amount);
      Ok(())
    });

    //Spawn a targeted projectile
    methods.add_method_mut("spawnTargetedProjectile", |_, world, (owner, target): (usize, usize)| {
      let auto_attack = create_ranged_auto_attack(world, Owner(owner), Target(Some(target)));
      world.create_entity().with_components(auto_attack).unwrap();
      Ok(())
    });

    //Deletes the script entity
    methods.add_method_mut("stop", |_, world, entity: usize| {
      world.delete_entity(entity).unwrap();
      Ok(())
    });

    //Increments the health an entity
    methods.add_method("addHealth", |_, world, (target, amount): (usize, i32)| {
      let mut health = world.get_component_mut::<Health>(target as usize).unwrap();
      health.add_remaining(amount);
      Ok(())
    });

    //Decrements the health of an entity
    methods.add_method("removeHealth", |_, world, (target, amount): (usize, i32)| {
      let mut health = world.get_component_mut::<Health>(target as usize).unwrap();
      health.sub_remaining(amount);
      Ok(())
    });

    //Returns the physical damage of the queried entity
    methods.add_method("getPhysicalDamage", |_, world, entity_id: usize| {
      let attack_damage = world.get_component::<PhysicalDamage>(entity_id).unwrap().total();
      Ok(attack_damage)
    });

    methods.add_method("getMagicDamage", |_, world, entity_id: usize| {
      let attack_damage = world.get_component::<MagicDamage>(entity_id).unwrap().total();
      Ok(attack_damage)
    });

    //Retrieves an entity's Destination
    methods.add_method("getDestination", |_, world, entity: usize| {
      let destination = world.get_component::<Destination>(entity).unwrap();
      Ok([destination.0.x, destination.0.y, destination.0.z])
    });

    //Retrieves an entity's Position
    methods.add_method("getPosition", |_, world, entity: usize| {
      let position = world.get_component::<Position>(entity).unwrap();
      Ok([position.0.x, position.0.y, position.0.z])
    });

    //Add a new node to an entity's Path component
    methods.add_method("addNodetoPath", |_, world, (entity, node): (usize, [f32; 3])| {
      let mut path = world.get_component_mut::<Path>(entity).unwrap();
      let node = Destination::from(node);
      path.push(node);
      Ok(())
    });

    //Instantly move an entity to a new position
    methods.add_method("blink", |_, world, (owner, new_position): (usize, [f32; 3])| {
      //set the position of the entity and its destination equal to the target position
      let mut position = world.get_component_mut::<Position>(owner).unwrap();
      let mut destination = world.get_component_mut::<Destination>(owner).unwrap();
      *position = Position::from(new_position);
      *destination = Destination::from(new_position);
      //need logic to eject a player if they're inside another hitbox
      //loop through entities, check for a collision, if inside push in direction facing by collided entity radius then check again not inside, etc
      // if it does this more than two times, try another direction
      // (backwards then 90 degrees etc)
      Ok(())
    });

    //Increase a unit's movement speed
    methods.add_method("accelerate", |_, world, (owner, amount): (usize, f32)| {
      let mut speed = world.get_component_mut::<UnitSpeed>(owner).unwrap();
      speed.add_bonus(amount);
      Ok(())
    });

    //Spawn an entity to track how long a script should execute
    methods.add_method_mut("spawnPersistentScript", |_, world, (owner, duration): (usize, f64)| {
      let running;
      let stop;
      {
        let map = world.get_component::<AbilityMap>(owner).unwrap();
        let script = map.get(AbilityThree.type_id());
        running = script.running();
        stop = script.stop();
      }
      create_persistent_script(world, owner, running, stop, duration).unwrap();
      Ok(())
    });

    methods.add_method("knockback", |_, world, (owner, target, speed, dist): (usize, usize, f32, f32)| {
      displacement(world, owner, target, speed, dist);
      Ok(())
    });

    methods.add_method("pull", |_, world, (owner, target, speed, dist): (usize, usize, f32, f32)| {
      displacement(world, owner, target, -speed, -dist);
      Ok(())
    });

    //Pre-cast checks. Read more in the utility functions' casting module.
    methods.add_method("offCooldown", |_, world, (entity, ability_name)| Ok(off_cooldown(world, entity, ability_name)));
    methods.add_method("hasResource", |_, world, (entity, cost)| Ok(has_resource(world, entity, cost)));
    methods.add_method("isUnoccupied", |_, world, entity| Ok(is_unoccupied(world, entity)));
    methods.add_method("targetIsalive", |_, world, target| Ok(target_is_alive(world, target)));
    methods.add_method("isEnemy", |_, world, (entity, target)| Ok(is_enemy(world, entity, target)));
    methods.add_method("isNeutral", |_, world, target| Ok(is_neutral(world, target)));
  }
}

#[derive(Debug, Clone, Copy)]
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

//Could be useful if I ever have scripts that swap or something
#[derive(Debug, Clone, Copy)]
pub struct LuaTypeId(pub TypeId);
impl UserData for LuaTypeId {}
impl<'lua> FromLua<'lua> for LuaTypeId {
  fn from_lua(value: Value<'lua>, _: &'lua Lua) -> LuaResult<Self> {
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

impl UserData for MouseRay {
  //Returns the mouse's position on the ground
  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_method("ground_intersection", |_, mouse, ()| {
      let position = mouse.ray_ground_intersection();
      Ok([position.x, position.y, position.z])
    });
  }
}

impl UserData for Terrain {}

// impl<'lua, T> IntoLua<'lua> for Grid {
//   fn into_lua(self, lua: &'lua mlua::prelude::Lua) -> mlua::prelude::LuaResult<mlua::prelude::LuaValue<'lua>> {
//     todo!()
//   }
// }
