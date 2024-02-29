use crate::{
  component_lib::{AutoAttack, Colliding, Exp, Gold, Killed, Owner, Target, KDA},
  ecs::World,
  event::{GameEvent, GameEventQueue},
};

//Refactor
// -Add death system
// -Gold should vary based on external factors not be hard coded.
// -EXP should vary depending on entity killed

///Queries all entities with `AutoAttack` and `Colliding` components.
/// If the auto attack killed, award its owner kill gold and increment their KDA.
/// Delete all collided auto attack entities.
pub fn resolve_attacks(world: &mut World) {
  //buffer holding the id of the attacks that hit and need to be removed at the end of this function
  let mut attacks_to_delete = Vec::new();

  //Fetch all entities with the AutoAttack and Colliding components
  let mut query = world.query();
  let entities = query.with_component::<AutoAttack>().unwrap().with_component::<Colliding>().unwrap().run();

  //Check the result of attacks landing.
  //Award gold and update KDAs
  for entity in entities {
    //Get a boolean balue indicating if the attack killed.
    let owner = entity.get_component::<Owner>().unwrap();
    let killed = world.get_component::<Killed>(owner.0).is_ok();

    if killed {
      //Get the target component
      let target = entity.get_component::<Target>().unwrap();

      //Update the owner's gold
      let mut owner_gold = world.get_component_mut::<Gold>(owner.0).unwrap();
      owner_gold.0 += 350;

      //Update the KDAs
      let mut owner_kda = world.get_component_mut::<KDA>(owner.0).unwrap();
      let mut target_kda = world.get_component_mut::<KDA>(target.0.unwrap()).unwrap();
      owner_kda.kill(1);
      target_kda.death(1);

      //Update the exp
      let mut owner_exp = world.get_component_mut::<Exp>(owner.0).unwrap();
      owner_exp.0 += 100;

      //Set the target entity's state to dead and start the death timer
    }
    //Add the entity id to the buffer to be deleted
    attacks_to_delete.push(entity.id);
  }

  //Delete all the attacks that hit
  for id in attacks_to_delete {
    world.delete_entity(id).unwrap();
  }
}

pub fn process_hits(world: &mut World) {
  let events = world.get_resource::<GameEventQueue>().unwrap();
  events.process_events(|event| match event {
    GameEvent::AutoAttackHit { attack_id, owner } => resolve_auto_hits(world, attack_id, owner),
    _ => {}
  })
}

fn resolve_auto_hits(world: &World, attack_id: &usize, owner: &Owner) {
  //this is where script logic goes
  world.delete_entity(index)
}
