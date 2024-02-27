use crate::{
  component_lib::{Armor, AttackDamage, AutoAttack, AutoAttackMesh, AutoAttackScript, Colliding, Controllable, Cooldowns, CrowdControlList, CrowdControlState, Destination, Exp, GameplayRadius, Gold, Health, Killed, Level, MissleSpeed, MovementScript, MovementState, Owner, Path, PathingRadius, Player, Position, PreviousPosition, SelectionRadius, SkinnedMesh, StaticMesh, Target, Team, UnitSpeed, Velocity, VisionRadius, KDA}, 
  ecs::{query::ComponentRef, World}, 
  view::AABB3DDebugMesh
};

pub fn register_components(world:&mut World){
  world
    //Movement components
    .register_component::<Position>()
    .register_component::<PreviousPosition>()
    .register_component::<Destination>()
    .register_component::<UnitSpeed>()
    .register_component::<Velocity>()
    .register_component::<Colliding>()
    .register_component::<Path>()
    .register_component::<MovementScript>()
    //Radii components
    .register_component::<SelectionRadius>()
    .register_component::<PathingRadius>()
    .register_component::<VisionRadius>()
    .register_component::<GameplayRadius>()
    //Identification components
    .register_component::<AutoAttack>()
    .register_component::<Player>()
    .register_component::<Controllable>()
    .register_component::<Target>()
    .register_component::<Owner>()
    .register_component::<Team>()
    .register_component::<Killed>()
    //Timer components
    .register_component::<Cooldowns>()
    //Combat components
    .register_component::<MissleSpeed>()
    .register_component::<Armor>()
    .register_component::<AttackDamage>()
    .register_component::<AttackDamage>()
    .register_component::<Health>()
    .register_component::<KDA>()
    //Script components
    .register_component::<AutoAttackScript>()
    .register_component::<ComponentRef<AutoAttackScript>>()
    //Level components
    .register_component::<Exp>()
    .register_component::<Level>()
    //Income components
    .register_component::<Gold>()
    //Status components
    .register_component::<MovementState>()
    .register_component::<CrowdControlState>()
    .register_component::<CrowdControlList>()
    //Render components
    .register_component::<SkinnedMesh>()
    .register_component::<StaticMesh>()
    .register_component::<AutoAttackMesh>()
    .register_component::<AABB3DDebugMesh>();
}