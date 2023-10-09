use crate::World;

//https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md

//https://docs.rs/specs/latest/specs/trait.System.html#associatedtype.SystemData
pub trait System{  
  //I think having it output the data in case the next system in the chain needs it is a solid idea
  fn run(&mut self,world: &mut World);
}
//confirm if I need to use boxes or other fancy things
//pub type BoxedSystem<'a> = Rc<RefCell<dyn System<'a,SystemData = Vec<QueryEntity<'a>>>>>;

pub type BoxedSystem = Box<dyn System>;


///Do not really need the stuff below since Specs uses it because they have different Data types
//https://docs.rs/shred/0.15.0/shred/trait.DynamicSystemData.html
pub trait DynamicSystemData {
  type Accessor;
}

//https://docs.rs/shred/0.15.0/shred/trait.Accessor.html 
pub trait Accessor {
  type DataType;

  fn new() -> Self;

  fn reads();

  fn writes();
}