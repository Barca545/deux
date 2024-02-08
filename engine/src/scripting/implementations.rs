use mlua::{UserData, UserDataMethods};
use crate::{ecs::{component_lib::{Armor, AttackDamage, Health}, World}, utility_functions::calc_post_mitigation_damage};

// impl Deref for Health{
//     type Target = Health;

//   fn deref(&self) -> &Self::Target {
//     &&self
//   }
// }

// impl<'lua> FromLua<'lua> for Health{
//   fn from_lua(value: Value<'lua>, lua: &'lua Lua) -> Result<Self> {
//     match value {
//       Value::UserData(data) => Ok(*data.borrow::<Self>()?),
//       _ => unreachable!()
//     }
//   }
// }

// //this could go in scripting
// //unsure I want to keep this since I moved stuff to the world struct
// impl UserData for Health {
//   fn add_fields<'lua, F: mlua::prelude::LuaUserDataFields<'lua, Self>>(fields: &mut F) {
//     fields.add_field_method_get("max", |_,this | Ok(this.max));
//     fields.add_field_method_get("remaining", |_,this | Ok(this.remaining));
//   }
//   fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
//     methods.add_method_mut("add", |_,this, amount:i32| {
//       this.remaining += amount;
//       Ok(())
//     });
//   }
// }

impl UserData for World {
  //get the component data should basically just be numbers 
  //introduce it to th
  //figure out how to convert the errors

  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
   //Increments the health of the queried entity
    methods.add_method("add_health", |_,world, (target,value):(usize,i32)| {
      let mut health = world.mut_get_component_by_entity_id::<Health>(target as usize).unwrap();
      health.remaining += value;
      Ok(())
    });
    
    //Decriments the health of the queried entity
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
    methods.add_method("deal_mitigated_damage", |_,world, (target,damage):(usize,i32)|{
      let armor = world.immut_get_component_by_entity_id::<Armor>(target).unwrap();
      let post_mitigation_damage = calc_post_mitigation_damage(damage, armor.0);
      let mut health = world.mut_get_component_by_entity_id::<Health>(target as usize).unwrap();
      health.remaining -= post_mitigation_damage;
      Ok(())
    })
  }
}

// impl<'a> UserData for QueryEntity<'a> {

// }