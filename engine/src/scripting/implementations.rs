use mlua::{UserData, UserDataMethods};
use crate::{component_lib::{identification::Killed, Armor, AttackDamage, Health, Owner, Target}, ecs::World, utility::calc_post_mitigation_damage};

// Refactor
// -Figure out how to convert ECS errors into LuaErrors

impl UserData for World {
  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
   //Increments the health of the queried entity
    methods.add_method("add_health", |_,world, (target,value):(usize,i32)| {
      let mut health = world.mut_get_component_by_entity_id::<Health>(target as usize).unwrap();
      health.remaining += value;
      Ok(())
    });
    
    //Decrements the health of the queried entity
    methods.add_method("remove_health", |_,world, (target,value):(usize,i32)| {
      let mut health = world.mut_get_component_by_entity_id::<Health>(target as usize).unwrap();
      health.remaining -= value;
      Ok(())
    });
    
    //Returns the attack damage of the queried entity
    methods.add_method("get_attack_damage", |_,world, entity_id:usize| {
      let attack_damage = world.immut_get_component_by_entity_id::<AttackDamage>(entity_id).unwrap().0;
      Ok(attack_damage)
    });

    //Deal mitigated damage to target enemy. 
    //If the entity dies, give the script's owner a Killed component.
    methods.add_method_mut("deal_mitigated_damage", |_,world, (target,owner, damage):(usize,usize,i32)|{
      {
        let armor = world.immut_get_component_by_entity_id::<Armor>(target).unwrap();
        let post_mitigation_damage = calc_post_mitigation_damage(damage, armor.0);
        let mut health = world.mut_get_component_by_entity_id::<Health>(target as usize).unwrap();
        health.remaining -= post_mitigation_damage;
      }

      let health = world.immut_get_component_by_entity_id::<Health>(target as usize).unwrap().remaining;
      //Apply the Killed component to the attack's owner if applicable
      if health < 0 {
        world.add_component(owner, Killed).unwrap();
      }
      Ok(())
    })
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
    LuaEntity(value.0)
  }
}

impl From<usize> for LuaEntity {
  fn from(value: usize) -> Self {
    LuaEntity(value)
  }
}

impl UserData for LuaEntity{
  fn add_fields<'lua, F: mlua::prelude::LuaUserDataFields<'lua, Self>>(fields: &mut F) {
    fields.add_field_method_get("id", |_,entity_id| Ok(entity_id.0))
  }
}