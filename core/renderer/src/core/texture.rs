use super::gpu_context::GpuContext;
use image::{DynamicImage, GenericImageView};
use wgpu::{
  AddressMode, CompareFunction, Extent3d, FilterMode, Origin3d, Sampler, SamplerDescriptor,
  TexelCopyBufferLayout, TexelCopyTextureInfo, TextureAspect, TextureDescriptor, TextureDimension,
  TextureFormat, TextureUsages, TextureView, TextureViewDescriptor,
};

// Refactor:
// - Bind group layout might be generic and should maybe get passed into the
// creation instead of generated inside
// - Modify texture loading

#[derive(Debug,)]
pub struct Texture {
  pub label: String,
  pub texture: wgpu::Texture,
  pub view: TextureView,
  pub sampler: Sampler,
}

impl Texture {
  pub const DEPTH_FORMAT: TextureFormat = TextureFormat::Depth32Float;
  // TODO: Unsure if this is needed
  // pub const DEFAULT_FORMAT: TextureFormat = TextureFormat::Rgba16Float;

  /// Create a new [`Texture`].
  pub fn from_image(ctx: &GpuContext, img: DynamicImage, label: &str,) -> Self {
    // Create rgba8 image and dimension info
    let rgba = img.to_rgba8();
    let dimensions = img.dimensions();

    let size = Extent3d {
      width: dimensions.0,
      height: dimensions.1,
      depth_or_array_layers: 1,
    };

    let texture = ctx.device.create_texture(&TextureDescriptor {
      label: Some("diffuse texture",),
      size,
      mip_level_count: 1,
      sample_count: 1,
      dimension: TextureDimension::D2,
      format: TextureFormat::Rgba8UnormSrgb,
      // TEXTURE_BINDING enables the texture for shaders
      // COPY_DST enables copying data to the texture
      usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
      view_formats: &[],
    },);

    // Copy the image data into the texture
    ctx.queue.write_texture(
      // Tells wgpu where to copy the texture
      TexelCopyTextureInfo {
        texture: &texture,
        mip_level: 0,
        origin: Origin3d::ZERO,
        aspect: TextureAspect::All,
      },
      &rgba,
      TexelCopyBufferLayout {
        offset: 0,
        bytes_per_row: Some(4 * dimensions.0,),
        rows_per_image: Some(dimensions.1,),
      },
      size,
    );

    // Create the texture view and sampler
    let view = texture.create_view(&TextureViewDescriptor::default(),);
    let sampler = ctx.device.create_sampler(&SamplerDescriptor {
      address_mode_u: AddressMode::ClampToEdge,
      address_mode_v: AddressMode::ClampToEdge,
      address_mode_w: AddressMode::ClampToEdge,
      mag_filter: FilterMode::Linear,
      min_filter: FilterMode::Nearest,
      mipmap_filter: FilterMode::Nearest,
      ..Default::default()
    },);

    Texture {
      label: label.to_string(),
      texture,
      view,
      sampler,
    }
  }

  /// Creates a [`Texture`] used for depth testing.
  pub fn create_depth_texture(ctx: &GpuContext,) -> Self {
    let label = "depth texture";

    let size = Extent3d {
      width: ctx.config.width,
      height: ctx.config.height,
      depth_or_array_layers: 1,
    };

    // Create a texture
    let texture = ctx.device.create_texture(&TextureDescriptor {
      label: Some(label,),
      size,
      mip_level_count: 1,
      sample_count: 1,
      dimension: TextureDimension::D2,
      format: Self::DEPTH_FORMAT,
      usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
      view_formats: &[],
    },);

    let view = texture.create_view(&TextureViewDescriptor::default(),);

    let sampler = ctx.device.create_sampler(&SamplerDescriptor {
      label: Some(label,),
      address_mode_u: AddressMode::ClampToEdge,
      address_mode_v: AddressMode::ClampToEdge,
      address_mode_w: AddressMode::ClampToEdge,
      mag_filter: FilterMode::Linear,
      min_filter: FilterMode::Linear,
      mipmap_filter: FilterMode::Nearest,
      lod_min_clamp: 0.0,
      lod_max_clamp: 100.0,
      compare: Some(CompareFunction::LessEqual,),
      ..Default::default()
    },);
    Texture {
      label: label.to_string(),
      texture,
      view,
      sampler,
    }
  }
}
