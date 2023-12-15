use gl::{Gl, types::{GLuint, GLint, GLvoid}, FALSE, FLOAT};

pub(crate) fn define_vertex_attrib_pointer(gl:&Gl, stride:usize, location:usize, offset:usize, tuple_size:GLint) {
  //why does GITGD (https://github.com/amengede/OpenGL-for-Beginners/blob/main/week%2006%20design%20patterns/4%20entity%20component%20system/src/controller/app.cpp#L12)
  //have EnableVertexAttribArray after VertexAttribPointer?
  unsafe{
    gl.EnableVertexAttribArray(location as GLuint);
    gl.VertexAttribPointer(
      location as GLuint,
      tuple_size,
      FLOAT,
      FALSE,
      stride as GLint,
      offset as *const GLvoid
    );
  }
  
}