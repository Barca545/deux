use crate::{
  scene::{material::Material, model::Model},
  utils::cache::{BindGroupCache, BindGroupKey, BindGroupLayoutCache, MaterialCache, MaterialKey},
};
use eyre::eyre;
use storage::Arena;
use wgpu::{BindGroup, Buffer, RenderPipeline};

// TODO: Need a better description in documentation. Problem is this is
// ultimately a pretty random struct that just holds data that doesn't easily
// fit anywhere else
/// Collection of all the data needed for rendering.
pub struct RenderResources {
  /// Collection of cached [`BindGroup`](wgpu::BindGroup)s the
  /// [`Renderer`](super::renderer::Renderer) will use.
  bindgroups: BindGroupCache,
  /// Collection of cached [`BindGroupLayout`](wgpu::BindGroupLayout)s the
  /// [`Renderer`](super::renderer::Renderer) will use.
  bindgrouplayouts: BindGroupLayoutCache,
  /// Collection of cached [`Material`](crate::scene::material::Material)s the
  /// [`Renderer`](super::renderer::Renderer) will use.
  materials: MaterialCache,
  // TODO: See if I can just use wgpu's pipelinecache instead
  /// Collection of cached [`RenderPipeline`](wgpu::RenderPipeline)s the
  /// [`Renderer`](super::renderer::Renderer) will use.
  // pipelines: RenderPipelineCache,
  pipelines: Vec<RenderPipeline,>,
  pub camera: CameraResources,
  pub models: Arena<Model,>,
}

impl RenderResources {
  pub fn new(camera: CameraResources,) -> Self {
    RenderResources {
      bindgroups: BindGroupCache::new(),
      bindgrouplayouts: BindGroupLayoutCache::new(),
      materials: MaterialCache::new(),
      pipelines: Vec::new(),
      camera,
      // TODO: Optimize by using with_capacity since the number of models per stage will be known
      // at initialization
      models: Arena::new(),
    }
  }

  pub fn insert_bindgroup(&mut self, bindgroup: BindGroup,) -> BindGroupKey {
    self.bindgroups.insert(bindgroup,)
  }

  pub fn get_bindgroup(&self, id: BindGroupKey,) -> &wgpu::BindGroup {
    &self.bindgroups.get(id,)
  }

  pub fn insert_material(&mut self, material: Material,) -> MaterialKey {
    self.materials.insert(material,)
  }

  pub fn get_material(&self, id: MaterialKey,) -> &Material {
    &self.materials.get(id,)
  }

  pub fn insert_pipeline(&mut self, id: usize, pipeline: RenderPipeline,) {
    assert_eq!(
      self.pipelines.len(),
      id,
      "{}",
      // TODO: Make this a real error
      format!(
        "Tried to insert Pipeline {} but the len of the pipeline cache was {}. Len must equal pipeline id for insertion.",
        id,
        self.pipelines.len()
      )
    );
    self.pipelines.push(pipeline,);
  }

  pub fn get_pipeline(&self, id: usize,) -> &RenderPipeline {
    &self.pipelines[id]
  }
}

// TODO: Need better documentation
/// Data needed to use the [`Camera`](crate::renderer::scene::camera::Camera)
/// for renderering.
pub struct CameraResources {
  pub bindgroup: wgpu::BindGroup,
  pub buffer: Buffer,
}
