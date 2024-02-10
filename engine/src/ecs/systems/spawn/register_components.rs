use crate::{
  component_lib::{AttackDamage, AutoAttack, AutoAttackCooldown, AutoAttackScript, Controllable, Destination, Exp, GameplayRadius, Gold, Health, Level, MissleSpeed, Owner, PathingRadius, Player, Position, SelectionRadius, Target, Team, UnitSpeed, Velocity, KDA}, 
  ecs::{component_lib::{AutoAttackMesh, SkinnedMesh, StaticMesh}, 
  query::ComponentRef, World}, 
  view::AABB3DDebugMesh
};

pub fn register_components(world:&mut World){
  world
    .register_component::<SkinnedMesh>()
    .register_component::<StaticMesh>()
    .register_component::<Position>()
    .register_component::<Destination>()
    .register_component::<UnitSpeed>()
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
    .register_component::<AutoAttackScript>()
    .register_component::<ComponentRef<AutoAttackScript>>();
}