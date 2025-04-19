use game_data::{
  Armor, AutoAttack, Controllable, Cooldown, Cooldowns, CrowdControlList, CrowdControlState, Dead,
  DebugModel, Destination, Exp, GameplayRadius, Gold, Health, IncomingDamage, Killed, Level,
  MagicDamage, MagicResist, MissleSpeed, Mob, MovementState, Owner, Path, PathingRadius,
  PersistentScript, PhysicalDamage, Player, PlayerState, Position, PreviousPosition, RunningScript,
  SelectionRadius, SkinnedRenderable, SpellResource, Stalker, StaticRenderable, Target, Team,
  UnitSpeed, Velocity, VisionRadius, KDA,
};
use nina::world::World;

pub fn register_components(world: &mut World,) {
  world
    // Basic stats
    .register_component::<Health>()
    .register_component::<Armor>()
    .register_component::<MagicResist>()
    .register_component::<PhysicalDamage>()
    .register_component::<MagicDamage>()
    .register_component::<SpellResource>()
    .register_component::<MissleSpeed>()
    .register_component::<UnitSpeed>()
    // Movement components
    .register_component::<Position>()
    .register_component::<PreviousPosition>()
    .register_component::<Destination>()
    .register_component::<Path>()
    .register_component::<Velocity>()
    .register_component::<Stalker>()
    // Radii components
    .register_component::<SelectionRadius>()
    .register_component::<PathingRadius>()
    .register_component::<VisionRadius>()
    .register_component::<GameplayRadius>()
    // Identification components
    .register_component::<AutoAttack>()
    .register_component::<Player>()
    .register_component::<Controllable>()
    .register_component::<Target>()
    .register_component::<Owner>()
    .register_component::<Team>()
    .register_component::<Killed>()
    .register_component::<Dead>()
    .register_component::<Mob>()
    // Timer components
    .register_component::<Cooldowns>()
    .register_component::<Cooldown>()
    // Combat components
    .register_component::<KDA>()
    .register_component::<IncomingDamage>()
    // Script components
    .register_component::<PersistentScript>()
    .register_component::<RunningScript>()
    // Level components
    .register_component::<Exp>()
    .register_component::<Level>()
    // Income components
    .register_component::<Gold>()
    // Status components
    .register_component::<PlayerState>()
    .register_component::<MovementState>()
    .register_component::<CrowdControlState>()
    .register_component::<CrowdControlList>()
    // Render components
    .register_component::<SkinnedRenderable>()
    .register_component::<StaticRenderable>()
    .register_component::<DebugModel>();
}
