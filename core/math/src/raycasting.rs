use super::math::Vec3;
use nalgebra_glm::vec3;

#[derive(Debug, Default, Clone, Copy,)]
pub struct RayCast {
  pub(crate) origin:Vec3,
  pub(crate) direction:Vec3,
}

impl RayCast {
  pub fn new(origin:Vec3, end:Vec3,) -> RayCast {
    let direction:Vec3 = (end - origin).normalize();
    RayCast { origin, direction, }
  }

  pub fn inverse_direction(&self,) -> Vec3 {
    vec3(
      1.0 / self.direction.x,
      1.0 / self.direction.y,
      1.0 / self.direction.z,
    )
  }

  /// Calculates the point of intersection between a `RayCast` in world
  /// coordinates and a given plane.
  ///
  /// Concept based on the equation outlined on [Scratchpixel](https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-plane-and-ray-disk-intersection.html).
  ///
  /// Code pulled from [Rosetta Code](https://rosettacode.org/wiki/Find_the_intersection_of_a_line_with_a_plane#Rust).
  ///
  /// The negative signs are different between the two tutorials.
  pub fn ray_plane_intersection(&self, plane_normal:Vec3, plane_origin:Vec3,) -> Vec3 {
    //checks for the distance where the ray has a point on the plane
    let numerator = (plane_origin - self.origin).dot(&plane_normal,);
    let denominator = self.direction.dot(&plane_normal,);
    let distance = numerator / denominator;

    //scale is the same as multiplying by distance so benchmark which is faster
    let intersection_point:Vec3 = self.origin + self.direction.scale(distance,);
    intersection_point
  }

  /// Calculates the point of intersection between a ray cast in world
  /// coordinates and the ground.
  ///
  /// Concept based on the equation outlined on [Scratchpixel](https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-plane-and-ray-disk-intersection.html).
  ///
  /// Code pulled from [Rosetta Code](https://rosettacode.org/wiki/Find_the_intersection_of_a_line_with_a_plane#Rust).
  ///
  /// The negative signs are different between the two tutorials.
  pub fn ray_ground_intersection(&self,) -> Vec3 {
    //I think the normal to plane xz is this
    let plane_normal:Vec3 = vec3(0.0, 1.0, 0.0,);
    //this is "plane_point" in the tutorial. I think it can just be any point on
    // the plane?
    let plane_origin:Vec3 = vec3(0.0, 0.0, 0.0,);

    //checks for the distance where the ray has a point on the plane
    let numerator = (plane_origin - self.origin).dot(&plane_normal,);
    let denominator = self.direction.dot(&plane_normal,);
    let distance = numerator / denominator;

    //scale is the same as multiplying by distance so benchmark which is faster
    let mut intersection_point:Vec3 = self.origin + self.direction.scale(distance,);
    intersection_point.y = 0.0;
    intersection_point
  }
}
