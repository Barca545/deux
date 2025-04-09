use math::{identity, look_at, scale, translate, FlatMat4, Mat4, Perspective, Vec3};

// Refactor:
// - Confirm if the OPENGL_TO_WGPU_MATRIX is needed is there a way to bake it's
//   info in instead of requiring math happen
// - FlatMat4 should be a new struct of some sort instead of a type alias. Must
//   be reprc packed

/// View angle of the [`Camera`].
const VIEW_ANGLE:f32 = 55.0;
/// Spawn position of the [`Camera`] on the `x` axis.
const DEFAULT_X_DIST:f32 = 0.0;
/// Spawn position of the [`Camera`] on the `z` axis.
const DEFAULT_Z_DIST:f32 = -15.0;
const DEFAULT_FOVY:f32 = 45.0;
const Z_NEAR:f32 = 0.1;
const Z_FAR:f32 = 100.0;

#[rustfmt::skip]
/// Matrix for converting the OpenGL coordinate system to the conventions wgpu uses.
const OPENGL_TO_WGPU_MATRIX:Mat4 = Mat4::new(   
  1.0, 0.0, 0.0, 0.0,
  0.0, 1.0, 0.0, 0.0,
  0.0, 0.0, 0.5, 0.5,
  0.0, 0.0, 0.0, 1.0,
);

#[derive(Debug,)]
pub struct Camera {
  /// Location in space the camera is looking from.
  pub position:Vec3,
  /// Point the target is looking at.
  pub target:Vec3,
  pub camera_up:Vec3,
  front:Vec3,
  aspect:f32,
  fovy:f32,
  znear:f32,
  zfar:f32,
}

impl Camera {
  /// Create a new [`Camera`].
  pub fn new(width:f32, height:f32,) -> Self {
    let x = DEFAULT_X_DIST;
    let z = DEFAULT_Z_DIST;
    let y = -z * VIEW_ANGLE.to_radians().tan();

    let position:Vec3 = Vec3::new(x, y, z,);
    let front:Vec3 = Vec3::new(-x, -y, -z,);
    // Normally: right = front.cross(&WORLD_UP,).normalize();
    // Cross product is (AyBz - AzBy)i + (AxBz - AzBx)j + (AxBy - AyBx)k
    // However WORLD_UP is (0, 1, 0) so right = front x WORLD_UP = (-z, 0, -x)
    // Normally: camera_up:Vec3 = right.cross(&front,).normalize();
    // However right is (z, 0.0, -x)
    // Thus right x front is (-AzBy)i + (AxBz - AzBx)j + (AxBy)
    // Given:
    // Ax = z   Bx = -x
    // Ay = 0   By = -y
    // Az = -x  Bz = -z
    // right x front =  (-xy, zz - xx, -zy)
    let camera_up:Vec3 = Vec3::new(-(-x * -y), z * -z - (-x * -x), z * -y,).normalize();
    let target:Vec3 = position + front;

    Camera {
      position,
      target,
      camera_up,
      front,
      aspect:width / height,
      fovy:DEFAULT_FOVY,
      znear:Z_NEAR,
      zfar:Z_FAR,
    }
  }

  pub fn front(&self,) -> Vec3 {
    self.front
  }

  // Returns the [`Camera`]'s `view` matrix.
  #[inline(always)]
  pub fn view_mat(&self,) -> Mat4 {
    look_at(self.position, self.target, self.camera_up,)
  }

  /// Returns the [`Camera`]'s `projection * view` matrix as a [`FlatMat4`].
  pub fn pv_mat(&self,) -> FlatMat4 {
    // This is the projection matrix
    (OPENGL_TO_WGPU_MATRIX
      * Perspective::new(self.aspect, self.fovy, self.znear, self.zfar,).as_matrix()
    // This is the view matrix  
      * self.view_mat())
    .into()
  }

  /// Offset the [`Camera`]'s `position` by the `x` and `y` of the the position
  /// vector. Set the `Camera`'s `target` to the provided `position`.
  pub fn offset_camera_relative_to_position(&mut self, pos:Vec3,) {
    // Move the camera so it keeps the same relationship to the new target as it had
    // before moving
    self.position = Vec3::new(pos.x, self.position.y, pos.z + DEFAULT_Z_DIST,);

    // Update the target
    self.target = pos + self.front;
  }
}

pub fn calculate_model_transform(position:&Vec3, scale_factor:f32,) -> Mat4 {
  let model:Mat4 = identity();
  let model:Mat4 = translate(&model, position,);
  let model:Mat4 = scale(&model, &Vec3::from([scale_factor; 3],),);
  model
}
