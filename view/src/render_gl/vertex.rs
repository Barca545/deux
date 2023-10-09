use super::data::{F32Tuple3, F32Tuple2};
use gl::{Gl, types::{GLuint, GLint, GLvoid}, FALSE, FLOAT};
use std::mem::size_of;


#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct TexturedVertex{
  pos:F32Tuple3,
  clr:F32Tuple3,
  txt: F32Tuple2
}

impl Vertex for TexturedVertex{}
impl From<(f32,f32,f32,f32,f32,f32,f32,f32)> for TexturedVertex{
  fn from(value: (f32,f32,f32,f32,f32,f32,f32,f32)) -> Self {
      let pos = (value.0,value.1,value.2);
      let clr = (value.3,value.4,value.5);
      let txt = (value.6,value.7);
      Self::new(pos, clr, txt)
  }
}
impl TexturedVertex{
  pub fn new(pos:(f32,f32,f32),clr:(f32,f32,f32),txt:(f32,f32))->Self{
    dbg!(F32Tuple2::from(txt));
    TexturedVertex{
      pos: F32Tuple3::from(pos),
      clr: F32Tuple3::from(clr),
      txt: F32Tuple2::from(txt)
    }
  }
  
  pub fn vertex_attrib_pointers(gl:&Gl){
    let stride = size_of::<Self>();
    let size = size_of::<F32Tuple3>();
    
    //shape
    let position = 0; //is location position or location?
    let position_offset = 0;
    unsafe {
      Self::define_vertex_attrib_pointer(gl, stride, position, position_offset,3);
    }

    //color
    let color = 1;
    let color_offset = position_offset + size;
    unsafe {
      Self::define_vertex_attrib_pointer(gl, stride, color, color_offset,3);
    }

    //texture
    let texture = 2;
    let texture_offset = color_offset + size;
    unsafe {
      Self::define_vertex_attrib_pointer(gl, stride, texture, texture_offset,2);
    }
  }
}


#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct UntexturedVertex{
  pos:F32Tuple3,
  clr:F32Tuple3,
}

impl Vertex for UntexturedVertex {}
impl UntexturedVertex{
  pub fn new(pos:F32Tuple3,clr:F32Tuple3,txt:F32Tuple3)->Self{
    UntexturedVertex{
      pos,
      clr,
    }
  }
   
  pub fn vertex_attrib_pointers(gl:&Gl){
    let stride = size_of::<Self>();
    let size = size_of::<F32Tuple3>();
    
    let location = 0; //is location position?
    let location_offset = 0;
    
    //shape
    unsafe {
      Self::define_vertex_attrib_pointer(gl, stride, location, location_offset,3);
    }

    let color = 1;
    let color_offset = location_offset + size;
    
    //color
    unsafe {
      Self::define_vertex_attrib_pointer(gl, stride, color, color_offset,3);
    }
  }
}

//make a derive for this?
//maybe eventually move new here once I figure out trait obj
trait Vertex {
  unsafe fn define_vertex_attrib_pointer(gl:&Gl, stride:usize, location:usize, offset:usize,tuple_size:GLint){
    gl.EnableVertexAttribArray(location as GLuint);
    gl.VertexAttribPointer(
      location as GLuint,
      tuple_size as GLint, 
      FLOAT,
      FALSE, 
      stride as GLint,
      offset as *const GLvoid
    );
  }
}