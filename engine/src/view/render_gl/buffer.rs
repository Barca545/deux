use gl;
use gl::{types::{GLuint,GLsizeiptr,GLvoid},Gl,ARRAY_BUFFER,ELEMENT_ARRAY_BUFFER};
use std::{mem::size_of,marker::PhantomData};

//const generics to rework this?
//https://blog.rust-lang.org/2021/02/26/const-generics-mvp-beta.html
pub trait BufferType{
  const BUFFER_TYPE:GLuint;
}

pub struct BufferTypeArray{}
impl BufferType for BufferTypeArray{
  const BUFFER_TYPE:GLuint = ARRAY_BUFFER;
}

pub struct BufferTypeElementArray{}
impl BufferType for BufferTypeElementArray{
  const BUFFER_TYPE:GLuint = ELEMENT_ARRAY_BUFFER;
}

pub struct Buffer<B> {
  gl: Gl,
  buffer_obj: GLuint,
  _marker: PhantomData<B>
}

impl <B>Buffer<B>where B: BufferType{
  pub fn new(gl:&Gl) -> Buffer<B> {
    let mut buffer_obj: GLuint = 0;
    unsafe{
      gl.GenBuffers(1,&mut buffer_obj)
    }

    Buffer { 
      gl: gl.clone(), 
      buffer_obj: buffer_obj, //can be a vbo or ebo
      _marker: PhantomData,
    }
  }

  pub fn bind(&self){
    unsafe{
      self.gl.BindBuffer(B::BUFFER_TYPE,self.buffer_obj);
    }
  }

  pub fn unbind(&self){
    unsafe{
      self.gl.BindBuffer(B::BUFFER_TYPE,0);
    }
  }

  pub fn static_draw_data<T>(&self, data: &[T]){
    unsafe{
      self.gl.BufferData(
        B::BUFFER_TYPE,
        (data.len() * size_of::<T>()) as GLsizeiptr,
        data.as_ptr() as *const GLvoid,
        gl::STATIC_DRAW
      )
    }
  }
}

impl<B> Drop for Buffer<B>{
  fn drop(&mut self) {
    unsafe{
      self.gl.DeleteBuffers(1, &mut self.buffer_obj)
    }
  }
}

pub type ArrayBuffer = Buffer<BufferTypeArray>;
pub type ElementArrayBuffer = Buffer<BufferTypeElementArray>;

/*
Yep, because of this additional gl field, 
this wrapper steps a bit outside of zero-cost claim. 
It may also be a problem if we tried to create millions of small array buffers
(which may also be another, bigger problem). 
Simplest way I can think of to remedy this would be to avoid all the lies 
and make another struct named ArrayBuffers, plural, 
which would always generate multiple buffers, 
but store one reference to Gl for all of them, and match OpenGL API 1:1:
*/

// pub struct ArrayBuffers {
//     gl: gl::Gl,
//     vbo: Vec<gl::types::GLuint>,
// }

pub struct VertexArray{
  gl: Gl,
  vao: GLuint,
}

impl VertexArray {
  pub fn new(gl:&Gl) -> VertexArray {
    let mut vao: GLuint = 0;
    unsafe{
      gl.GenVertexArrays(1,&mut vao);
    }
    VertexArray { 
      gl: gl.clone(), 
      vao: vao 
    }
  }

  pub fn bind(&self){
    unsafe{
      self.gl.BindVertexArray(self.vao);
    }
  }

  pub fn unbind(&self){
    unsafe{
      self.gl.BindVertexArray(0);
    }
  }
}

impl Drop for VertexArray{
  fn drop(&mut self) {
      unsafe{
        self.gl.DeleteVertexArrays(1,&mut self.vao);
      }
  }
}