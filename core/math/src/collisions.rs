use super::bounding_box::AABB3D;
use crate::{math::Vec3, raycasting::Ray3D};

/// Compares the distance between the colliders' centers. If it is smaller than
/// the sum of their radii a collision has occured and the test will return
/// true.
pub fn ray_aabb3d_collision_test(aabb: AABB3D, ray: Ray3D,) -> bool {
  // Distance to xmin and xmax
  let min_dx = (aabb.min.x - ray.origin.x) * ray.inverse_direction().x;
  let max_dx = (aabb.max.x - ray.origin.x) * ray.inverse_direction().x;

  let mut tmin = min_dx.min(max_dx,);
  let mut tmax = min_dx.max(max_dx,);

  // Distance to ymin and ymax
  let min_dy = (aabb.min.y - ray.origin.y) * ray.inverse_direction().y;
  let max_dy = (aabb.max.y - ray.origin.y) * ray.inverse_direction().y;

  tmin = tmin.max(min_dy,);
  tmax = tmax.min(max_dy,);

  // Distance to zmin and zmax
  let min_dz = (aabb.min.z - ray.origin.z) * ray.inverse_direction().z;
  let max_dz = (aabb.max.z - ray.origin.z) * ray.inverse_direction().z;

  tmin = tmin.max(min_dz,);
  tmax = tmax.min(max_dz,);

  tmax >= tmin
}

pub fn ray_aabb2d_collision_test(aabb: AABB3D, ray: Ray3D,) -> bool {
  let tx1 = (aabb.max.x - ray.origin.x) * ray.inverse_direction().x;
  let tx2 = (aabb.max.x - ray.origin.x) * ray.inverse_direction().x;

  let tmin = tx1.min(tx2,);
  let tmax = tx1.max(tx2,);

  let tz1 = (aabb.min.z - ray.origin.y) * ray.inverse_direction().y;
  let tz2 = (aabb.max.z - ray.origin.y) * ray.inverse_direction().y;

  let tmin = tmin.max(tz1.min(tz2,),);
  let tmax = tmax.min(tz1.max(tz2,),);

  tmax >= tmin
}

/// Collision test between two AABB3Ds.
pub fn aabb3d_aabb3d_collision_test(aabb1: AABB3D, aabb2: AABB3D,) -> bool {
  aabb1.min.x <= aabb2.max.x && aabb1.max.x <= aabb2.min.x &&
  // && 
  // aabb1.min.y <= aabb2.max.y && aabb1.max.y <= aabb2.min.y
  // && 
  aabb1.min.z <= aabb2.max.z && aabb1.max.z <= aabb2.min.z
}

