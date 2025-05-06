use crate::{Vec2, Vec3};

// Refactor:
//  - If I end up using more than one type of ray make a Ray trait.

#[derive(Debug, Default, Clone, Copy,)]
pub struct Ray3D {
  pub(crate) origin: Vec3,
  pub(crate) direction: Vec3,
}

impl Ray3D {
  // Create a new `Ray3D`
  pub fn new(origin: Vec3, end: Vec3,) -> Ray3D {
    let direction: Vec3 = (end - origin).normalize();
    Ray3D { origin, direction, }
  }

  pub fn inverse_direction(&self,) -> Vec3 {
    Vec3::new(
      1.0 / self.direction.x,
      1.0 / self.direction.y,
      1.0 / self.direction.z,
    )
  }

  /// Calculates the point of intersection between a [`Ray3D`] in world
  /// coordinates and a given plane.
  ///
  /// Concept based on the equation outlined on [Scratchpixel](https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-plane-and-ray-disk-intersection.html).
  ///
  /// Code pulled from [Rosetta Code](https://rosettacode.org/wiki/Find_the_intersection_of_a_line_with_a_plane#Rust).
  ///
  /// The negative signs are different between the two tutorials.
  pub fn ray_plane_intersection(&self, plane_normal: Vec3, plane_origin: Vec3,) -> Vec3 {
    // Checks for the distance where the ray has a point on the plane
    let numerator = (plane_origin - self.origin).dot(&plane_normal,);
    let denominator = self.direction.dot(&plane_normal,);
    let distance = numerator / denominator;

    // Scale is the same as multiplying by distance so benchmark which is faster
    let intersection_point: Vec3 = self.origin + self.direction.scale(distance,);
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
    let plane_normal: Vec3 = Vec3::new(0.0, 1.0, 0.0,);
    //this is "plane_point" in the tutorial. I think it can just be any point on
    // the plane?
    let plane_origin: Vec3 = Vec3::new(0.0, 0.0, 0.0,);

    //checks for the distance where the ray has a point on the plane
    let numerator = (plane_origin - self.origin).dot(&plane_normal,);
    let denominator = self.direction.dot(&plane_normal,);
    let distance = numerator / denominator;

    //scale is the same as multiplying by distance so benchmark which is faster
    let mut intersection_point: Vec3 = self.origin + self.direction.scale(distance,);
    intersection_point.y = 0.0;
    intersection_point
  }
}

/// A 2D Ray3D.
pub struct Ray2D {
  origin: Vec2,
  direction: Vec2,
}

impl Ray2D {
  pub fn new(origin: Vec2, direction: Vec2,) -> Self {
    Ray2D { origin, direction, }
  }
}
