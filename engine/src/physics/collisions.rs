use crate::math::{RayCast, Vec3};

use super::bounding_box::AABB3D;

///Compares the distance between the colliders' centers. If it is smaller than
/// the sum of their radii a collision has occured and the test will return
/// true.
pub fn ray_aabb3d_collision_test(aabb: AABB3D, ray: RayCast) -> bool {
  //distance to xmin and xmax
  let min_dx = (aabb.min.x - ray.origin.x) * ray.inverse_direction.x;
  let max_dx = (aabb.max.x - ray.origin.x) * ray.inverse_direction.x;

  let mut tmin = min_dx.min(max_dx);
  let mut tmax = min_dx.max(max_dx);

  //distance to ymin and ymax
  let min_dy = (aabb.min.y - ray.origin.y) * ray.inverse_direction.y;
  let max_dy = (aabb.max.y - ray.origin.y) * ray.inverse_direction.y;

  tmin = tmin.max(min_dy);
  tmax = tmax.min(max_dy);

  //distance to zmin and zmax
  let min_dz = (aabb.min.z - ray.origin.z) * ray.inverse_direction.z;
  let max_dz = (aabb.max.z - ray.origin.z) * ray.inverse_direction.z;

  tmin = tmin.max(min_dz);
  tmax = tmax.min(max_dz);

  tmax >= tmin
}

pub fn ray_aabb2d_collision_test(aabb: AABB3D, ray: RayCast) -> bool {
  let tx1 = (aabb.max.x - ray.origin.x) * ray.inverse_direction.x;
  let tx2 = (aabb.max.x - ray.origin.x) * ray.inverse_direction.x;

  let tmin = tx1.min(tx2);
  let tmax = tx1.max(tx2);

  let tz1 = (aabb.min.z - ray.origin.y) * ray.inverse_direction.y;
  let tz2 = (aabb.max.z - ray.origin.y) * ray.inverse_direction.y;

  let tmin = tmin.max(tz1.min(tz2));
  let tmax = tmax.min(tz1.max(tz2));

  tmax >= tmin
}

//honestly, use a circle for unit collisions
pub fn aabb3d_aabb3d_collision_test(aabb1: AABB3D, aabb2: AABB3D) -> bool {
  aabb1.min.x <= aabb2.max.x && aabb1.max.x <= aabb2.min.x &&
  // && 
  // aabb1.min.y <= aabb2.max.y && aabb1.max.y <= aabb2.min.y
  // && 
  aabb1.min.z <= aabb2.max.z && aabb1.max.z <= aabb2.min.z
}

pub fn horizonal_collision_test(moving_velocity: Vec3, moving_object_aabb: AABB3D, test_object_aabb: AABB3D) -> bool {
  let mut test = false;

  if moving_velocity.x > 0.0 {
    test = test_object_aabb.min.x >= moving_object_aabb.max.x;
  }
  //I think this can just be an else statement
  else if moving_velocity.x < 0.0 {
    test = test_object_aabb.min.x >= moving_object_aabb.max.x;
  }
  test
}

//confirm it returns true if two circles

///Returns `true` if two circles are colliding.
pub fn circle_circle_collision_test(position_1: Vec3, radius_1: f32, position_2: Vec3, radius_2: f32) -> bool {
  let center_to_center = ((position_1.x - position_2.x).powi(2) + (position_1.z - position_2.z).powi(2)).sqrt();
  let edge_to_edge = radius_1 + radius_2;

  center_to_center <= edge_to_edge
}

///Returns `true` if a point is inside a radius.
pub fn circle_point_collision_test(position_1: Vec3, position_2: Vec3, radius_2: f32) -> bool {
  // let center_to_center = ((position_1.x - position_2.x).powi(2) + (position_1.z - position_2.z).powi(2)).sqrt();
  // let edge_to_edge = radius_1 + radius_2;

  // center_to_center <= edge_to_edge

  //return true if center_to_center <= radius 2
  let center_to_center = ((position_1.x - position_2.x).powi(2) + (position_1.z - position_2.z).powi(2)).sqrt();
  center_to_center <= radius_2
}

#[cfg(test)]
mod test {
  use crate::{
    math::{Dimensions, MouseRay, Transforms, Vec3},
    physics::collisions::{ray_aabb3d_collision_test, AABB3D},
  };

  #[test]
  fn check_collision() {
    let position: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let aabb = AABB3D::new(position, 100.0, 5.0);

    let x = 1280.0 / 2.0;
    let y = 720.0 / 2.0;

    let screen_dimensions = Dimensions::new(720, 1280);
    let transforms = Transforms::new(screen_dimensions.aspect);

    let ray = MouseRay::new(x, y, &screen_dimensions, &transforms).0;

    let hit_check = ray_aabb3d_collision_test(aabb, ray);
    dbg!(hit_check);
  }
}
