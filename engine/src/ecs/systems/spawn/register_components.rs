use crate::{ecs::{
  World, 
  component_lib::{AttackDamage, AutoAttack, AutoAttackCooldown, AutoAttackMesh, Controllable, Destination, Exp, GameplayRadius, Gold, Health, Level, MissleSpeed, Owner, PathingRadius, Player, Position, Scripts, SelectionRadius, SkinnedMesh, Speed, StaticMesh, Target, Team, Velocity, KDA},
}, view::AABB3DDebugMesh};

pub fn register_components(world:&mut World){
  world
    .register_component::<SkinnedMesh>()
    .register_component::<StaticMesh>()
    .register_component::<Position>()
    .register_component::<Destination>()
    .register_component::<Speed>()
    .register_component::<Velocity>()
    .register_component::<Controllable>()
    .register_component::<SelectionRadius>()
    .register_component::<AABB3DDebugMesh>()
    .register_component::<PathingRadius>()
    .register_component::<Target>()
    .register_component::<Team>()
    .register_component::<MissleSpeed>()
    .register_component::<AutoAttackMesh>()
    .register_component::<AutoAttackCooldown>()
    .register_component::<AutoAttack>()
    .register_component::<Owner>()
    .register_component::<AttackDamage>()
    .register_component::<Health>()
    .register_component::<GameplayRadius>()
    .register_component::<Player>()
    .register_component::<Gold>()
    .register_component::<KDA>()
    .register_component::<Exp>()
    .register_component::<Level>()
    .register_component::<Scripts>();
}