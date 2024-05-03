use glm::{look_at as glm_look_at, TMat4, TVec2, TVec3, TVec4};
use nalgebra::Perspective3;
use std::{cmp::PartialOrd, f32::consts::PI};

pub type Vec2 = TVec2<f32>;
pub type Vec3 = TVec3<f32>;
pub type Vec4 = TVec4<f32>;
pub type Mat4 = TMat4<f32>;
pub type Point3 = Vec3;
pub type Point2 = Vec2;
pub type Perspective = Perspective3<f32>;
///Array represenation of a [`Mat4`] for GPU usage.
pub type FlatMat4 = [[f32; 4]; 4];

#[derive(Debug, Clone, Copy)]
pub struct Rect {
  pub min: Point2,
  pub max: Point2,
}

impl Rect {
  ///Create a new [`Rect`].
  pub fn new(width: f32, height: f32) -> Self {
    //Calculate the x and y maxima
    let x = width / 2.0;
    let y = height / 2.0;

    let min = Point2::new(-x, -y);
    let max = Point2::new(x, y);
    Rect { min, max }
  }
}

pub fn normalize(vec: Vec3) -> Vec3 {
  let x = vec.x / vec.x.abs();
  let y = vec.y / vec.y.abs();
  let z = vec.z / vec.z.abs();
  Vec3::new(x, y, z)
}

/// Build a look at view matrix based on the right handedness.
///
/// # Parameters:
///
/// * `eye` − Position of the camera.
/// * `center` − Position where the camera is looking at.
/// * `u` − Normalized up vector, how the camera is oriented. Typically `(0, 1, 0)`.
///
/// # See also:
///
/// * [`look_at_lh`](fn.look_at_lh.html)
/// * [`look_at_rh`](fn.look_at_rh.html)
pub fn look_at(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
  glm_look_at(&eye, &center, &up)
}

// pub fn look_at(right: Vector3<f32>, up: Vector3<f32>, direction: Vector3<f32>, position: Vector3<f32>) -> Matrix4<f32> {
//   Matrix4 {
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

pub fn radians(degrees: f32) -> f32 {
  degrees * PI / 180.0
}
//also need a viewmatrix https://medium.com/@carmencincotti/lets-look-at-magic-lookat-matrices-c77e53ebdf78

///Finds the maximum between two numbers. Works for both integers and floats.
pub fn max<N: PartialOrd>(a: N, b: N) -> N {
  if a >= b {
    a
  } else {
    b
  }
}

///Maps an `(x,y)` pixel value to a value in normalized device coordinates, range[-1,1].
pub fn to_ndc(x: f32, y: f32, width: i32, height: i32) -> (f32, f32) {
  let x = 2.0 * x as f32 / width as f32 - 1.0;
  let y = 1.0 - (2.0 * y as f32) / height as f32;
  (x, y)
}
