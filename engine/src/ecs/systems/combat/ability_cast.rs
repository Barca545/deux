// Todo:
// -Move the spawn/run scripts logic from the ability start to here.
//  The ability_start subsystem should just handle queuing an a cast and setting the windup timer
// -This system needs to check if a casting cooldown is over and if it is, cast the specified ability

use crate::ecs::World;

pub fn ability_cast(world: &World) {}
