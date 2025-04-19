use crate::number::Number;
use nalgebra::Perspective3;
use nalgebra_glm::{TMat4, TVec2, TVec3, TVec4};

pub type Vec2 = TVec2<f32,>;
pub type Vec3 = TVec3<f32,>;
pub type Vec4 = TVec4<f32,>;
pub type Mat4 = TMat4<f32,>;
pub type Point3 = Vec3;
pub type Point2 = Vec2;
pub type Perspective = Perspective3<f32,>;

/// Array represenation of a [`Mat4`] for GPU usage.
pub type FlatMat4 = [[f32; 4]; 4];

#[derive(Debug, Clone, Copy,)]
pub struct Rect {
  pub min: Point2,
  pub max: Point2,
}

impl Rect {
  /// Create a new [`Rect`].
  pub fn new(width: f32, height: f32,) -> Self {
    // Calculate the x and y maxima
    let x = width / 2.0;
    let y = height / 2.0;

    let min = Point2::new(-x, -y,);
    let max = Point2::new(x, y,);
    Rect { min, max, }
  }
}

pub fn normalize(vec: Vec3,) -> Vec3 {
  let x = vec.x / vec.x.abs();
  let y = vec.y / vec.y.abs();
  let z = vec.z / vec.z.abs();
  Vec3::new(x, y, z,)
}

// TODO: Implement a viewmatrix myself https://medium.com/@carmencincotti/lets-look-at-magic-lookat-matrices-c77e53ebdf78
// pub fn look_at(right: Vector3<f32>, up: Vector3<f32>, direction:
// Vector3<f32>, position: Vector3<f32>) -> Matrix4<f32> {   Matrix4 {
//     x: Vector4 {
//       x: right.x,
//       y: right.y,
//       z: right.z,
//       w: position.x,
//     },
//     y: Vector4 {
//       x: up.x,
//       y: up.y,
//       z: up.z,
//       w: position.y,
//     },
//     z: Vector4 {
//       x: direction.x,
//       y: direction.y,
//       z: direction.z,
//       w: position.z,
//     },
//     w: Vector4 {
//       x: 0.0,
//       y: 0.0,
//       z: 0.0,
//       w: 1.0,
//     },
//   }
// }

/// Build a look at view matrix based on the right handedness.
///
/// # Parameters:
/// - `eye` − Position of the camera.
/// - `center` − Position where the camera is looking at.
/// - `u` − Normalized up vector, how the camera is oriented. Typically `(0, 1,
///   0)`.
pub fn look_at(eye: Vec3, center: Vec3, up: Vec3,) -> Mat4 {
  nalgebra_glm::look_at(&eye, &center, &up,)
}

/// Builds a scale 4 * 4 matrix created from 3 scalars and right-multiply it to
/// `m`.
///
/// # Parameters:
/// - `m` − Input matrix multiplied by this scale matrix.
/// - `v` − Ratio of scaling for each axis.
pub fn scale(m: &Mat4, v: &Vec3,) -> Mat4 {
  nalgebra_glm::scale(m, v,)
}

/// Builds a translation 4 * 4 matrix created from a vector of 3 components and
/// right-multiply it to `m`.
///
/// # Parameters:
///
/// - `m` − Input matrix multiplied by this translation matrix.
/// - `v` − Coordinates of a translation vector.
pub fn translate(m: &Mat4, v: &Vec3,) -> Mat4 {
  nalgebra_glm::translate(m, v,)
}

/// A 4 * 4 identity matrix.
pub fn identity() -> Mat4 {
  nalgebra_glm::identity()
}

/// Finds the maximum between two numbers.
pub fn max<N: Number,>(a: N, b: N,) -> N {
  match a >= b {
    true => a,
    false => b,
  }
}

/// Maps an `(x,y)` pixel value to a value in [normalized device coordinates](https://learnopengl.com/Getting-started/Coordinate-Systems).
pub fn convert_screen_coords_to_ndc(x: f32, y: f32, width: i32, height: i32,) -> (f32, f32,) {
  let x = 2.0 * x / width as f32 - 1.0; // range [-1,1]
  let y = 1.0 - (2.0 * y) / height as f32; // range [-1,1]
  (x, y,)
}

// TODO: Needs real documentation
pub fn interpolate(a: Vec3, b: Vec3, factor: f32,) -> Vec3 {
  // TODO: This does not seem like the correct condition
  if b.x == a.x && b.y == a.y && b.z == a.z {
    nalgebra_glm::lerp(&b, &a, factor as f32,)
  } else {
    a
  }
}

// - Updated `Renderer` struct
// - Removed `update` method as its logic overlaped with render
// From here:https://gamedev.stackexchange.com/questions/18615/how-do-i-linearly-interpolate-between-two-vectors
fn lerp(a: Vec3, b: Vec3, factor: f32,) -> Vec3 {
  // TODO: Confirm this is correct
  a * factor + (1.0 - factor) * b
}
