use crate::{component_lib::AutoAttackScripts, ecs::World, event::{GameEvent, GameEventQueue}};

///System for beginning auto attacks. 
/// Queries the [`GameEventQueue`] for `AutoAttackStart` events. 
/// For each `AutoAttackStart` found, this system calls the any scripts associated with the [`Owner`]'s auto attack start up process.
pub fn auto_attack_start(world:&World){
  let events = world.get_resource::<GameEventQueue>().unwrap();
  
  let operation = |event: &mut GameEvent| {
    if let GameEvent::AutoAttackStart{owner} = event {
      let auto_attack_scripts = world.immut_get_component_by_entity_id::<AutoAttackScripts>(owner.0).unwrap();
      let start_script = auto_attack_scripts.get_script("AutoAttackStart");
      let start_script = r#"
        target = world:getTarget(owner.id)
        speed = world:getMissleSpeed(owner.id)
        world:spawnTargetedProjectile(owner.id, target.id, speed)
      "#;

      //AutoAttackStart needs to: 
      // -get the target 
      // -Spawn the attack obj
      // -Set the entity's state to Attacking
      // -Reset the auto wind up
    }
  };

  // events.process_events()
}

