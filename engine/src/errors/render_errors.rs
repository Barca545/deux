use thiserror::Error;

#[derive(Debug, Error)]
pub enum RenderErrors {
	#[error("Unable to create a texture from the provided integer.")]
	FailedToCreatePickingTexture,
}

#[derive(Debug, Error)]
#[allow(non_camel_case_types)]
pub enum FramebufferErrors {
	#[error("Framebuffer is the default read or draw framebuffer, but the default framebuffer does not exist.")]
	GL_FRAMEBUFFER_UNDEFINED,
	#[error("The framebuffer attachment points are framebuffer incomplete.")]
	GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT,
	#[error("The framebuffer does not have an image attached.")]
	GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT,
	#[error("The value of GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE is GL_NONE for any color attachment point(s) named by GL_DRAW_BUFFERi.")]
	GL_FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER,
	#[error("GL_READ_BUFFER is not GL_NONE and the value of GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE is GL_NONE for the color attachment point named by GL_READ_BUFFER.")]
	GL_FRAMEBUFFER_INCOMPLETE_READ_BUFFER,
	#[error("The combination of internal formats of the attached images violates an implementation-dependent set of restrictions")]
	GL_FRAMEBUFFER_UNSUPPORTED,
	#[error("The value of GL_RENDERBUFFER_SAMPLES is not the same for all attached renderbuffers; if the value of GL_TEXTURE_SAMPLES is the not same for all attached textures; or, if the attached images are a mix of renderbuffers and textures, the value of GL_RENDERBUFFER_SAMPLES does not match the value of GL_TEXTURE_SAMPLES or the value of GL_TEXTURE_FIXED_SAMPLE_LOCATIONS is not the same for all attached textures; or, if the attached images are a mix of renderbuffers and textures, the value of GL_TEXTURE_FIXED_SAMPLE_LOCATIONS is not GL_TRUE for all attached textures.")]
	GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE,
	#[error("A framebuffer attachment is layered, and a populated attachment is not layered, or if all populated color attachments are not from textures of the same target.")]
	GL_FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS,
	#[error("Target was not GL_DRAW_FRAMEBUFFER, GL_READ_FRAMEBUFFER or GL_FRAMEBUFFER.")]
	GL_INVALID_ENUM,
	#[error("Framebuffer was not zero or the name of an existing framebuffer object.")]
	GL_INVALID_OPERATION,
}