use std::cmp::max;

use glm::vec3;
use crate::math::{Vec3, RayCast};

use super::bounding_box::AABB3D;




//refactor to use the slab algorithm and avoid branches
pub fn check_ray_aabb3d_collision(aabb:AABB3D,ray:RayCast)->bool{
  //distance to xmin and xmax
  let min_dx = (aabb.min.x - ray.origin.x)*ray.inverse_direction.x;
  let max_dx = (aabb.max.x - ray.origin.x)*ray.inverse_direction.x;

  let mut tmin = min_dx.min(max_dx);
  let mut tmax = min_dx.max(max_dx);

  //distance to ymin and ymax
  let min_dy = (aabb.min.y - ray.origin.y)*ray.inverse_direction.y;
  let max_dy = (aabb.max.y - ray.origin.y)*ray.inverse_direction.y;

  tmin = tmin.max(min_dy);
  tmax = tmax.min(max_dy);

  //distance to zmin and zmax
  let min_dz = (aabb.min.z - ray.origin.z)*ray.inverse_direction.z;
  let max_dz = (aabb.max.z - ray.origin.z)*ray.inverse_direction.z;

  tmin = tmin.max(min_dz);
  tmax = tmax.min(max_dz);

  tmax >= tmin
}

#[cfg(test)]
mod test{
    use crate::{math::{Vec3, MouseRay, Transforms}, ecs::world_resources::ScreenDimensions, view::camera::Camera, physics::collisions::{AABB3D, check_ray_aabb3d_collision}};

  #[test]
  fn check_collision(){
    let position:Vec3 = Vec3::new(0.0,0.0,0.0);
    let aabb = AABB3D::new(position,100.0, 5.0);

    let x = 1280.0 / 2.0;
    let y = 720.0 / 2.0;

    let screen_dimensions = ScreenDimensions::new(720, 1280);
    let camera = Camera::new();
    let transforms = Transforms::new(&screen_dimensions.aspect, &camera);

    let ray = MouseRay::new(x, y, &screen_dimensions, &transforms).0;

    let hit_check = check_ray_aabb3d_collision(aabb,ray);
    dbg!(hit_check);
  }
}