pub fn horizonal_collision_test(
  moving_velocity: Vec3,
  moving_object_aabb: AABB3D,
  test_object_aabb: AABB3D,
) -> bool {
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

/// Returns `true` if two circles have intersected.
pub fn intersection_circle_circle(p1: Vec3, r1: f32, p2: Vec3, r2: f32,) -> bool {
  let center_to_center = ((p1.x - p2.x).powi(2,) + (p1.z - p2.z).powi(2,)).sqrt();

  // If the cenger of the circles are closer than the combined radius of the
  // circles the have intersected
  center_to_center <= r1 + r2
}

fn collision_circle_circle(
  b1: BoundingCircle,
  v1: Vec3,
  b2: BoundingCircle,
  v2: Vec3,
) -> ColliderResult {
  // TODO: This derivation should be part of the docs somewhere

  // Capital letters indicate a vector.
  // A circle centered at `C` with radius `r` is the set of all points `P`
  // where `∥P − C∥² = r²`.
  //
  // A circle-circle collision can be simplified into a collision between a
  // circle with radius `r = r₁ + r₂` and a point.
  // Make the following substitutions:
  // - Position of the point: `P = V₁t + P₁`.
  // - Position of the circle's center: `C = V₂t + P₂`.
  //
  // The resulting equation `∥V₁t + P₁ − V₂t + P₂∥² = (r₁ + r₂)² = r²` can be
  // solved to find the time `t` where the point is first on the circle's
  // border.
  //
  // For ease of calculation make the following substitutions:
  // - `M = V₁ - V₂`
  // - `K = P₁ - P₂`
  // - `r = r₁ + r₂`
  //
  // The resulting equation `∥Mt + K∥² = r²` is easier to reason about.
  //
  // Expanding the magnitude yields:`(tMₓ + Kₓ)² + (tM_z + K_z)² = r²`
  //
  // As a shortcut we can now take the derivative;
  // the derivative's first critical point indicates where the distance between
  // the point and circle is 0.
  //
  // This yields: `2Mₓ(tMₓ + Kₓ) + 2M_z(tM_z + K_z) = 0`
  //
  // yields: `2t(Mₓ)² + 2MₓKₓ + 2t(M_z)² + 2M_zK_z = 0`
  //
  // yields: `2t(Mₓ² + M_z²) = -2(MₓKₓ + M_zK_z)`
  //
  // yields: `t = -(MₓKₓ + M_zK_z)/(Mₓ² + M_z²)`
  //
  // if `Mₓ² + M_z² = 0`, no intersection occurs.
  //
  // if t > 1/60, no intersection occurs.
  //
  // Otherwise intersection occurs at:
  // `V₁t + P₁ = V₁[-(MₓKₓ + M_zK_z)/(Mₓ² + M_z²)] + P₁`
  //
  // Which expands to:
  // V₁[-(V₁ₓ - V₂ₓ)(P₁ₓ - P₂ₓ) + (V₁_z - V₂_z)(P₁_z - P₂_z)/2((V₁ₓ - V₂ₓ)² +
  // (V₁_z - V₂_z)²)] + P₁
  //
  // Time `t` is only when the circles are closest, they may not necessarily
  // collide

  // If the points are already overlapping, return early
  if intersection_circle_circle(b1.pos, b1.radius, b2.pos, b2.radius,) {
    return ColliderResult::AlreadyOverlapping;
  }

  let m = v1 - v2;
  let k = b1.pos - b2.pos;

  let det = m.x.powi(2,) + m.z.powi(2,);

  // TODO: Is this the determinant or discriminant or something else entirely?
  // TODO: This can be modified, if the dots are parallel and not on the same line
  // if det == 0.0 {
  //   return ColliderResult::Parallel;
  // }

  let t = -(m.x * k.x + m.z * k.z) / det;

  // TODO: I don't love hardcoding this check's value
  if t > 1.0 / 60.0 {
    return ColliderResult::Late;
  }

  // Center of the two circles at time of "intersection"
  let c_t_1 = v1 * t + b1.pos;
  let c_t_2 = v2 * t + b2.pos;

  let m = v1 - v2;

  // Filter out false positives
  if !intersection_circle_circle(c_t_1, b1.radius, c_t_2, b2.radius,) {
    return ColliderResult::Missed;
  }

  ColliderResult::Success(c_t_1,)
}

pub struct Collision {
  result: ColliderResult,
  // TODO: Maybe I can store refrences to the colliders here instead of copying the type?
  bodies: [ColliderType; 2],
}

impl Collision {
  /// Returns `true` if both bodies in the collision are both
  /// [`ColliderType::Blocker`]s.
  pub fn blocking(&self,) -> bool {
    self.bodies == [ColliderType::Blocker; 2]
  }

  pub fn result(&self,) -> &ColliderResult {
    &self.result
  }
}

#[derive(Debug, Default, Clone, Copy, PartialEq,)]
enum ColliderType {
  Blocker,
  #[default]
  Sensor,
}

impl ColliderType {
  pub fn is_blocker(&self,) -> bool {
    *self == ColliderType::Blocker
  }
}

#[derive(Debug, PartialEq,)]
pub enum ColliderResult {
  /// A collision point was successfully found for the two colliders.
  Success(Vec3,),
  /// The two colliders were already overlapping at the begining of the frame.
  AlreadyOverlapping,
  /// The two colliders' paths overlap but they do not collide.
  Missed,
  /// The two colliders have parallel paths and will not collide.
  Parallel,
  /// The two colliders hit after the frame would conclude.
  Late,
}

pub struct BoundingCircle {
  /// The radius of the bounding circle.
  radius: f32,
  /// The type of collider.
  ty: ColliderType,
  // TODO: One optimization is I could make all the position logic 2D and then just have a separate
  // height component since height is purely visual and has no link to the game logic.
  /// The position of the collider in space
  pos: Vec3,
}

#[cfg(test)]
mod test {
  use super::collision_circle_circle;
  use crate::{
    collisions::{BoundingCircle, ColliderResult, ColliderType},
    Vec3,
  };

  #[test]
  fn circle_circle_collison() {
    // Overlapping points
    let b1 = BoundingCircle {
      radius: 0.5,
      ty: ColliderType::Blocker,
      pos: Vec3::new(0.0, 0.0, 0.0,),
    };
    let v1 = Vec3::new(0.0, 0.0, 0.0,);

    let b2 = BoundingCircle {
      radius: 0.5,
      ty: ColliderType::Blocker,
      pos: Vec3::new(0.0, 0.0, 0.0,),
    };
    let v2 = Vec3::new(100.0, 0.0, 0.0,);

    assert_eq!(
      collision_circle_circle(b1, v1, b2, v2,),
      ColliderResult::AlreadyOverlapping
    );

    // Head on collision
    let b1 = BoundingCircle {
      radius: 0.5,
      ty: ColliderType::Blocker,
      pos: Vec3::new(-1.0, 0.0, 0.0,),
    };
    let v1 = Vec3::new(0.0, 0.0, 0.0,);

    let b2 = BoundingCircle {
      radius: 0.1,
      ty: ColliderType::Blocker,
      pos: Vec3::new(0.0, 0.0, 0.0,),
    };
    let v2 = Vec3::new(-1000.0, 0.0, 0.0,);

    assert_eq!(
      collision_circle_circle(b1, v1, b2, v2,),
      ColliderResult::Success(Vec3::new(-1.0, 0.0, 0.0))
    );

    // Overlapping paths but colliders do not hit
    let b1 = BoundingCircle {
      radius: 1.0,
      ty: ColliderType::Blocker,
      pos: Vec3::new(0.0, -1.0, 0.0,),
    };
    let v1 = Vec3::new(0.0, 1000.0, 0.0,);

    let b2 = BoundingCircle {
      radius: 1.0,
      ty: ColliderType::Blocker,
      pos: Vec3::new(14.0, 0.0, 0.0,),
    };
    let v2 = Vec3::new(0.0, 1.0, 0.0,);

    assert_eq!(
      collision_circle_circle(b1, v1, b2, v2,),
      ColliderResult::Missed,
    );

    // Collide after the frame
    let b1 = BoundingCircle {
      radius: 1.0,
      ty: ColliderType::Blocker,
      pos: Vec3::new(0.0, -1.0, 0.0,),
    };
    let v1 = Vec3::new(1.0, 0.0, 0.0,);

    let b2 = BoundingCircle {
      radius: 1.0,
      ty: ColliderType::Blocker,
      pos: Vec3::new(14.0, 0.0, 0.0,),
    };
    let v2 = Vec3::new(0.0, 0.0, 0.0,);

    assert_eq!(
      collision_circle_circle(b1, v1, b2, v2,),
      ColliderResult::Late,
    );
  }
}
