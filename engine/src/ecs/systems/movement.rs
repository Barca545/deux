use crate::{
	ecs::{
		component_lib::{Controllable, Destination, Position, Speed, Velocity},
		world_resources::ScreenDimensions,
		World,
	},
	math::{math::Vec3, MouseRay, Transforms},
};
use eyre::Result;
use glm::distance;

//the below should be incorporated into the system's description
//mouse ray should be a resource that is updated when the mouse moves
//arguably the mouse ray is information both the selection and this system needs
//selection needs to run first and do the AABB test
//this should only run if the selection test says nothing is selected
///Updates the destination and direction of all controllable entities.   
pub fn set_destination(world: &mut World, x: f64, y: f64, transforms: &Transforms) -> Result<()> {
	//I think I created a resource to hold events so get the mouse position from there

	//could I set up the immut_get_resource to return a result without needing to unwrap?

	// let transforms = world.immut_get_resource::<Transforms>().unwrap();

	//mouse ray should be a resource that is updated when the mouse moves
	//arguably the mouse ray is information both the selection and this system needs
	//selection needs to run first and do the AABB test
	//this should only run if the selection test says nothing is selected
	// let mouse_ray = world.immut_get_resource::<MouseRay>().unwrap();

	//this gets the index of the component and updates its destination
	//this is necesary for adding a commponent
	//this system does not need this but future systems will
	// let mut index = -1;
	// {
	//   let mut query = world.query();

	//   let entities = query
	//   .with_component::<Controllable>()?
	//   .run_entity();

	//   for entity in entities {
	//     index = entity.id as i32;
	//   }
	// }

	// world.add_component_to_entity_by_id(destination, index as usize)?;

	// let mut query = world.query();
	// let entities = query.with_component::<Destination>()?.run_entity();

	// for entity in entities {
	//   let destination = entity.immut_get_component::<Destination>()?;
	//   dbg!(*destination);
	// }

	let screen_dimensions = world.immut_get_resource::<ScreenDimensions>().unwrap();
	let mouse_ray = MouseRay::new(x, y, &screen_dimensions, &transforms).0;
	let intersection: Vec3 = mouse_ray.calculate_ray_ground_intersection();

	let mut query = world.query();
	let entities = query
		.with_component::<Controllable>()?
		.with_component::<Position>()?
		.with_component::<Destination>()?
		.with_component::<Speed>()?
		.with_component::<Velocity>()?
		.run_entity();

	for entity in entities {
		let mut destination = entity.mut_get_component::<Destination>()?;
		let position = entity.mut_get_component::<Position>()?;
		let speed = entity.immut_get_component::<Speed>()?;
		let mut velocity = entity.mut_get_component::<Velocity>()?;

		*destination = Destination(intersection);
		*velocity = Velocity::new(&position, &destination, &speed);
	}
	Ok(())
}

//maybe call this system like resolve movement
///Moves all units towards their destination by adding their velocity to their position.
pub fn resolve_movement(world: &World) -> Result<()> {
	let mut query = world.query();

	let entities = query
		.with_component::<Position>()?
		.with_component::<Destination>()?
		.with_component::<Speed>()?
		.with_component::<Velocity>()?
		.run_entity();

	for entity in entities {
		let mut position = entity.mut_get_component::<Position>()?;
		let destination = entity.immut_get_component::<Destination>()?;

		if position.0 != destination.0 {
			let velocity = entity.immut_get_component::<Velocity>()?;

			let new_position = Position(position.0 + velocity.0);

			let d1 =
				(new_position.0.x - position.0.x).powi(2) + (new_position.0.z - position.0.z).powi(2);
			let d2 = (destination.0.x - position.0.x).powi(2) + (destination.0.z - position.0.z).powi(2);

			if d1 < d2 {
				*position = new_position;
			} else {
				*position = Position::from(destination.clone());
			}
		}
	}
	Ok(())
}

#[cfg(test)]
mod test {
	use super::resolve_movement;
	use crate::ecs::{
		component_lib::{Destination, Position, Speed, Velocity},
		World,
	};
	use eyre::Result;

	#[test]
	fn get_direction() {
		let position = Position::new(0.0, 0.0, 0.0);
		let destination = Destination::new(3.0, 4.0, 0.0);
		let speed = Speed(1.0);

		let mut velocity = Velocity::new(&position, &destination, &speed);
		dbg!(velocity.0);

		let destination = Destination::new(3.0, 3.0, 0.0);
		velocity.update(&position, &destination);

		dbg!(velocity.0);
	}

	#[test]
	fn update_position() -> Result<()> {
		let position = Position::new(0.0, 0.0, 0.0);
		let destination = Destination::new(3.0, 0.0, 3.0);
		let speed = Speed(5.0);
		let velocity = Velocity::new(&position, &destination, &speed);
		dbg!(velocity);

		let mut world = World::new();

		world
			.register_component::<Position>()
			.register_component::<Destination>()
			.register_component::<Speed>()
			.register_component::<Velocity>();

		//entity the system should target
		world
			.create_entity()
			.with_component(position)?
			.with_component(destination)?
			.with_component(speed)?
			.with_component(velocity)?;

		//entity the system should ignore
		world
			.create_entity()
			.with_component(position)?
			.with_component(speed)?
			.with_component(velocity)?;

		//check that ppl without the components are not affected

		resolve_movement(&world)?;

		//Confirm the update occured
		let mut query = world.query();

		let entities = query.with_component::<Position>()?.run_entity();

		for entity in entities {
			let updated_position = entity.immut_get_component::<Position>()?;
			dbg!(updated_position.0);
		}
		Ok(())
	}
}
