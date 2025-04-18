mod errors;
pub mod frame_inputs;
pub mod keybinds;
mod mouseray;
// pub mod user_inputs;

// TODO: Another big problem is with something like the auto attacks.
// Because that is not an input but it will need to know the mouse position.
// Specific case is an auto attack that fires towards the mouse location every
// X miliseconds

// Maybe add some functionality that accepts a timestamp and returns the mouse's
// position at that timestamp?

// TODO: Somewhere I need to define that a "Frame" is the term I am using for a
// single tick. Should I just use "Tick" tho?
