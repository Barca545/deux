use std::{ptr, ffi::c_void};

use gl::{types::{GLuint, GLint}, Gl, TEXTURE_2D, RGB32I, RGB_INTEGER, UNSIGNED_INT, TEXTURE_MIN_FILTER, NEAREST, FRAMEBUFFER, TEXTURE_MAG_FILTER, COLOR_ATTACHMENT0, DEPTH_COMPONENT, FLOAT, DEPTH_ATTACHMENT, READ_FRAMEBUFFER, NONE, RGB32UI, DRAW_FRAMEBUFFER};
use image::Pixel;
use nalgebra::Perspective3;
use crate::{ecs::{world_resources::ScreenDimensions, World}, math::{Transforms, math::{Mat4, Vec3}}};

use super::{pixel_info::PixelInfo, FrameBuffer, Texture, TextureAttachment, Program, PickingProgram};
use eyre::Result;

//I think I need another struct that lists the object index and their pixel info together
pub struct SelectableObject{
  fbo:FrameBuffer,
  program:PickingProgram,
  picking_texture:TextureAttachment,
  depth_texture:TextureAttachment,
  object_index:GLint,
  pixel_info:PixelInfo,
  screen_dimensions:ScreenDimensions
}

impl SelectableObject{
  pub fn new(gl:&Gl,world:&World,name:&str,screen_dimensions:&ScreenDimensions)->Self{
    let fbo = FrameBuffer::new(&gl);
    let program = PickingProgram::new(gl, world, name);
    let picking_texture = TextureAttachment::new(&gl, 0);
    let depth_texture = TextureAttachment::new(&gl, 0);
    let object_index = 0;
    let pixel_info = PixelInfo::default();
    let screen_dimensions = screen_dimensions.clone();
    
    SelectableObject { 
      fbo, 
      program, 
      picking_texture, 
      depth_texture,
      object_index, 
      pixel_info, 
      screen_dimensions 
    }
  }

  // I think I need to delete the frame buffer at the end?
  pub fn render(&self)->Result<()>{
    self.program.set_object_index(self.object_index);
    
    self.fbo.enable_writing();
    self.fbo.bind();

    self.generate_information_buffer();
    self.generate_depth_buffer();

    self.fbo.bind_default();

    self.fbo.check_framebuffer_status()
  }

  ///Creates and attaches the texture object for the primitive information buffer.
  fn generate_information_buffer(&self){
    //unsure if this should be in the main update function
    self.picking_texture.generate_texture_attachment(
      &self.screen_dimensions,
      RGB32UI,
      RGB_INTEGER,
      UNSIGNED_INT
    );

    let attachment = COLOR_ATTACHMENT0;
    let texture_obj = self.picking_texture.get_texture_obj();
    self.fbo.attach_2d_texture(attachment, texture_obj);
  }

  ///Creates and attaches the texture object for the depth buffer.
  fn generate_depth_buffer(&self){
    //unsure if this should be in the main update function
    //do I want to use integers instead of floating points in order to avoid imprecision?
    self.depth_texture.generate_texture_attachment(
      &self.screen_dimensions,
      DEPTH_COMPONENT,
      DEPTH_COMPONENT,
      FLOAT
    );
    
    let attachment = DEPTH_ATTACHMENT;
    let texture_obj = self.depth_texture.get_texture_obj();
    self.fbo.attach_2d_texture(attachment, texture_obj);
  }

  pub fn read_pixel_info(&self,gl:&Gl){
    unsafe{
      gl.BindFramebuffer(READ_FRAMEBUFFER, self.fbo.get_buffer_obj());

      gl.ReadBuffer(COLOR_ATTACHMENT0);

      let pixel:i128 = 1;
      gl.ReadPixels(1, 1, 1, 1, RGB_INTEGER, UNSIGNED_INT, pixel as *mut c_void);

      gl.ReadBuffer(NONE);

      gl.BindFramebuffer(READ_FRAMEBUFFER, 0);

      dbg!(pixel);

    // return Pixel;
  }
    todo!()
  }

  //I think the render for this should go into the Renderable Object render logic
  // pub fn render(&self){
  //   // Create the FBO
  //   let fbo:FrameBuffer = FrameBuffer::new(&gl);
  //   self.generate_information_buffer();
  // }
}