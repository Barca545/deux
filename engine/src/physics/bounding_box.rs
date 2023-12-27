use glm::vec3;

use crate::math::{math::Vec3, RayCast};

///A square shaped 2D AABB.
pub struct AABB2D {
  pub radius:f32,
  pub min:Vec3,
  pub max:Vec3
}

impl AABB2D {
  pub fn new(position:&Vec3, radius:f32) -> Self {
    let max:Vec3 = vec3(position.x - radius, 0.0, position.z - radius);
    let min:Vec3 = vec3(position.x + radius, 0.0, position.z + radius);

    AABB2D { radius, min, max }
  }

  pub fn update(&mut self, position:&Vec3) {
    self.min = vec3(position.x - self.radius, 0.0, position.z - self.radius);
    self.max = vec3(position.x + self.radius, 0.0, position.z - self.radius);
  }

  //this only checks for collision with a vector on the plane
  ///Returns true if the given ray intersects the AABB.
  ///
  ///Based on the equation found here: https://tavianator.com/2011/ray_box.html.
  ///
  ///Image illustrating the technique: https://www.researchgate.net/figure/The-slab-method-for-ray-intersection-detection-15_fig3_283515372
  pub fn check_ray_collision(&self, ray:&RayCast) -> bool {
    //I do not really understand why this is the calculation of the tx/tz values
    // let tx1 = (self.max.x - ray.origin.x) * ray.inverse_direction.x;
    // let tx2 = (self.max.x - ray.origin.x) * ray.inverse_direction.x;

    // let tmin = tx1.min(tx2);
    // let tmax = tx1.max(tx2);

    // let tz1 = (self.min.z - ray.origin.y) * ray.inverse_direction.y;
    // let tz2 = (self.max.z - ray.origin.y) * ray.inverse_direction.y;

    // let tmin = tmin.max(tz1.min(tz2));
    // let tmax = tmax.min(tz1.max(tz2));
    let t1 = (self.min.x - ray.origin.x) / ray.direction.x;
    let t2 = (self.max.x - ray.origin.x) / ray.direction.x;

    let xmin = t1.min(t2);
    let xmax = t1.max(t2);

    let t3 = (self.min.y - ray.origin.y) / ray.direction.y;
    let t4 = (self.max.y - ray.origin.y) / ray.direction.y;

    let ymin = t3.min(t4);
    let ymax = t3.max(t4);

    let t5 = (self.min.z - ray.origin.z) / ray.direction.z;
    let t6 = (self.max.z - ray.origin.z) / ray.direction.z;

    let zmin = t5.min(t6);
    let zmax = t5.max(t6);

    let tmin = (xmin.max(ymin)).max(zmin);
    let tmax = (xmax.min(ymax)).min(zmax);

    tmax >= tmin
  }

  //for 3d a collision occurs if the xmin,ymin,zmin > xmax,ymax,zmax
}

#[derive(Debug, Clone, Copy)]
pub struct AABB3D {
  pub height:f32,
  pub radius:f32,
  pub min:Vec3,
  pub max:Vec3
}

impl AABB3D {
  pub fn new(position:Vec3, height:f32, radius:f32) -> Self {
    let min:Vec3 = vec3(position.x - radius, height, position.z - radius);
    let max:Vec3 = vec3(position.x + radius, 0.0, position.z + radius);
    AABB3D { height, radius, min, max }
  }

  // pub fn update(&mut self, position:Vec3){
  //   self.min = vec3(position.x - self.radius, self.height, position.z -
  // self.radius);   self.max = vec3(position.x + self.radius, 0.0, position.z +
  // self.radius); }
}

#[cfg(test)]
mod test {
  use super::AABB2D;
  use crate::math::{RayCast, Vec3};

  #[test]
  fn check_collision() {
    let position:Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let aabb = AABB2D::new(&position, 5.0);

    let origin:Vec3 = Vec3::new(0.0, 0.0, -5.0);
    let end:Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let ray = RayCast::new(origin, end);

    let hit_check = aabb.check_ray_collision(&ray);
    dbg!(hit_check);
  }
}
