use gl;
use gl::{
  types::{GLsizeiptr, GLuint, GLvoid, GLenum},
  Gl, ARRAY_BUFFER, ELEMENT_ARRAY_BUFFER
};
use std::{marker::PhantomData, mem::size_of};

//const generics to rework this?
//https://blog.rust-lang.org/2021/02/26/const-generics-mvp-beta.html
pub trait BufferType {
  const BUFFER_TYPE:GLuint;
}

#[derive(Debug, Clone, Copy)]
pub struct Array;
impl BufferType for Array {
  const BUFFER_TYPE:GLuint = ARRAY_BUFFER;
}
#[derive(Debug, Clone, Copy)]
pub struct ElementArray;
impl BufferType for ElementArray {
  const BUFFER_TYPE:GLuint = ELEMENT_ARRAY_BUFFER;
}

#[derive(Debug, Clone, Copy)]
pub struct Buffer<B> {
  // gl:Gl,
  pub buffer_obj:GLuint,
  _marker:PhantomData<B>
}

impl<B> Buffer<B>
where B: BufferType
{
  pub fn new(gl:&Gl) -> Buffer<B> {
    let mut buffer_obj:GLuint = 0;
    unsafe { gl.GenBuffers(1, &mut buffer_obj) }

    Buffer {
      // gl:gl.clone(),
      buffer_obj, //can be a vbo or ebo
      _marker:PhantomData
    }
  }

  pub fn bind(&self,gl:&Gl) {
    unsafe {gl.BindBuffer(B::BUFFER_TYPE, self.buffer_obj)}
    // unsafe {self.gl.BindBuffer(B::BUFFER_TYPE, self.buffer_obj)}
  }

  pub fn unbind(&self,gl:&Gl) {
    // unsafe {self.gl.BindBuffer(B::BUFFER_TYPE, 0)}
    unsafe {gl.BindBuffer(B::BUFFER_TYPE, 0)}
  }

  pub fn buffer_data<T>(&self, gl:&Gl, data:&[T], usage:GLenum) {
    unsafe {
      gl.BufferData(
        B::BUFFER_TYPE,
        (data.len() * size_of::<T>()) as GLsizeiptr,
        data.as_ptr() as *const GLvoid,
        usage
      )
    }
    
    // unsafe {
    //   self.gl.BufferData(
    //     B::BUFFER_TYPE,
    //     (data.len() * size_of::<T>()) as GLsizeiptr,
    //     data.as_ptr() as *const GLvoid,
    //     usage
    //   )
    // }
  }
}

// impl<B> Drop for Buffer<B> {
//   fn drop(&mut self) {
//     unsafe { self.gl.DeleteBuffers(1, &mut self.buffer_obj) }
//   }
// }

pub type ArrayBuffer = Buffer<Array>;
pub type ElementArrayBuffer = Buffer<ElementArray>;

/*
Yep, because of this additional gl field,
this wrapper steps a bit outside of zero-cost claim.
It may also be a problem if we tried to create millions of small array buffers
(which may also be another, bigger problem).
Simplest way I can think of to remedy this would be to avoid all the lies
and make another struct named ArrayBuffers, plural,
which would always generate multiple buffers,
but store one reference to Gl for all of them, and match OpenGL API 1:1:

can use a hashmap like with the loader to make this work
*/

// pub struct ArrayBuffers {
//     gl: gl::Gl,
      //I think 
//     vbo: Vec<gl::types::GLuint>,
// }
#[derive(Debug, Clone)]
pub struct VertexArray {
  // gl:Gl,
  vao:GLuint
}

impl VertexArray {
  pub fn new(gl:&Gl) -> VertexArray {
    let mut vao:GLuint = 0;
    unsafe {gl.GenVertexArrays(1, &mut vao)}
    VertexArray {
      // gl:gl.clone(), 
      vao 
    }
  }

  pub fn bind(&self, gl:&Gl) {
    unsafe {gl.BindVertexArray(self.vao)}
    // unsafe {self.gl.BindVertexArray(self.vao)}
  }

  pub fn unbind(&self, gl:&Gl) {
    unsafe {gl.BindVertexArray(0)}
    // unsafe {self.gl.BindVertexArray(0)}
  }
}

// impl Drop for VertexArray {
//   fn drop(&mut self) {
//     unsafe {
//       self.gl.DeleteVertexArrays(1, &mut self.vao);
//     }
//   }
// }
