// use std::any::Any;

// use crate::{entities::query::Query, World};

// use super::system::System;

// type BoxedSystem = Box<dyn Any>;

  
// pub struct Dispatcher{
//   systems:Vec<BoxedSystem>
// }

// impl Dispatcher{
//   pub fn new()->Self{
//     Dispatcher{
//       systems:Vec::new()
//     }
//   }

//   pub fn register_system<S>(&mut self,system:S)->&mut Self
//   where S: System
//   {
//     let system = system;
//     self.systems.push(Box::new(system));
//     self
//   }

//   pub fn run(&mut self,world:&World){
//     for system in &mut self.systems {
//       system.run(world)
//     }
//   }

//   pub fn number_of_systems(&self)->usize{
//     dbg!(self.systems.len())
//   }
// }