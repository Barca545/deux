use std::{any::Any, cell::{Ref, RefCell, RefMut}, rc::Rc};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct ComponentRef<T>{
  component_ref:Rc<RefCell<Box<dyn Any>>>,
  component_type: PhantomData<T>
}

impl<T:Any> ComponentRef<T> {
  pub fn new<Takes>(component_ref:Rc<RefCell<Box<dyn Any>>>) -> Self {
    ComponentRef{
      component_ref,
      component_type: PhantomData
    }
  }

  pub fn get_component(&self) -> Ref<T> {
    let component_ref = &self.component_ref;
    let component = Ref::map(component_ref.borrow(), |any| any.downcast_ref::<T>().unwrap());
    component
  }

  pub fn get_component_mut(&self) -> RefMut<T> {
    let component_ref = &self.component_ref;
    let component = RefMut::map(component_ref.borrow_mut(), |any| any.downcast_mut::<T>().unwrap());
    component
  }
}
