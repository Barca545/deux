use glm::vec3;

use crate::math::{math::Vec3, RayCast};

///A square shaped 2D AABB.
pub struct AABB2D{
  radius:f32,
  min:Vec3,
  max:Vec3,
}

impl AABB2D{
  pub fn new(position:&Vec3,radius:f32)->Self{
    let min:Vec3 = vec3(position.x - radius, 0.0, position.z - radius);
    let max:Vec3 = vec3(position.x + radius, 0.0, position.z - radius);
    
    AABB2D { 
      radius,
      min,
      max
    }
  }

  pub fn update(&mut self,position:&Vec3){
    self.min = vec3(position.x - self.radius, 0.0, position.z - self.radius);
    self.max= vec3(position.x + self.radius, 0.0, position.z - self.radius);
  }
  
  ///Returns true if the given ray intersects the AABB.
  /// 
  ///Based on the equation found here: https://tavianator.com/2011/ray_box.html.
  /// 
  ///Image illustrating the technique: https://www.researchgate.net/figure/The-slab-method-for-ray-intersection-detection-15_fig3_283515372
  pub fn check_2d_ray_collision(&self,ray:&RayCast)->bool{
    //I do not really understand why this is the calculation of the tx/tz values
    let tx1 = (self.max.x - ray.origin.x)*ray.inverse_direction.x;
    let tx2 = (self.max.x - ray.origin.x)*ray.inverse_direction.x;
  
    let tmin = tx1.min(tx2);
    let tmax = tx1.max(tx2);

    let tz1 = (self.min.z - ray.origin.y)*ray.inverse_direction.y;
    let tz2 = (self.max.z - ray.origin.y)*ray.inverse_direction.y;

    let tmin = tmin.max(tz1.min(tz2));
    let tmax = tmax.min(tz1.max(tz2));

    tmax >= tmin
  }

  //for 3d a collision occurs if the xmin,ymin,zmin > xmax,ymax,zmax
}