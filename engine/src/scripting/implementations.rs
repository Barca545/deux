use crate::{
  arena::{Grid, Terrain},
  component_lib::{
    Armor, AttackDamage, AutoAttack, AutoAttackMesh, Destination, Health, Killed, MissleSpeed, Owner, Path, Position, PreviousPosition, SkinnedMesh,
    Target, Velocity,
  },
  ecs::World,
  math::Vec3,
  utility::calc_post_mitigation_damage,
};
use mlua::{UserData, UserDataMethods};

// Refactor
// -Figure out how to convert ECS errors into LuaErrors
// -Convert terrain to lua. ergonomically, I think I'll do strings until it shows as a performance issue

impl UserData for World {
  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    //Get an entity's target
    methods.add_method("getTarget", |_, world, entity: usize| {
      let target = world.get_component::<Target>(entity).unwrap();
      Ok(target.0)
    });

    //Spawn a targeted projectile
    methods.add_method_mut("spawnTargetedProjectile", |_, world, (owner, target): (usize, usize)| {
      let bundle;
      {
        //Get the owner's position
        let owner_position = world.get_component::<Position>(owner).unwrap();

        //Create the projectile's position information
        let attack_position = Position(owner_position.0);
        let previous_attack_position = PreviousPosition(owner_position.0);

        //Get the target's position
        let destination = Destination::from(*world.get_component::<Position>(target).unwrap());

        //Create the projectile speed
        let speed = world.get_component::<MissleSpeed>(owner).unwrap();

        //Calculate velocity
        let velocity = Velocity::new(&attack_position, &destination, &speed.0);

        //Get the mesh info
        let auto_attack_mesh = world.get_component::<AutoAttackMesh>(owner).unwrap();

        //Create the Target and owner wrappers
        let target = Target(Some(target));
        let owner = Owner(owner);

        bundle = (
          AutoAttack::default(),
          attack_position,
          previous_attack_position,
          *speed,
          velocity,
          SkinnedMesh::from(auto_attack_mesh.clone()),
          owner,
          target,
        );
      }
      world.create_entity().with_components(bundle).unwrap();
      Ok(())
    });

    //Increments the health of the queried entity
    methods.add_method("add_health", |_, world, (target, value): (usize, i32)| {
      let mut health = world.get_component_mut::<Health>(target as usize).unwrap();
      health.remaining += value;
      Ok(())
    });

    //Decrements the health of the queried entity
    methods.add_method("remove_health", |_, world, (target, value): (usize, i32)| {
      let mut health = world.get_component_mut::<Health>(target as usize).unwrap();
      health.remaining -= value;
      Ok(())
    });

    //Returns the attack damage of the queried entity
    methods.add_method("get_attack_damage", |_, world, entity_id: usize| {
      let attack_damage = world.get_component::<AttackDamage>(entity_id).unwrap().0;
      Ok(attack_damage)
    });

    //Deal mitigated damage to target enemy.
    //If the entity dies, give the script's owner a Killed component.
    methods.add_method_mut("deal_mitigated_damage", |_, world, (target, owner, damage): (usize, usize, i32)| {
      {
        let armor = world.get_component::<Armor>(target).unwrap();
        let post_mitigation_damage = calc_post_mitigation_damage(damage, armor.0);
        let mut health = world.get_component_mut::<Health>(target as usize).unwrap();
        health.remaining -= post_mitigation_damage;
      }

      let health = world.get_component::<Health>(target as usize).unwrap().remaining;
      //Apply the Killed component to the attack's owner if applicable
      if health < 0 {
        world.add_component(owner, Killed).unwrap();
      }
      Ok(())
    });

    //Retrieves an entity's Destination
    methods.add_method("get_destination", |_, world, entity: usize| {
      let destination = world.get_component::<Destination>(entity).unwrap();
      Ok([destination.0.x, destination.0.y, destination.0.z])
    });

    //Retrieves an entity's Position
    methods.add_method("get_position", |_, world, entity: usize| {
      let position = world.get_component::<Position>(entity).unwrap();
      Ok([position.0.x, position.0.y, position.0.z])
    });

    //Add a new node to an entity's Path component
    methods.add_method("add_node_to_path", |_, world, (entity, node): (usize, [f32; 3])| {
      let mut path = world.get_component_mut::<Path>(entity).unwrap();
      let node = Destination::from(node);
      path.push(node);
      Ok(())
    });
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
    LuaEntity(value.0.unwrap())
  }
}

impl From<usize> for LuaEntity {
  fn from(value: usize) -> Self {
    LuaEntity(value)
  }
}

impl UserData for LuaEntity {
  fn add_fields<'lua, F: mlua::prelude::LuaUserDataFields<'lua, Self>>(fields: &mut F) {
    fields.add_field_method_get("id", |_, entity_id| Ok(entity_id.0))
  }
}

impl UserData for Grid {
  fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
    methods.add_method("is_passable", |_, grid, position: [f32; 3]| Ok(grid.is_passable(Vec3::from(position))));

    // methods.add_method("test", |_,grid,|)
  }
}

impl UserData for Terrain {}

// impl<'lua, T> IntoLua<'lua> for Grid {
//   fn into_lua(self, lua: &'lua mlua::prelude::Lua) -> mlua::prelude::LuaResult<mlua::prelude::LuaValue<'lua>> {
//     todo!()
//   }
// }
