extern crate nalgebra_glm as glm;
use crate::math::{
  math::{look_at, radians, Mat4, Vec3},
  Transforms,
};

// Refactor:
// - Confirm if the OPENGL_TO_WGPU_MATRIX is needed
// - Move the transforms into a shared mod
// - Up doesn't change so I can just use (0.0, 1.0, 0.0)

const VIEW_ANGLE: f32 = 55.0;
const DEFAULT_Z_DIST: f32 = -10.0;

#[derive(Debug)]
pub struct Camera {
  pub position: Vec3,
  pub target: Vec3,
  pub up: Vec3,
  front: Vec3,
  pv_mat: Option<[[f32; 4]; 4]>,
}

impl Camera {
  ///Create a new [`Camera`].
  pub fn new() -> Self {
    // let x = 0.0;
    // let z = DEFAULT_Z_DIST;
    // let y = -z * radians(VIEW_ANGLE).tan();

    // let position: Vec3 = Vec3::new(x, y, z);

    //Delete
    let position: Vec3 = Vec3::new(0.0, 1.0, 2.0);
    let target: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let up: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    let front: Vec3 = Vec3::new(0.0, 1.0, 0.0);

    // let world_up: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    // let front: Vec3 = Vec3::new(-x, -y, -z);
    // let right: Vec3 = front.cross(&world_up).normalize();
    // let up: Vec3 = right.cross(&front).normalize();
    // let target: Vec3 = position + front;

    Camera {
      position,
      target,
      up,
      front,
      pv_mat: None,
    }
  }

  pub fn front(&self) -> Vec3 {
    self.front
  }

  pub fn view_mat(&self) -> Mat4 {
    look_at(self.position, self.target, self.up)
  }
  ///Updates the [`Camera`]'s `projection * view` matrix.
  pub fn update_pv(&mut self, transforms: &Transforms) {
    self.pv_mat = Some((transforms.proj_mat() * self.view_mat()).into())
  }

  ///Returns the [`Camera`]'s `projection * view` matrix.
  pub fn pv_mat(&self) -> [[f32; 4]; 4] {
    self.pv_mat.unwrap()
  }
}

impl Default for Camera {
  fn default() -> Self {
    Self::new()
  }
}

// #[derive(Debug)]
// pub struct Camera{
//   camera_position:TVec3<f32>,
//   camera_front:TVec3<f32>,
//   fov: f32,
//   camera_up:TVec3<f32>,
//   right:TVec3<f32>,
//   vertical_move:TVec3<f32>,
//   projection:TMat4<f32>,
//   horizontal_speed:f32,
//   vertical_speed:f32,
//   zoom_sensitivity:f32,
//   zoom_amount:i32,
//   max_zoom:i32,
//   min_zoom:i32,
// }

// impl Camera{
//   pub fn new(aspect:f32)->Self{
//     let fov = radians(0.0);

//     let projection:TMat4<f32> = perspective(aspect, fov, 0.1, 100.0);
//     let world_up:TVec3<f32> = vec3(0.0,1.0,0.0);

//     let z = 5.0;
//     let y = z*radians(55.0).tan();
//     let camera_front = vec3(
//       0.0,
//       -y,
//       -z
//     );

//     let camera_position:TVec3<f32> = vec3(
//       0.0,
//       y,
//       z
//     );

//     let right:TVec3<f32> = camera_front.cross(&world_up).normalize();
//     let camera_up:TVec3<f32> = right.cross(&camera_front).normalize();

//     let vertical_move:TVec3<f32> = vec3(0.0, 0.0, -1.0);

//     Camera {
//       camera_position,
//       camera_front,
//       fov,
//       camera_up,
//       right,
//       vertical_move,
//       projection,
//       horizontal_speed:0.1,
//       vertical_speed:0.1,
//       zoom_sensitivity:0.1,
//       zoom_amount:0,
//       max_zoom:5,
//       min_zoom:-5
//     }
//   }

//   pub fn get_camera_view(&self)->TMat4<f32>{
//     let center:TVec3<f32> = self.camera_position+self.camera_front;

//     let view:TMat4<f32> = look_at(
//       &self.camera_position,
//       &center,
//       &self.camera_up
//     );
//     view
//   }

//   pub fn update_projection(&mut self,aspect:f32){
//     //what does the FOV do here?
//     self.projection = perspective(aspect, self.fov, 0.1, 100.0);
//   }

//   //why does horizontal movement happen faster than vertical movement
//   pub fn new_position(&mut self,frame_inputs:&mut FrameInputs){
//     for input in frame_inputs.get_inputs(){
//       match input{
//         UserInputs::MoveCameraUp => {
//           self.camera_position += self.vertical_move*self.vertical_speed;
//         },
//         UserInputs::MoveCameraDown => {
//           self.camera_position -= self.vertical_move*self.vertical_speed;
//         },
//         UserInputs::MoveCameraRight => {
//           self.camera_position += self.right*self.horizontal_speed;
//         },
//         UserInputs::MoveCameraLeft => {
//           self.camera_position -= self.right*self.horizontal_speed;
//         },
//         UserInputs::ZoomInCamera => {
//           if self.zoom_amount < self.max_zoom{
//             self.zoom_amount+=1;
//             self.camera_position += self.camera_front*self.zoom_sensitivity;
//           }
//         },
//         UserInputs::ZoomOutCamera => {
//           if self.zoom_amount > self.min_zoom{
//             self.zoom_amount-=1;
//             self.camera_position -= self.camera_front*self.zoom_sensitivity;
//           }
//         },
//         UserInputs::CenterCamera => {
//           while self.zoom_amount>0 {
//             self.camera_position -= self.camera_front*self.zoom_sensitivity;
//             self.zoom_amount-=1;
//           }
//           while self.zoom_amount<0 {
//             self.camera_position += self.camera_front*self.zoom_sensitivity;
//             self.zoom_amount+=1;
//           }
//         },
//         _=>{}
//       }
//     }
//   }

//   //what I can do is have the update update some target position and then
// move it by the speed each frame   //I think the new position is just the
// delta time * the speed   //not sure I need to interpolate at all
//   //but check the article since it might explain why I need to
//   // pub fn update(&self,interpolation_factor:f64){
//   //   let interpolation_factor = interpolation_factor as f32;
//   // }
// }
