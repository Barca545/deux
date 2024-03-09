use crate::{
  component_lib::{Armor, DamageType, Dead, Health, IncomingDamage, MagicResist},
  ecs::World,
  event::{GameEvent, GameEventQueue},
  utility::{calc_gross_magic_damage, calc_gross_physical_damage},
};

// Refactor:
// -Add logic to handle deaths.
//  Give the killer gold, set a death timer, etc

///Subsystem which handles processing all damage entities recieve during a tick and killing them if necessary.
/// If the an was killed, give it a [`Dead`] component and create an `EntityKilled` [`GameEvent`].
pub fn process_damage(world: &mut World) {
  let mut damaged_entities = Vec::default();

  let mut query = world.query();
  let entities = query.with_component::<IncomingDamage>().unwrap().run();
  for entity in entities {
    let mut incoming_damage = entity.get_component_mut::<IncomingDamage>().unwrap();

    for damage in incoming_damage.drain() {
      match damage {
        DamageType::Physical { owner, damage } => {
          let armor = entity.get_component::<Armor>().unwrap();
          let gross_damage = calc_gross_physical_damage(damage, *armor);
          damaged_entities.push((owner, entity.id, gross_damage))
        }
        DamageType::Magic { owner, damage } => {
          let magic_resist = entity.get_component::<MagicResist>().unwrap();
          let gross_damage = calc_gross_magic_damage(damage, *magic_resist);
          damaged_entities.push((owner, entity.id, gross_damage))
        }
        DamageType::True { owner, damage } => {
          //No need to calculate damage for true damage
          damaged_entities.push((owner, entity.id, damage))
        }
      }
    }
  }

  for (owner, target, damage) in damaged_entities {
    apply_damage(world, owner, target, damage);
  }
}

///Function which applies damage to an entity and checks if the damage killed it.
/// If the entity was killed, give it a [`Dead`] component and create an `EntityKilled` [`GameEvent`].
pub fn apply_damage(world: &mut World, owner: usize, target: usize, damage: i32) {
  //Deal damage
  let is_zero;
  {
    let mut target_health = world.get_component_mut::<Health>(target).unwrap();
    target_health.sub_remaining(damage);
    is_zero = target_health.is_zero();
  }

  //If a target is dead, Mark them as dead.
  if is_zero {
    //Give the target the Dead marker
    world.add_component(target, Dead).unwrap();
    //Create a target killed event
    let mut events = world.get_resource_mut::<GameEventQueue>().unwrap();
    events.push(GameEvent::EntityKilled { target, killer: owner })
  }
}
