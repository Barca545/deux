use std::f32::consts::PI;

use cgmath::{Vector3, Matrix4, Vector4};

use super::data::F32Tuple3;

//When multiplying matrices the right-most matrix is first multiplied with the vector so you should read the multiplications from right to left.

enum Axis{
  x,
  y,
  z,
}


//should the trait be Vector or Point? Avoiding this to avoid confusion for the moment.
fn translate(position:F32Tuple3,x:f32,y:f32,z:f32)->F32Tuple3{
  F32Tuple3{
    d0: position.d0 + x, 
    d1: position.d1 + y, 
    d2: position.d2 + z
  }
}

fn scale(position:F32Tuple3,x:f32,y:f32,z:f32)->F32Tuple3{    
  F32Tuple3{
    d0: position.d0 * x, 
    d1: position.d1 * y, 
    d2: position.d2 * z
  }
}

//consider quaternions for rotation
fn rotate(position:F32Tuple3,axis:Axis,degrees:f32)->F32Tuple3{    
  match axis {
    Axis::x => {
      F32Tuple3{
        d0: position.d0, 
        d1: degrees.cos()*position.d1-degrees.sin()*position.d2, 
        d2: degrees.sin()*position.d1+degrees.cos()*position.d2, 
      }
    },
    Axis::y => {
      F32Tuple3{
        d0: degrees.cos()*position.d0+degrees.sin()*position.d2, 
        d1: position.d1, 
        d2: -degrees.sin()*position.d0+degrees.cos()*position.d2, 
      }
    },
    Axis::z => {
      F32Tuple3{
        d0: degrees.cos()*position.d0-degrees.sin()*position.d1, 
        d1: degrees.sin()*position.d0+degrees.cos()*position.d1, 
        d2: position.d2, 
      }
    }
  }
}

pub fn normalize(vec:Vector3<f32>)->Vector3<f32>{
  let x = vec.x/vec.x.abs();
  let y = vec.y/vec.y.abs();
  let z = vec.z/vec.z.abs();
  Vector3{
    x,
    y,
    z
  }
}

pub fn look_at(right:Vector3<f32>,up:Vector3<f32>,direction:Vector3<f32>,position:Vector3<f32>)->Matrix4<f32>{
  Matrix4{
    x:Vector4{
      x:right.x,
      y:right.y,
      z:right.z,
      w:position.x,
    },
    y:Vector4{
      x:up.x,
      y:up.y,
      z:up.z,
      w:position.y,
    },
    z:Vector4{
      x:direction.x,
      y:direction.y,
      z:direction.z,
      w:position.z,
    },
    w:Vector4{
      x:0.0,
      y:0.0,
      z:0.0,
      w:1.0,
    }
  }
}

pub fn radians(degrees:f32)->f32{
  degrees*PI/180.0
}
//also need a viewmatrix https://medium.com/@carmencincotti/lets-look-at-magic-lookat-matrices-c77e53ebdf78