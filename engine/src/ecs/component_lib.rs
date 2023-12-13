use crate::{
	math::math::Vec3,
	view::render_gl::{Program, Vertex},
};
use std::any::Any;
//unsure if this is where I should store stuff like movespeed
//why does making both dyn Any cause an issue? Says the size for both must be known at compile time but I thought that defeated the point of any?

///Represents units the player can control.
#[derive(Debug, Clone, Copy)]
pub struct Controllable;

#[derive(Debug, Clone, Copy)]
pub struct Health(i64);

#[derive(Debug, Clone, Copy)]
pub struct Position(pub Vec3);
impl Position {
	pub fn new(x: f32, y: f32, z: f32) -> Self {
		Position(Vec3::new(x, y, z))
	}
}
impl From<Destination> for Position {
	fn from(destination: Destination) -> Self {
		Position(destination.0)
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Destination(pub Vec3);
impl Destination {
	pub fn new(x: f32, y: f32, z: f32) -> Self {
		Destination(Vec3::new(x, y, z))
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Velocity(pub Vec3);
impl Velocity {
	pub fn new(position: &Position, destination: &Destination, speed: &Speed) -> Self {
		let velocity: Vec3 = (destination.0 - position.0).normalize().scale(speed.0);
		Velocity(velocity)
	}

	pub fn update(&mut self, position: &Position, destination: &Destination) {
		self.0 = (destination.0 - position.0).normalize();
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Speed(pub f32);

// #[derive(Debug,Clone,Copy)]
pub struct Model(pub Vec<Vertex>);

pub struct GroundModel(pub Vec<Vertex>);

// #[derive(Debug,Clone,Copy)]
//this should be the inner and outer bounding box
pub struct Hitbox {
	center: Box<dyn Any>, //probably should just use vertex's for this or something
	bounding_box: Box<dyn Any>,
}

//Can use the following two to construct a ward entity.
//Duration can be reused for other stuff too.
pub struct VisionRange(i32);
pub struct Duration(f64);

pub enum MovementState {
	DASHING,
	WALKING,
}

pub type EntityId = i32; //probably can't be a number. This is a placeholder.

pub enum CrowdControlState {
	STUNNED(EntityId),
	SLOWED(EntityId),
	AIRBORNE(EntityId),
}

pub type CrowdControlList = Vec<CrowdControlState>;

pub struct ModelUniformLocation(pub i32);
pub struct ViewUniformLocation(pub i32);
pub struct ProjectionUniformLocation(pub i32);
