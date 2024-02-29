use crate::{
  component_lib::{Destination, Path, Position},
  ecs::{CommandBuffer, World},
};

///Checks for entities with a [`Path`]. If a `Path` exists and the current [`Destination`] has been reached, replaces the current `Destination` with the next `Destination` in the `Path`.
pub fn update_path(world: &mut World) {
  let mut query = world.query();
  let entities = query.with_component::<Path>().unwrap().run();

  //Create the CommandBuffer
  let mut commands = CommandBuffer::default();

  for entity in entities {
    let mut destination = entity.get_component_mut::<Destination>().unwrap();
    let position = entity.get_component::<Position>().unwrap();
    //If the entity's destination has been reached check for the next node in the path
    if position.0 == destination.0 {
      let mut path = entity.get_component_mut::<Path>().unwrap();
      //If there is another node in the path, set the destination equal to the node
      if let Some(node) = path.next() {
        *destination = node;
      }
      //If the path has no more nodes, remove the path
      else {
        commands.remove_component::<Path>(entity.id);
      }
    }
  }
  commands.run(world);
}
