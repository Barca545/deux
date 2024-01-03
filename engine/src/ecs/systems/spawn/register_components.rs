use crate::{ecs::{
  World, 
  component_lib::{Controllable, Destination, SelectionRadius, Position, Speed, Velocity, PathingRadius, SkinnedMesh, StaticMesh, Target, Team, MissleSpeed, AutoAttackCooldown, AutoAttack, AutoAttackMesh, Owner, AttackDamage, Health, GameplayRadius, Player, Gold, KDA, Exp, Level},
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
    .register_component::<Level>();
}