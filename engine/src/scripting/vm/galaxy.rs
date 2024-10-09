use std::collections::HashMap;

use super::errors::VMError;
use nina::world::World;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

// To Do:
// - Add support for arrays -> this means the ability to read from the heap
// - Tail call optimization in the compiler?
// - Implement div and pow with wrapping?
// - Add PUSH/POP instructions

// Refactor:
// - Instead of to/from bits just use direct transmutes since compilation should
//   take care of the same checks they take care of? I don't think the above is
//   true actually.
// - Should MOVE be renamed copy?
// - No reason for both JNZ and JZ?
// - Don't use a reference but an RC? might be less fidly and I don't think I
//   ever need mutable world. Could do Rc<dyn any> in a hashmap for external
//   variables
// - Make the opcodes constants?
// - Add a better error for if an op is not recognized than just unwrapping

// x86 opcode definitions
// https://math.hws.edu/eck/cs220/f22/registers.html

//Define opcodes
#[allow(non_camel_case_types)]
#[derive(FromPrimitive, Debug, PartialEq, Clone, Copy,)]
pub enum OpCode {
  /// Halt the execution of code.
  HLT,
  /// Takes 2 arguments:
  /// - `Dest` register.
  /// - Immediate u8 value.
  ///
  /// Loads an immediate u8 value into `Dest`.
  LOAD_U8,
  /// Takes 2 arguments:
  /// - `Dest` register.
  /// - Immediate i32 value.
  ///
  /// Loads an immediate i32 value into `Dest`.
  LOAD_INT,
  /// Takes 2 arguments:
  /// - `Dest` register.
  /// - Immediate f32 value.
  ///
  /// Loads an immediate f32 value into `Dest`.
  LOAD_FLOAT,
  /// Takes 2 arguments:
  /// - `Dest` register.
  /// - `Src` register.
  ///
  /// Copy the value in `Src` into `Dest`.
  MOVE,
  /// Takes 2 arguments:
  /// - Integer stored in the register of index a.
  /// - Integer stored in the register of index b.
  ///
  /// Store a * b in the [`RAX`](Tardis::RAX).
  MULT,
  /// Takes 2 arguments:
  /// - Integer stored in the register of index a.
  /// - Immediate i32 value.
  ///
  /// Store a * b in the [`RAX`](Tardis::RAX).
  MULT_RI,
  /// Takes 2 arguments:
  /// - Immediate i32 value.
  /// - Immediate i32 value.
  ///
  /// Store a * b in the [`RAX`](Tardis::RAX).
  MULT_II,
  /// Takes 2 arguments:
  /// - Integer stored in the register of index a.
  /// - Integer stored in the register of index b.
  ///
  /// Store a / b in the [`RAX`](Tardis::RAX).
  DIV,
  /// Takes 2 arguments:
  /// - Integer stored in the register of index a.
  /// - Immediate i32 value.
  ///
  /// Store a + b in the [`RAX`](Tardis::RAX).
  DIV_RI,
  /// Takes 2 arguments:
  /// - Immediate i32 value.
  /// - Immediate i32 value.
  ///
  /// Store a + b in the [`RAX`](Tardis::RAX).
  DIV_II,
  /// Takes 2 arguments:
  /// - Integer stored in the register of index a.
  /// - Integer stored in the register of index b.
  ///
  /// Store a + b in the [`RAX`](Tardis::RAX).
  ADD,
  /// Takes 2 arguments:
  /// - Integer stored in the register of index a.
  /// - Immediate i32 value.
  ///
  /// Store a - b in the [`RAX`](Tardis::RAX).
  ADD_RI,
  /// Takes 2 arguments:
  /// - Immediate i32 value.
  /// - Immediate i32 value.
  ///
  /// Store a - b in the [`RAX`](Tardis::RAX).
  ADD_II,
  /// Takes 2 arguments:
  /// - Integer stored in the register of index a.
  /// - Integer stored in the register of index b.
  ///
  /// Store a - b in the [`RAX`](Tardis::RAX).
  SUB,
  /// Takes 2 arguments:
  /// - Integer stored in the register of index a.
  /// - Immediate i32 value.
  ///
  /// Store a - b in the [`RAX`](Tardis::RAX).
  SUB_RI,
  /// Takes 2 arguments:
  /// - Immediate i32 value.
  /// - Immediate i32 value.
  ///
  /// Store a - b in the [`RAX`](Tardis::RAX).
  SUB_II,
  /// Takes 2 arguments:
  /// - Float stored in the register of index a.
  /// - Float stored in the register of index b.
  ///
  /// Store a^b in the [`RAX`](Tardis::RAX).
  POW,
  /// Takes 2 arguments:
  /// - Float stored in the register of index a.
  /// - Immediate i32 value.
  ///
  /// Store a^b in the [`RAX`](Tardis::RAX).
  POW_RI,
  /// Takes 2 arguments:
  /// - Immediate i32 value.
  /// - Immediate i32 value.
  ///
  /// Store a^b in the [`RAX`](Tardis::RAX).
  POW_II,
  /// Takes 2 arguments:
  /// - Float stored in the register of index a.
  /// - Float stored in the register of index b.
  ///
  /// Store a * b in the [`RAX`](Tardis::RAX).
  F_MULT,
  /// Takes 2 arguments:
  /// - Float stored in the register of index a.
  /// - Immediate f32 value.
  ///
  /// Store a / b in the [`RAX`](Tardis::RAX).
  F_MULT_RI,
  /// Takes 2 arguments:
  /// - Immediate f32 value.
  /// - Immediate f32 value.
  ///
  /// Store a / b in the [`RAX`](Tardis::RAX).
  F_MULT_II,
  /// Takes 2 arguments:
  /// - Float stored in the register of index a.
  /// - Float stored in the register of index b.
  ///
  /// Store a / b in the [`RAX`](Tardis::RAX).
  F_DIV,
  /// Takes 2 arguments:
  /// - Float stored in the register of index a.
  /// - Immediate f32 value.
  ///
  /// Store a / b in the [`RAX`](Tardis::RAX).
  F_DIV_RI,
  /// Takes 2 arguments:
  /// - Immediate f32 value.
  /// - Immediate f32 value.
  ///
  /// Store a / b in the [`RAX`](Tardis::RAX).
  F_DIV_II,
  /// Takes 2 arguments:
  /// - Float stored in the register of index a.
  /// - Float stored in the register of index b.
  ///
  /// Store a + b in the [`RAX`](Tardis::RAX).
  F_ADD,
  /// Takes 2 arguments:
  /// - Float stored in the register of index a.
  /// - Immediate f32 value.
  ///
  /// Store a + b in the [`RAX`](Tardis::RAX).
  F_ADD_RI,
  /// Takes 2 arguments:
  /// - Immediate f32 value.
  /// - Immediate f32 value.
  ///
  /// Store a + b in the [`RAX`](Tardis::RAX).
  F_ADD_II,
  /// Takes 2 arguments:
  /// - Float stored in the register of index a.
  /// - Float stored in the register of index b.
  ///
  /// Store a - b in the [`RAX`](Tardis::RAX).
  F_SUB,
  /// Takes 2 arguments:
  /// - Float stored in the register of index a.
  /// - Immediate f32 value.
  ///
  /// Store a - b in the [`RAX`](Tardis::RAX).
  F_SUB_RI,
  /// Takes 2 arguments:
  /// - Immediate f32 value.
  /// - Immediate f32 value.
  ///
  /// Store a - b in the [`RAX`](Tardis::RAX).
  F_SUB_II,
  /// Takes 2 arguments:
  /// - Float stored in the register of index a.
  /// - Float stored in the register of index b.
  ///
  /// Store a^b in the [`RAX`](Tardis::RAX).
  F_POW,
  /// Takes 2 arguments:
  /// - Float stored in the register of index a.
  /// - Immediate f32 value.
  ///
  /// Store the result of a == b in the [`REQ`](Tardis::REQ).
  F_POW_RI,
  /// Takes 2 arguments:
  /// - Immediate f32 value.
  /// - Immediate f32 value.
  ///
  /// Store the result of a == b in the [`REQ`](Tardis::REQ).
  F_POW_II,
  /// Takes 2 arguments:
  /// - Float stored in the register of index a.
  /// - Float stored in the register of index b.
  ///
  /// Store the result of a == b in the [`REQ`](Tardis::REQ).
  EQUAL,
  /// Takes 2 arguments:
  /// - First register.
  /// - Second register.
  ///
  /// Store the result of a != b in the [`REQ`](Tardis::REQ).
  NOT_EQUAL,
  /// Takes 2 arguments:
  /// - First register.
  /// - Second register.
  ///
  /// Store the result of a > b in the [`REQ`](Tardis::REQ).
  GREATER,
  /// Takes 2 arguments:
  /// - First register.
  /// - Second register.
  ///
  /// Store the result of a < b in the [`REQ`](Tardis::REQ).
  LESS,
  /// Takes 2 arguments:
  /// - First register.
  /// - Second register.
  ///
  /// Store the result of a => b in the [`REQ`](Tardis::REQ).
  GREATER_REQUAL,
  /// Takes 2 arguments:
  /// - First register.
  /// - Second register.
  ///
  /// Store the result of a <= b in the [REQ](`Tardis::REQ`).
  LESS_REQUAL,
  /// Takes 1 argmumet:
  /// - New `pc`value.
  JUMP,
  /// Takes 2 arguments:
  /// - Register to check.
  /// - New `pc`value if a == 0.
  JZ,
  /// Takes 2 arguments:
  /// - Register to check.
  /// - New `pc`value if a != 0.
  JNZ,
  /// Takes 5 arguments:
  /// - Memory address of the function.
  ///
  /// Saves the address of the next instruction and sets the `pc` to the
  /// address of the function.
  CALL,
  /// Calls an external function.
  /// Takes 5 arguments:
  /// - Function id
  /// - Number of arguments
  /// - Number of returns
  /// - Return base register
  /// - Args
  SYS_CALL,
  /// Sets the `pc` to the `pc` value the last [`CALL`](OpCode::CALL) placed on
  /// the stack.
  RETURN,
  /// A noop.
  NOOP,
}

#[derive(PartialEq,)]
pub enum LoopControl {
  Continue,
  Break,
}

//Define arguments

///VM for Galaxy bytecode.
///
/// Tardis is big endian.
#[allow(unused)]
pub struct Tardis<'w,> {
  ///World Ref
  world:&'w World,
  /// The program counter indicates the next instruction to execute.
  pc:usize,
  /// Program bytecode.
  program:Vec<u8,>,
  /// The VM's registers:
  /// - R0-R3 function arguments and returns. If a function has more than four
  ///   args or returns, they go onto the stack.
  /// - REQ register is R4.
  /// - RAX is R5.
  /// - RCX is R6.
  /// - Floats are R7-R15
  /// - General purpose registers are R16-R255.
  registers:[u32; 255],
  stack:Vec<u32,>,
  /// Heap memory.
  mem:Vec<u8,>,
  /// External functions.
  functions:HashMap<usize, fn(&mut Tardis,),>,
}

#[allow(unused)]
impl<'w,> Tardis<'w,> {
  ///Index of the register which stores the result of the last equality
  /// operation.
  const REQ:usize = 32;

  /// Index of the accumulator register, holds the result of arithmetic
  /// operations.
  const RAX:usize = 33;

  /// Index of the counter register which holds the number of times a process is
  /// to be repeated.
  const RCX:usize = 34;

  pub fn new(world:&'w World,) -> Self {
    Tardis {
      world,
      pc:0,
      program:Vec::new(),
      registers:[0; 255],
      stack:Vec::new(),
      mem:Vec::new(),
      functions:HashMap::new(),
    }
  }

  /// Load a new external function into the VM's registry of external functions.
  fn register_external_function(&mut self, func:fn(&mut Tardis,),) {
    let id = self.functions.len();
    self.functions.insert(id, func,);
  }

  /// Decode a byte into an [OpCode].
  fn decode(&mut self,) -> (OpCode) {
    let op_byte = self.program[self.pc];
    let op = FromPrimitive::from_u8(op_byte,).ok_or(VMError::UnrecognizedOpCode(op_byte,),).unwrap();
    dbg!(op);
    self.pc += 1;
    (op)
  }

  /// Execute the provided [OpCode].
  fn execute(&mut self, op:OpCode,) -> LoopControl {
    match op {
      OpCode::HLT => LoopControl::Break,
      OpCode::LOAD_U8 => self.LOAD_U8(),
      OpCode::LOAD_INT => self.LOAD_INT(),
      OpCode::LOAD_FLOAT => self.LOAD_FLOAT(),
      OpCode::MOVE => self.MOVE(),
      OpCode::MULT => self.MULT(),
      OpCode::MULT_RI => self.MULT_RI(),
      OpCode::MULT_II => self.MULT_II(),
      OpCode::DIV => self.DIV(),
      OpCode::DIV_RI => self.DIV_RI(),
      OpCode::DIV_II => self.DIV_II(),
      OpCode::ADD => self.ADD(),
      OpCode::ADD_RI => self.ADD_RI(),
      OpCode::ADD_II => self.ADD_II(),
      OpCode::SUB => self.SUB(),
      OpCode::SUB_RI => self.SUB_RI(),
      OpCode::SUB_II => self.SUB_II(),
      OpCode::POW => self.POW(),
      OpCode::POW_RI => self.POW_RI(),
      OpCode::POW_II => self.POW_II(),
      OpCode::F_MULT => self.F_MULT(),
      OpCode::F_MULT_RI => self.F_MULT_RI(),
      OpCode::F_MULT_II => self.F_MULT_II(),
      OpCode::F_DIV => self.F_DIV(),
      OpCode::F_DIV_RI => self.F_DIV_RI(),
      OpCode::F_DIV_II => self.F_DIV_II(),
      OpCode::F_ADD => self.F_ADD(),
      OpCode::F_ADD_RI => self.F_ADD_RI(),
      OpCode::F_ADD_II => self.F_ADD_II(),
      OpCode::F_SUB => self.F_SUB(),
      OpCode::F_SUB_RI => self.F_SUB_RI(),
      OpCode::F_SUB_II => self.F_SUB_II(),
      OpCode::F_POW => self.F_POW(),
      OpCode::F_POW_RI => self.F_POW_RI(),
      OpCode::F_POW_II => self.F_POW_II(),
      OpCode::EQUAL => self.EQUAL(),
      OpCode::NOT_EQUAL => self.NOT_EQUAL(),
      OpCode::GREATER => self.GREATER(),
      OpCode::LESS => self.LESS(),
      OpCode::GREATER_REQUAL => self.GREATER_EQUAL(),
      OpCode::LESS_REQUAL => self.LESS_EQUAL(),
      OpCode::JUMP => self.JUMP(),
      OpCode::JZ => self.JZ(),
      OpCode::JNZ => self.JNZ(),
      OpCode::CALL => self.CALL(),
      OpCode::RETURN => self.RETURN(),
      OpCode::SYS_CALL => self.SYS_CALL(),
      OpCode::NOOP => LoopControl::Continue,
    }
  }

  /// Run the loaded program.
  pub fn run(&mut self,) {
    loop {
      let op = self.decode();
      match self.execute(op,) {
        LoopControl::Break => break,
        LoopControl::Continue => continue,
      }
    }
  }

  /// Load a program into the [`Tardis`]'s `program` slot.
  pub fn load(&mut self, program:Vec<u8,>,) {
    self.program = program;
  }

  /// Empty the [`Tardis`]'s `program` slot and resets the the `pc`.
  pub fn clear(&mut self,) {
    self.program = Vec::new();
    self.pc = 0;
  }

  /// Fetch the next byte from the `program`.
  pub(super) fn get_u8(&mut self,) -> u8 {
    // Fetch the next byte in the program
    let num = self.program[self.pc];
    // Increment the pc
    self.pc += 1;
    num
  }

  /// Fetch the next 4 bytes from the `program`as a u32.
  pub(super) fn get_u32(&mut self,) -> u32 {
    // Fetch the next four bytes in the program as a [u8;4]
    // and convert them into a u32
    let num = u32::from_be_bytes([
      self.program[self.pc],
      self.program[self.pc + 1],
      self.program[self.pc + 2],
      self.program[self.pc + 3],
    ],);

    // Increment the pc
    self.pc += 4;
    num
  }

  /// Fetch the next 4 bytes from the `program`as a i32.
  fn get_i32(&mut self,) -> i32 {
    // Fetch the next four bytes in the program as a [u8;4]
    // and convert them into a i32
    let num = i32::from_be_bytes([
      self.program[self.pc],
      self.program[self.pc + 1],
      self.program[self.pc + 2],
      self.program[self.pc + 3],
    ],);

    // Increment the pc
    self.pc += 4;
    num
  }

  /// Fetch the next 4 bytes from the `program` as a f32.
  fn get_f32(&mut self,) -> f32 {
    // Fetch the next four bytes in the program as a [u8;4]
    // and transmute them into a f32
    let num = f32::from_be_bytes([
      self.program[self.pc],
      self.program[self.pc + 1],
      self.program[self.pc + 2],
      self.program[self.pc + 3],
    ],);
    // Increment the pc
    self.pc += 4;
    num
  }
}

// Opcode implementation block.
#[allow(non_snake_case)]
impl<'w,> Tardis<'w,> {
  fn LOAD_U8(&mut self,) -> LoopControl {
    let register = self.get_u8() as usize;
    let byte = self.get_u8() as u32;
    self.registers[register] = byte;
    LoopControl::Continue
  }

  fn LOAD_INT(&mut self,) -> LoopControl {
    // Get the arguments
    let register = self.get_u8() as usize;
    let int = self.get_u32();

    // Perform the operation
    self.registers[register] = int;
    LoopControl::Continue
  }

  fn LOAD_FLOAT(&mut self,) -> LoopControl {
    // Get the arguments
    let register = self.get_u8() as usize;
    let float = self.get_f32();

    // Perform the operation
    self.registers[register] = float.to_bits();
    LoopControl::Continue
  }

  fn MOVE(&mut self,) -> LoopControl {
    let a = self.get_u8() as usize;
    let b = self.get_u8() as usize;

    self.registers[a] = self.registers[b];
    LoopControl::Continue
  }

  fn MULT(&mut self,) -> LoopControl {
    // Get the arguments
    // let a = self.registers[self.get_u8() as usize] as i32;
    // let b = self.registers[self.get_u8() as usize] as i32;
    let a = self.registers[self.get_u8() as usize];
    let b = self.registers[self.get_u8() as usize];

    // Perform the operation
    self.registers[Self::RAX] = a.wrapping_mul(b,);
    LoopControl::Continue
  }

  fn MULT_RI(&mut self,) -> LoopControl {
    // Get the arguments
    let a = self.registers[self.get_u8() as usize];
    let b = self.get_u32();

    // Perform the operation
    self.registers[Self::RAX] = a.wrapping_mul(b,);
    LoopControl::Continue
  }

  fn MULT_II(&mut self,) -> LoopControl {
    // Get the arguments
    let a = self.get_u32();
    let b = self.get_u32();

    // Perform the operation
    self.registers[Self::RAX] = a.wrapping_mul(b,);
    LoopControl::Continue
  }

  fn DIV(&mut self,) -> LoopControl {
    // Get the arguments
    let a = self.registers[self.get_u8() as usize] as i32;
    let b = self.registers[self.get_u8() as usize] as i32;

    // Perform the operation
    self.registers[Self::RAX] = (a / b) as u32;
    LoopControl::Continue
  }

  fn DIV_RI(&mut self,) -> LoopControl {
    // Get the arguments
    let a = self.registers[self.get_u8() as usize] as i32;
    let b = self.get_i32();

    // Perform the operation
    self.registers[Self::RAX] = (a / b) as u32;
    LoopControl::Continue
  }

  fn DIV_II(&mut self,) -> LoopControl {
    // Get the arguments
    let a = self.get_i32();
    let b = self.get_i32();

    // Perform the operation
    self.registers[Self::RAX] = (a / b) as u32;
    LoopControl::Continue
  }

  fn ADD(&mut self,) -> LoopControl {
    // Get the arguments
    let a = self.registers[self.get_u8() as usize];
    let b = self.registers[self.get_u8() as usize];

    // Perform the operation
    self.registers[Self::RAX] = a.wrapping_add(b,);
    LoopControl::Continue
  }

  fn ADD_RI(&mut self,) -> LoopControl {
    // Get the arguments
    let a = self.registers[self.get_u8() as usize];
    let b = self.get_u32();

    // Perform the operation
    self.registers[Self::RAX] = a.wrapping_add(b,);
    LoopControl::Continue
  }

  fn ADD_II(&mut self,) -> LoopControl {
    // Get the arguments
    let a = self.get_u32();
    let b = self.get_u32();

    // Perform the operation
    self.registers[Self::RAX] = a.wrapping_add(b,);
    LoopControl::Continue
  }

  fn SUB(&mut self,) -> LoopControl {
    // Get the arguments
    let a = self.registers[self.get_u8() as usize];
    let b = self.registers[self.get_u8() as usize];

    // Perform the operation
    self.registers[Self::RAX] = a.wrapping_sub(b,);
    LoopControl::Continue
  }

  fn SUB_RI(&mut self,) -> LoopControl {
    // Get the arguments
    let a = self.registers[self.get_u8() as usize];
    let b = self.get_u32();

    // Perform the operation
    self.registers[Self::RAX] = a.wrapping_sub(b,);
    LoopControl::Continue
  }

  fn SUB_II(&mut self,) -> LoopControl {
    // Get the arguments
    let a = self.get_u32();
    let b = self.get_u32();

    // Perform the operation
    self.registers[Self::RAX] = a.wrapping_sub(b,);
    LoopControl::Continue
  }

  fn POW(&mut self,) -> LoopControl {
    // Get the arguments
    let a = self.registers[self.get_u8() as usize] as i32;
    let b = self.registers[self.get_u8() as usize] as i32;

    // Perform the operation
    if b > 0 {
      self.registers[Self::RAX] = a.pow(b as u32,) as u32;
    }
    else {
      // a^-b=1/a^b < 0 which rounds to 0
      self.registers[Self::RAX] = 0;
    }

    LoopControl::Continue
  }

  fn POW_RI(&mut self,) -> LoopControl {
    // Get the arguments
    let a = self.registers[self.get_u8() as usize] as i32;
    let b = self.get_i32();

    // Perform the operation
    if b > 0 {
      self.registers[Self::RAX] = a.pow(b as u32,) as u32;
    }
    else {
      // a^-b=1/a^b < 0 which rounds to 0
      self.registers[Self::RAX] = 0;
    }

    LoopControl::Continue
  }

  fn POW_II(&mut self,) -> LoopControl {
    // Get the arguments
    let a = self.get_i32();
    let b = self.get_i32();

    // Perform the operation
    if b > 0 {
      self.registers[Self::RAX] = a.pow(b as u32,) as u32;
    }
    else {
      // a^-b=1/a^b < 0 which rounds to 0
      self.registers[Self::RAX] = 0;
    }

    LoopControl::Continue
  }

  fn F_MULT(&mut self,) -> LoopControl {
    // Get the arguments
    let a = f32::from_bits(self.registers[self.get_u8() as usize],);
    let b = f32::from_bits(self.registers[self.get_u8() as usize],);

    // Perform the operation
    self.registers[Self::RAX] = (a * b).to_bits();
    LoopControl::Continue
  }

  fn F_MULT_RI(&mut self,) -> LoopControl {
    // Get the arguments
    let a = f32::from_bits(self.registers[self.get_u8() as usize],);
    let b = f32::from_bits(self.get_u32(),);

    // Perform the operation
    self.registers[Self::RAX] = (a * b).to_bits();
    LoopControl::Continue
  }

  fn F_MULT_II(&mut self,) -> LoopControl {
    // Get the arguments
    let a = f32::from_bits(self.get_u32(),);
    let b = f32::from_bits(self.get_u32(),);

    // Perform the operation
    self.registers[Self::RAX] = (a * b).to_bits();
    LoopControl::Continue
  }

  fn F_DIV(&mut self,) -> LoopControl {
    // Get the arguments
    let a = f32::from_bits(self.registers[self.get_u8() as usize],);
    let b = f32::from_bits(self.registers[self.get_u8() as usize],);

    // Perform the operation
    self.registers[Self::RAX] = (a / b).to_bits();
    LoopControl::Continue
  }

  fn F_DIV_RI(&mut self,) -> LoopControl {
    // Get the arguments
    let a = f32::from_bits(self.registers[self.get_u8() as usize],);
    let b = f32::from_bits(self.get_u32(),);

    // Perform the operation
    self.registers[Self::RAX] = (a / b).to_bits();
    LoopControl::Continue
  }

  fn F_DIV_II(&mut self,) -> LoopControl {
    // Get the arguments
    let a = f32::from_bits(self.get_u32(),);
    let b = f32::from_bits(self.get_u32(),);

    // Perform the operation
    self.registers[Self::RAX] = (a / b).to_bits();
    LoopControl::Continue
  }

  fn F_ADD(&mut self,) -> LoopControl {
    // Get the arguments
    let a = f32::from_bits(self.registers[self.get_u8() as usize],);
    let b = f32::from_bits(self.registers[self.get_u8() as usize],);

    // Perform the operation
    self.registers[Self::RAX] = (a + b).to_bits();
    LoopControl::Continue
  }

  fn F_ADD_RI(&mut self,) -> LoopControl {
    // Get the arguments
    let a = f32::from_bits(self.registers[self.get_u8() as usize],);
    let b = f32::from_bits(self.get_u32(),);

    // Perform the operation
    self.registers[Self::RAX] = (a + b).to_bits();
    LoopControl::Continue
  }

  fn F_ADD_II(&mut self,) -> LoopControl {
    // Get the arguments
    let a = f32::from_bits(self.get_u32(),);
    let b = f32::from_bits(self.get_u32(),);

    // Perform the operation
    self.registers[Self::RAX] = (a + b).to_bits();
    LoopControl::Continue
  }

  fn F_SUB(&mut self,) -> LoopControl {
    // Get the arguments
    let a = f32::from_bits(self.registers[self.get_u8() as usize],);
    let b = f32::from_bits(self.registers[self.get_u8() as usize],);

    // Perform the operation
    self.registers[Self::RAX] = (a - b).to_bits();
    LoopControl::Continue
  }

  fn F_SUB_RI(&mut self,) -> LoopControl {
    // Get the arguments
    let a = f32::from_bits(self.registers[self.get_u8() as usize],);
    let b = f32::from_bits(self.get_u32(),);

    // Perform the operation
    self.registers[Self::RAX] = (a - b).to_bits();
    LoopControl::Continue
  }

  fn F_SUB_II(&mut self,) -> LoopControl {
    // Get the arguments
    let a = f32::from_bits(self.get_u32(),);
    let b = f32::from_bits(self.get_u32(),);

    // Perform the operation
    self.registers[Self::RAX] = (a - b).to_bits();
    LoopControl::Continue
  }

  fn F_POW(&mut self,) -> LoopControl {
    // Get the arguments
    let a = f32::from_bits(self.registers[self.get_u8() as usize],);
    let b = f32::from_bits(self.registers[self.get_u8() as usize],);

    // Perform the operation
    self.registers[Self::RAX] = a.powf(b,).to_bits();
    LoopControl::Continue
  }

  fn F_POW_RI(&mut self,) -> LoopControl {
    // Get the arguments
    let a = f32::from_bits(self.registers[self.get_u8() as usize],);
    let b = self.get_f32();

    dbg!(a);
    dbg!(b);
    dbg!(a.powf(b,));

    // Perform the operation
    self.registers[Self::RAX] = a.powf(b,).to_bits();
    LoopControl::Continue
  }

  fn F_POW_II(&mut self,) -> LoopControl {
    // Get the arguments
    let a = f32::from_bits(self.get_u32(),);
    let b = self.get_f32();

    // Perform the operation
    self.registers[Self::RAX] = a.powf(b,).to_bits();
    LoopControl::Continue
  }

  fn EQUAL(&mut self,) -> LoopControl {
    // Get the arguments
    let a = f32::from_bits(self.registers[self.get_u8() as usize],);
    let b = f32::from_bits(self.registers[self.get_u8() as usize],);

    // Perform the operation and store it in the registers as a u32
    self.registers[Self::REQ] = (a == b) as u32;
    LoopControl::Continue
  }

  fn NOT_EQUAL(&mut self,) -> LoopControl {
    // Get the arguments
    let a = self.registers[self.get_u8() as usize];
    let b = self.registers[self.get_u8() as usize];

    // Perform the operation
    self.registers[Self::REQ] = (a != b) as u32;
    LoopControl::Continue
  }

  fn GREATER(&mut self,) -> LoopControl {
    // Get the arguments
    let a = self.registers[self.get_u8() as usize];
    let b = self.registers[self.get_u8() as usize];

    // Perform the operation
    self.registers[Self::REQ] = (a > b) as u32;
    LoopControl::Continue
  }

  fn LESS(&mut self,) -> LoopControl {
    // Get the arguments
    let a = self.registers[self.get_u8() as usize];
    let b = self.registers[self.get_u8() as usize];

    // Perform the operation
    self.registers[Self::REQ] = (a < b) as u32;
    LoopControl::Continue
  }

  fn GREATER_EQUAL(&mut self,) -> LoopControl {
    // Get the arguments
    let a = self.registers[self.get_u8() as usize];
    let b = self.registers[self.get_u8() as usize];

    // Perform the operation
    self.registers[Self::REQ] = (a >= b) as u32;
    LoopControl::Continue
  }

  fn LESS_EQUAL(&mut self,) -> LoopControl {
    // Get the arguments
    let a = self.registers[self.get_u8() as usize];
    let b = self.registers[self.get_u8() as usize];

    // Perform the operation
    self.registers[Self::REQ] = (a <= b) as u32;
    LoopControl::Continue
  }

  fn JUMP(&mut self,) -> LoopControl {
    self.pc = self.get_u8() as usize;
    LoopControl::Continue
  }

  fn JZ(&mut self,) -> LoopControl {
    let test = self.registers[self.get_u8() as usize];
    let pc = self.get_u8() as usize;

    if test == 0 {
      self.pc = pc;
    }
    LoopControl::Continue
  }

  fn JNZ(&mut self,) -> LoopControl {
    let test = self.registers[self.get_u8() as usize];
    let pc = self.get_u8() as usize;

    if test != 0 {
      self.pc = pc
    }
    LoopControl::Continue
  }

  fn CALL(&mut self,) -> LoopControl {
    let function_address = self.get_u8() as usize;
    // Save the pc to the stack
    self.stack.push(self.pc as u32,);

    //Jump to the function
    self.pc = function_address;

    LoopControl::Continue
  }

  fn RETURN(&mut self,) -> LoopControl {
    self.pc = self.stack.pop().unwrap() as usize;

    LoopControl::Continue
  }

  fn SYS_CALL(&mut self,) -> LoopControl {
    // Get the function by ID
    let id = self.get_u8() as usize;

    let func = *self.functions.get(&id,).unwrap();
    func(self,);

    LoopControl::Continue
  }
}

#[cfg(test)]
mod test {
  use super::Tardis;
  use crate::scripting::vm::galaxy::OpCode;
  use nina::world::World;

  #[test]
  fn get_u8_and_get_u32_work() {
    let world = World::new();
    let mut vm = Tardis::new(&world,);
    vm.load(vec![2, 0, 0, 0, 32],);

    assert_eq!(2, vm.get_u8());
    assert_eq!(32, vm.get_u32());
  }

  #[test]
  fn decode_works() {
    let world = World::new();
    let mut vm = Tardis::new(&world,);
    let program = vec![0, 1];
    vm.load(program,);
    let op = vm.decode();
    assert_eq!(op, OpCode::HLT);
  }

  #[test]
  // TESTS: EQUAL, NOT_EQUAL, LESS, GREATER, GREATER_EQUAL, LESS_EQUAL, JUMP,
  // JZ, JNZ, LOADU8
  fn test_equality_jumps_and_loadu8() {
    // let world = World::new();
    // let mut vm = Tardis::new(&world,);
    // let mut program = vec![OpCode::LOAD_INT as u8, 35, 0, 0, 0, 15,
    // OpCode::LOAD_INT as u8, 36, 0, 0, 0, 10];

    // // TEST: JUMP
    // program.extend_from_slice(&[OpCode::JUMP as u8, 15,],);
    // program.resize(15, 0,);
    // program.extend_from_slice(&[OpCode::JUMP as u8, 30,],);
    // program.resize(30, 0,);

    // // TEST: REQUAL & JZ

    // // Equality check
    // program.extend_from_slice(&[OpCode::NOT_EQUAL as u8, 35, 36,],);

    // // JZ check
    // program.extend_from_slice(&[OpCode::JZ as u8, Tardis::REQ as u8, 40,],);
    // program.resize(40, 0,);

    // // TEST: NOT_REQUAL & JNZ

    // // Equality check
    // program.extend_from_slice(&[OpCode::NOT_EQUAL as u8, 35, 36,],);

    // // JNZ check
    // program.extend_from_slice(&[OpCode::JNZ as u8, Tardis::REQ as u8, 50,],);
    // program.resize(50, 0,);

    // // TEST: LESS & JZ

    // // Equality check
    // program.extend_from_slice(&[OpCode::LESS as u8, 35, 36,],);
    // assert_eq!(program[50], OpCode::LESS as u8);

    // // Jump check
    // program.extend_from_slice(&[OpCode::JZ as u8, Tardis::REQ as u8, 60,],);
    // program.resize(60, 0,);

    // // TEST: GREATER & JNZ

    // // Equality check
    // program.extend_from_slice(&[OpCode::GREATER as u8, 35, 36,],);

    // // Jump check
    // program.extend_from_slice(&[OpCode::JNZ as u8, Tardis::REQ as u8, 70,],);
    // program.resize(70, 0,);

    // // TEST: GREATER_REQUAL & JNZ

    // // Equality check
    // program.extend_from_slice(&[OpCode::GREATER_REQUAL as u8, 35, 36,],);

    // // Jump check
    // program.extend_from_slice(&[OpCode::JNZ as u8, Tardis::REQ as u8, 80,],);
    // program.resize(80, 0,);

    // // TEST: LESS_REQUAL & JZ

    // // Equality check
    // program.extend_from_slice(&[OpCode::LESS_REQUAL as u8, 35, 36,],);

    // // Jump check
    // program.extend_from_slice(&[OpCode::JZ as u8, Tardis::REQ as u8, 90,],);
    // program.resize(90, 0,);

    // // Calculation to test this is reached
    // program.extend_from_slice(&[OpCode::LOAD_U8 as u8, 35, 9,],);
    // program.extend_from_slice(&[OpCode::LOAD_U8 as u8, 36, 14,],);

    // // Halt
    // program.extend_from_slice(&[0,],);

    // vm.load(program,);
    // vm.run();

    // assert_eq!(vm.registers[35], 9);
    // assert_eq!(vm.registers[36], 14);
  }

  #[test]
  #[rustfmt::skip]
  fn test_opcode_mult() {
    let world = World::new();
    let mut vm = Tardis::new(&world,);

    // Test positive ints
    let program = vec![
      OpCode::LOAD_INT as u8, 16, 0, 0, 0, 3,
      OpCode::LOAD_INT as u8, 17, 0, 0, 0, 7,
      OpCode::MULT as u8, 16, 17,
      OpCode::HLT as u8,
    ];

    vm.load(program,);
    vm.run();
    assert_eq!(vm.registers[Tardis::RAX], 21);

    // Reset the vm
    vm.clear();

    // Test negative ints
    let program = vec![
      OpCode::LOAD_INT as u8, 16, 255, 255, 255, 253,
      OpCode::LOAD_INT as u8, 17, 0, 0, 0, 7,
      OpCode::MULT as u8, 16, 17,
      OpCode::HLT as u8,
    ];

    vm.load(program,);
    vm.run();
    assert_eq!(vm.registers[Tardis::RAX] as i32, -21);
  }

  #[test]
  #[rustfmt::skip]
  fn test_opcode_mult_ri() {
    let world = World::new();
    let mut vm = Tardis::new(&world,);

    // Test positive ints
    let program = vec![
      OpCode::LOAD_INT as u8, 16, 0, 0, 0, 3, 
      OpCode::MULT_RI as u8, 16, 0, 0, 0, 7, 
      OpCode::HLT as u8
    ];

    vm.load(program,);
    vm.run();
    assert_eq!(vm.registers[Tardis::RAX], 21);

    // Reset the vm
    vm.clear();

    // Test negative ints
    let program = vec![
      OpCode::LOAD_INT as u8, 16, 255, 255, 255, 253,
      OpCode::MULT_RI as u8, 16, 0, 0, 0, 7,
      OpCode::HLT as u8,
    ];

    vm.load(program,);
    vm.run();
    assert_eq!(vm.registers[Tardis::RAX] as i32, -21);
  }

  #[test]
  #[rustfmt::skip]
  fn test_opcode_mult_ii() {
    let world = World::new();
    let mut vm = Tardis::new(&world,);
    let program = vec![
      OpCode::MULT_II as u8, 0, 0, 0, 3, 0, 0, 0, 7, 
      OpCode::HLT as u8
    ];

    vm.load(program,);
    vm.run();
    assert_eq!(vm.registers[Tardis::RAX], 21);

    // Reset the vm
    vm.clear();

    // Test negative ints
    let program = vec![
      OpCode::MULT_II as u8, 255, 255, 255, 253, 0, 0, 0, 7,
      OpCode::HLT as u8
    ];

    vm.load(program,);
    vm.run();
    assert_eq!(vm.registers[Tardis::RAX] as i32, -21);
  }

  #[test]
  #[rustfmt::skip]
  fn test_opcode_div() {
    let world = World::new();
    let mut vm = Tardis::new(&world,);

    // Test positive ints
    let program = vec![
      OpCode::LOAD_INT as u8, 16, 0, 0, 0, 38,
      OpCode::LOAD_INT as u8,17, 0, 0, 0, 2,
      OpCode::DIV as u8, 16, 17,
      OpCode::HLT as u8,
    ];

    vm.load(program,);
    vm.run();
    assert_eq!(vm.registers[Tardis::RAX], 19);

    // Reset the vm
    vm.clear();

    // Test negative ints
    let program = vec![
      OpCode::LOAD_INT as u8, 16, 0, 0, 0, 38,
      OpCode::LOAD_INT as u8, 17, 255, 255, 255, 254,
      OpCode::DIV as u8, 16, 17,
      OpCode::HLT as u8,
    ];
    vm.load(program,);
    vm.run();
    assert_eq!(vm.registers[Tardis::RAX] as i32, -19);
  }

  #[test]
  #[rustfmt::skip]
  fn test_opcode_div_ri() {
    let world = World::new();
    let mut vm = Tardis::new(&world,);
    
    // Test positive ints
    let program = vec![
      OpCode::LOAD_INT as u8, 16, 0, 0, 0, 38, 
      OpCode::DIV_RI as u8, 16, 0, 0, 0, 2, 
      OpCode::HLT as u8
    ];

    vm.load(program,);
    vm.run();
    assert_eq!(vm.registers[Tardis::RAX], 19);

    // Reset the vm
    vm.clear();

    // Test negative ints
    let program = vec![
      OpCode::LOAD_INT as u8, 16, 0, 0, 0, 38,
      OpCode::DIV_RI as u8, 16, 255, 255, 255, 254,
      OpCode::HLT as u8,
    ];
    vm.load(program,);
    vm.run();
    assert_eq!(vm.registers[Tardis::RAX] as i32, -19);
  }

  #[test]
  #[rustfmt::skip]
  fn test_opcode_div_ii() {
    let world = World::new();
    let mut vm = Tardis::new(&world,);
    
    // Test positive ints
    let program = vec![
      OpCode::DIV_II as u8, 0, 0, 0, 38, 0, 0, 0, 2, 
      OpCode::HLT as u8
    ];

    vm.load(program,);
    vm.run();
    assert_eq!(vm.registers[Tardis::RAX], 19);

    // Reset the vm
    vm.clear();

    // Test negative ints
    let program = vec![
      OpCode::DIV_II as u8, 0, 0, 0, 38, 255, 255, 255, 254,
      OpCode::HLT as u8,
    ];
    vm.load(program,);
    vm.run();
    assert_eq!(vm.registers[Tardis::RAX] as i32, -19);
  }

  #[test]
  #[rustfmt::skip]
  fn test_opcode_add() {
    let world = World::new();
    let mut vm = Tardis::new(&world,);
    
    // Test positive ints
    let program = vec![
      OpCode::LOAD_INT as u8, 16, 0, 0, 0, 38,
      OpCode::LOAD_INT as u8, 17, 0, 0, 0, 2,
      OpCode::ADD as u8, 16, 17,
      OpCode::HLT as u8,
    ];

    vm.load(program,);
    vm.run();
    assert_eq!(vm.registers[Tardis::RAX], 40);

    // Reset the vm
    vm.clear();

    // Test negative ints
    let program = vec![
      OpCode::LOAD_INT as u8, 16, 0, 0, 0, 38,
      OpCode::LOAD_INT as u8, 17, 255, 255, 255, 254,
      OpCode::ADD as u8, 16, 17,
      OpCode::HLT as u8,
    ];
    vm.load(program,);
    vm.run();
    assert_eq!(vm.registers[Tardis::RAX] as i32, 36);
  }

  #[test]
  #[rustfmt::skip]
  fn test_opcode_add_ri() {
    let world = World::new();
    let mut vm = Tardis::new(&world,);
    
    // Test positive ints
    let program = vec![
      OpCode::LOAD_INT as u8, 16, 0, 0, 0, 38, 
      OpCode::ADD_RI as u8, 16, 0, 0, 0, 2, 
      OpCode::HLT as u8
    ];

    vm.load(program,);
    vm.run();
    assert_eq!(vm.registers[Tardis::RAX], 40);

    // Reset the vm
    vm.clear();

    // Test negative ints
    let program = vec![
      OpCode::LOAD_INT as u8, 16, 0, 0, 0, 38,
      OpCode::ADD_RI as u8, 16, 255, 255, 255, 254,
      OpCode::HLT as u8,
    ];
    vm.load(program,);
    vm.run();
    assert_eq!(vm.registers[Tardis::RAX] as i32, 36);
  }

  #[test]
  #[rustfmt::skip]
  fn test_opcode_add_ii() {
    let world = World::new();
    let mut vm = Tardis::new(&world,);
    
    // Test positive ints
    let program = vec![
      OpCode::ADD_II as u8, 0, 0, 0, 38, 0, 0, 0, 2, 
      OpCode::HLT as u8
    ];

    vm.load(program,);
    vm.run();
    assert_eq!(vm.registers[Tardis::RAX], 40);

    // Reset the vm
    vm.clear();

    // Test negative ints
    let program = vec![
      OpCode::ADD_II as u8, 0, 0, 0, 38, 255, 255, 255, 254,
      OpCode::HLT as u8,
    ];
    vm.load(program,);
    vm.run();
    assert_eq!(vm.registers[Tardis::RAX] as i32, 36);
  }

  #[test]
  #[rustfmt::skip]
  fn test_opcode_sub() {
    let world = World::new();
    let mut vm = Tardis::new(&world,);
    
    // Test positive ints
    let program = vec![
      OpCode::LOAD_INT as u8, 16, 0, 0, 0, 38,
      OpCode::LOAD_INT as u8, 17, 0, 0, 0, 2,
      OpCode::SUB as u8, 16, 17,
      OpCode::HLT as u8,
    ];

    vm.load(program,);
    vm.run();
    assert_eq!(vm.registers[Tardis::RAX], 36);

    // Reset the vm
    vm.clear();

    // Test negative ints
    let program = vec![
      OpCode::LOAD_INT as u8, 16, 0, 0, 0, 38,
      OpCode::LOAD_INT as u8, 17, 255, 255, 255, 254,
      OpCode::SUB as u8, 16, 17,
      OpCode::HLT as u8,
    ];
    vm.load(program,);
    vm.run();
    assert_eq!(vm.registers[Tardis::RAX] as i32, 40);
  }

  #[test]
  #[rustfmt::skip]
  fn test_opcode_sub_ri() {
    let world = World::new();
    let mut vm = Tardis::new(&world,);
    
    // Test positive ints
    let program = vec![
      OpCode::LOAD_INT as u8, 16, 0, 0, 0, 38, 
      OpCode::SUB_RI as u8, 16, 0, 0, 0, 2, 
      OpCode::HLT as u8
    ];

    vm.load(program,);
    vm.run();
    assert_eq!(vm.registers[Tardis::RAX], 36);

    // Reset the vm
    vm.clear();

    // Test negative ints
    let program = vec![
      OpCode::LOAD_INT as u8, 16, 0, 0, 0, 38,
      OpCode::SUB_RI as u8, 16, 255, 255, 255, 254,
      OpCode::HLT as u8,
    ];
    vm.load(program,);
    vm.run();
    assert_eq!(vm.registers[Tardis::RAX] as i32, 40);
  }

  #[test]
  #[rustfmt::skip]
  fn test_opcode_sub_ii() {
    let world = World::new();
    let mut vm = Tardis::new(&world,);
    
    // Test positive ints
    let program = vec![
      OpCode::SUB_II as u8, 0, 0, 0, 38, 0, 0, 0, 2, 
      OpCode::HLT as u8
    ];

    vm.load(program,);
    vm.run();
    assert_eq!(vm.registers[Tardis::RAX], 36);

    // Reset the vm
    vm.clear();

    // Test negative ints
    let program = vec![
      OpCode::SUB_II as u8, 0, 0, 0, 38, 255, 255, 255, 254,
      OpCode::HLT as u8,
    ];
    vm.load(program,);
    vm.run();
    assert_eq!(vm.registers[Tardis::RAX] as i32, 40);
  }

  #[test]
  #[rustfmt::skip]
  fn test_opcode_pow() {
    let world = World::new();
    let mut vm = Tardis::new(&world,);
    
    // Test positive ints
    let program = vec![
      OpCode::LOAD_INT as u8, 16, 0, 0, 0, 2,
      OpCode::LOAD_INT as u8, 17, 0, 0, 0, 4,
      OpCode::POW as u8, 16, 17,
      OpCode::HLT as u8,
    ];

    vm.load(program,);
    vm.run();
    assert_eq!(vm.registers[Tardis::RAX], 16);

    // Reset the vm
    vm.clear();

    // Test negative ints
    let program = vec![
      OpCode::LOAD_INT as u8, 16, 0, 0, 0, 4,
      OpCode::LOAD_INT as u8, 17, 255, 255, 255, 254,
      OpCode::POW as u8, 16, 17,
      OpCode::HLT as u8,
    ];
    vm.load(program,);
    vm.run();
    assert_eq!(vm.registers[Tardis::RAX] as i32, 0);
  }

  #[test]
  #[rustfmt::skip]
  fn test_opcode_pow_ri() {   
    let world = World::new();
    let mut vm = Tardis::new(&world,);
    
    // Test positive ints
    let program = vec![
      OpCode::LOAD_INT as u8, 16, 0, 0, 0, 2, 
      OpCode::POW_RI as u8, 16, 0, 0, 0, 4, 
      OpCode::HLT as u8
    ];

    vm.load(program,);
    vm.run();
    assert_eq!(vm.registers[Tardis::RAX], 16);

    // Reset the vm
    vm.clear();

    // Test negative ints
    let program = vec![
      OpCode::LOAD_INT as u8, 16, 0, 0, 0, 4,
      OpCode::POW_RI as u8, 16, 255, 255, 255, 254,
      OpCode::HLT as u8,
    ];
    vm.load(program,);
    vm.run();
    assert_eq!(vm.registers[Tardis::RAX] as i32, 0);
  }

  #[test]
  #[rustfmt::skip]
  fn test_opcode_pow_ii() {
    let world = World::new();
    let mut vm = Tardis::new(&world,);
    
    // Test positive ints
    let program = vec![
      OpCode::POW_II as u8, 0, 0, 0, 2, 0, 0, 0, 4, 
      OpCode::HLT as u8
    ];

    vm.load(program,);
    vm.run();
    assert_eq!(vm.registers[Tardis::RAX], 16);

    // Reset the vm
    vm.clear();

    // Test negative ints
    let program = vec![
      OpCode::POW_II as u8, 0, 0, 0, 38, 255, 255, 255, 254,
      OpCode::HLT as u8,
    ];
    vm.load(program,);
    vm.run();
    assert_eq!(vm.registers[Tardis::RAX] as i32, 0);
  }

  #[test]
  #[rustfmt::skip]
  fn test_opcode_f_mult() {
    let world = World::new();
    let mut vm = Tardis::new(&world,);
    let program = vec![
      OpCode::LOAD_FLOAT as u8, 16, 65, 72, 0, 0,
      OpCode::LOAD_FLOAT as u8, 17, 64, 240, 0, 0,
      OpCode::F_MULT as u8, 16, 17,
      OpCode::HLT as u8,
    ];

    vm.load(program,);
    vm.run();
    assert_eq!(f32::from_bits(vm.registers[Tardis::RAX]), 12.5 * 7.5);
  }

  #[test]
  #[rustfmt::skip]
  fn test_opcode_f_mult_ri() {
    let world = World::new();
    let mut vm = Tardis::new(&world,);
    let program = vec![
      OpCode::LOAD_FLOAT as u8, 16, 65, 72, 0, 0,
      OpCode::F_MULT_RI as u8, 16, 64, 240, 0, 0,
      OpCode::HLT as u8,
    ];

    vm.load(program,);
    vm.run();
    assert_eq!(f32::from_bits(vm.registers[Tardis::RAX]), 12.5 * 7.5);
  }

  #[test]
  #[rustfmt::skip]
  fn test_opcode_f_mult_ii() {
    let world = World::new();
    let mut vm = Tardis::new(&world,);
    let program = vec![
      OpCode::F_MULT_II as u8, 65, 72, 0, 0, 64, 240, 0, 0,
      OpCode::HLT as u8,
    ];

    vm.load(program,);
    vm.run();
    assert_eq!(f32::from_bits(vm.registers[Tardis::RAX]), 12.5 * 7.5);
  }

  #[test]
  #[rustfmt::skip]
  fn test_opcode_f_div() {
    let world = World::new();
    let mut vm = Tardis::new(&world,);
    let program = vec![
      OpCode::LOAD_FLOAT as u8, 16, 65, 72, 0, 0,
      OpCode::LOAD_FLOAT as u8, 17, 64, 240, 0, 0,
      OpCode::F_DIV as u8, 16, 17,
      OpCode::HLT as u8,
    ];

    vm.load(program,);
    vm.run();
    assert_eq!(f32::from_bits(vm.registers[Tardis::RAX]), 12.5 / 7.5);
  }

  #[test]
  #[rustfmt::skip]
  fn test_opcode_f_div_ri() {
    let world = World::new();
    let mut vm = Tardis::new(&world,);
    let program = vec![
      OpCode::LOAD_FLOAT as u8, 16, 65, 72, 0, 0,
      OpCode::F_DIV_RI as u8, 16, 64, 240, 0, 0,
      OpCode::HLT as u8,
    ];

    vm.load(program,);
    vm.run();
    assert_eq!(f32::from_bits(vm.registers[Tardis::RAX]), 12.5 / 7.5);
  }

  #[test]
  #[rustfmt::skip]
  fn test_opcode_f_div_ii() {
    let world = World::new();
    let mut vm = Tardis::new(&world,);
    let program = vec![
      OpCode::F_DIV_II as u8, 65, 72, 0, 0, 64, 240, 0, 0,
      OpCode::HLT as u8,
    ];

    vm.load(program,);
    vm.run();
    assert_eq!(f32::from_bits(vm.registers[Tardis::RAX]), 12.5 / 7.5);
  }

  #[test]
  #[rustfmt::skip]
  fn test_opcode_f_add() {
    let world = World::new();
    let mut vm = Tardis::new(&world,);
    let program = vec![
      OpCode::LOAD_FLOAT as u8, 16, 65, 72, 0, 0,
      OpCode::LOAD_FLOAT as u8, 17, 64, 240, 0, 0,
      OpCode::F_ADD as u8, 16, 17,
      OpCode::HLT as u8,
    ];

    vm.load(program,);
    vm.run();
    assert_eq!(f32::from_bits(vm.registers[Tardis::RAX]), 12.5 + 7.5);
  }

  #[test]
  #[rustfmt::skip]
  fn test_opcode_f_add_ri() {
    let world = World::new();
    let mut vm = Tardis::new(&world,);
    let program = vec![
      OpCode::LOAD_FLOAT as u8, 16, 65, 72, 0, 0,
      OpCode::F_ADD_RI as u8, 16, 64, 240, 0, 0,
      OpCode::HLT as u8,
    ];

    vm.load(program,);
    vm.run();
    assert_eq!(f32::from_bits(vm.registers[Tardis::RAX]), 12.5 + 7.5);
  }

  #[test]
  #[rustfmt::skip]
  fn test_opcode_f_add_ii() {
    let world = World::new();
    let mut vm = Tardis::new(&world,);
    let program = vec![
      OpCode::F_ADD_II as u8, 65, 72, 0, 0, 64, 240, 0, 0,
      OpCode::HLT as u8,
    ];

    vm.load(program,);
    vm.run();
    assert_eq!(f32::from_bits(vm.registers[Tardis::RAX]), 12.5 + 7.5);
  }

  #[test]
  #[rustfmt::skip]
  fn test_opcode_f_sub() {
    let world = World::new();
    let mut vm = Tardis::new(&world,);
    let program = vec![
      OpCode::LOAD_FLOAT as u8, 16, 65, 72, 0, 0,
      OpCode::LOAD_FLOAT as u8, 17, 64, 240, 0, 0,
      OpCode::F_SUB as u8, 16, 17,
      OpCode::HLT as u8,
    ];

    vm.load(program,);
    vm.run();
    assert_eq!(f32::from_bits(vm.registers[Tardis::RAX]), 12.5 - 7.5);
  }

  #[test]
  #[rustfmt::skip]
  fn test_opcode_f_sub_ri() {
    let world = World::new();
    let mut vm = Tardis::new(&world,);
    let program = vec![
      OpCode::LOAD_FLOAT as u8, 16, 65, 72, 0, 0,
      OpCode::F_SUB_RI as u8, 16, 64, 240, 0, 0,
      OpCode::HLT as u8,
    ];

    vm.load(program,);
    vm.run();
    assert_eq!(f32::from_bits(vm.registers[Tardis::RAX]), 12.5 - 7.5);
  }

  #[test]
  #[rustfmt::skip]
  fn test_opcode_f_sub_ii() {
    let world = World::new();
    let mut vm = Tardis::new(&world,);
    let program = vec![
      OpCode::F_POW_II as u8, 65, 72, 0, 0, 64, 240, 0, 0,
      OpCode::HLT as u8,
    ];

    vm.load(program,);
    vm.run();
    assert_eq!(f32::from_bits(vm.registers[Tardis::RAX]), 12.5_f32.powf(7.5));
  }

  #[test]
  #[rustfmt::skip]
  fn test_opcode_f_pow() {
    let world = World::new();
    let mut vm = Tardis::new(&world,);
    
    // Test positive exponentials
    let program = vec![
      OpCode::LOAD_FLOAT as u8, 16, 65, 72, 0, 0,
      OpCode::LOAD_FLOAT as u8, 17, 64, 240, 0, 0,
      OpCode::F_POW as u8, 16, 17,
      OpCode::HLT as u8,
    ];

    vm.load(program,);
    vm.run();
    assert_eq!(f32::from_bits(vm.registers[Tardis::RAX]), 12.5_f32.powf(7.5));

    // Reset the vm
    vm.clear();

    // Test negative exponentials
    let program = vec![
      OpCode::LOAD_FLOAT as u8, 16, 65, 72, 0, 0,
      OpCode::LOAD_FLOAT as u8, 17, 192, 240, 0, 0,
      OpCode::F_POW as u8, 16, 17,
      OpCode::HLT as u8,
    ];

    
    vm.load(program,);
    vm.run();
    assert_eq!(f32::from_bits(vm.registers[Tardis::RAX]), 12.5_f32.powf(-7.5));

    // Reset the vm
    vm.clear();

    // Test fractional exponentials
    let program = vec![
    OpCode::LOAD_FLOAT as u8, 16, 65, 72, 0, 0,
    OpCode::LOAD_FLOAT as u8, 17, 63, 0, 0, 0,
    OpCode::F_POW as u8, 16, 17,
    OpCode::HLT as u8,
  ];

  vm.load(program,);
  vm.run();
  assert_eq!(f32::from_bits(vm.registers[Tardis::RAX]), 12.5_f32.powf(0.5));
  }

  #[test]
  #[rustfmt::skip]
  fn test_opcode_f_pow_ri() {
    let world = World::new();
    let mut vm = Tardis::new(&world,);
    
    // Test positive exponentials
    let program = vec![
      OpCode::LOAD_FLOAT as u8, 16, 65, 72, 0, 0,
      OpCode::F_POW_RI as u8, 16, 64, 240, 0, 0,
      OpCode::HLT as u8,
    ];

    vm.load(program,);
    vm.run();
    assert_eq!(f32::from_bits(vm.registers[Tardis::RAX]), 12.5_f32.powf(7.5));

    // Reset the vm
    vm.clear();

    // Test negative exponentials
    let program = vec![
      OpCode::LOAD_FLOAT as u8, 16, 65, 72, 0, 0,
      OpCode::F_POW_RI as u8, 16, 192, 240, 0, 0,
      OpCode::HLT as u8,
    ];

    vm.load(program,);
    vm.run();
    assert_eq!(f32::from_bits(vm.registers[Tardis::RAX]), 12.5_f32.powf(-7.5));

    // Reset the vm
    vm.clear();

    // Test fractional exponentials
    let program = vec![
    OpCode::LOAD_FLOAT as u8, 16, 65, 72, 0, 0,
    OpCode::F_POW_RI as u8, 16, 63, 0, 0, 0,
    OpCode::HLT as u8,
  ];

  vm.load(program,);
  vm.run();
  assert_eq!(f32::from_bits(vm.registers[Tardis::RAX]), 12.5_f32.powf(0.5));
  }

  #[test]
  #[rustfmt::skip]
  fn test_opcode_f_pow_ii() {
    let world = World::new();
    let mut vm = Tardis::new(&world,);
    
    // Test positive exponentials
    let program = vec![
      OpCode::F_POW_II as u8, 65, 72, 0, 0, 64, 240, 0, 0,
      OpCode::HLT as u8,
    ];

    vm.load(program,);
    vm.run();
    assert_eq!(f32::from_bits(vm.registers[Tardis::RAX]), 12.5_f32.powf(7.5));

    // Reset the vm
    vm.clear();

    // Test negative exponentials
    let program = vec![
      OpCode::F_POW_II as u8, 65, 72, 0, 0, 192, 240, 0, 0,
      OpCode::HLT as u8,
    ];

    vm.load(program,);
    vm.run();
    assert_eq!(f32::from_bits(vm.registers[Tardis::RAX]), 12.5_f32.powf(-7.5));

    // Reset the vm
    vm.clear();

    // Test fractional exponentials
    let program = vec![
    OpCode::F_POW_II as u8, 65, 72, 0, 0, 63, 0, 0, 0,
    OpCode::HLT as u8,
  ];

  vm.load(program,);
  vm.run();
  assert_eq!(f32::from_bits(vm.registers[Tardis::RAX]), 12.5_f32.powf(0.5));
  }

  #[test]
  #[rustfmt::skip]
  // TESTS: CALL and RETURN
  fn test_function_calling() {
    let world = World::new();
    let mut vm = Tardis::new(&world,);
    let mut program = vec![
      // Load 2 into R16 as the number to be cubed
      OpCode::LOAD_INT as u8, 16, 0, 0, 0, 3,
      // Move the value into the argument registers
      OpCode::MOVE as u8, 0, 16,
      // Call the cube function
      OpCode::CALL as u8, 50,
      // End the program after the cube function returns
      OpCode::HLT as u8,
    ];

    // Pad the program so the function def is at R50
    program.resize(50, 0,);

    // Cube Function Def:
    // A fancy cube function which uses a while loop to cube
    // a number and return the result
    program.extend_from_slice(&[
      // Set the number of loops
      OpCode::LOAD_U8 as u8, Tardis::RCX as u8, 2,
      
      //LOOP

      // Copy the number into R1
      OpCode::MOVE as u8, 1, 0,
      // Multiply R0 by R1
      OpCode::MULT as u8, 0, 1,
      // Move the result of the multiplication into R0
      OpCode::MOVE as u8, 0, Tardis::RAX as u8,
      // Decrement the number of remaining loops and move the result into RCS
      OpCode::SUB_RI as u8, Tardis::RCX as u8, 0, 0, 0, 1,
      OpCode::MOVE as u8, Tardis::RCX as u8, Tardis::RAX as u8,
      // If the number of remaining loops is not 0, loop again
      OpCode::JNZ as u8, Tardis::RCX as u8, 56,
      // If the number of remaining loops is 0, return
      OpCode::RETURN as u8,
    ],);

    vm.load(program,);
    vm.run();

    assert_eq!(vm.registers[0], 27);
  }

  #[test]
  #[rustfmt::skip]
  // TESTS: SYS_CALL
  fn test_syscalling() {
    // Define the external methods
    struct Health {
      max:i32,
      current:i32,
    }

    fn get_max_health_inner(world:&World, target:usize,) -> i32 {
      world.get_component::<Health>(target,).unwrap().max
    }

    fn get_max_health(vm:&mut Tardis,) {
      let target = vm.registers[0] as usize;
      vm.registers[0] = get_max_health_inner(vm.world, target,) as u32;
    }

    fn deal_true_damage_inner(world:&World, target:usize, amount:i32){
      let health = world.get_component_mut::<Health>(target,).unwrap();
      health.current -= amount;
    }
    
    fn deal_true_damage(vm:&mut Tardis,){
      let target = vm.registers[0] as usize;
      let amount = vm.registers[1] as i32;
      deal_true_damage_inner(vm.world, target, amount,)
    }

    // Set up the world
    let mut world = World::new();
    world.register_component::<Health>();
    world.create_entity().with_component(Health{ max: 1005, current: 74 }).unwrap();


    // Create the vm and register the external function
    let mut vm = Tardis::new(&world,);
    vm.register_external_function(get_max_health,);
    vm.register_external_function(deal_true_damage,);

    // Get the health
    let program = vec![
      OpCode::LOAD_INT as u8, 0, 0, 0, 0, 0,
      OpCode::SYS_CALL as u8, 0,
      OpCode::HLT as u8,
    ];

    vm.load(program,);
    vm.run();

    assert_eq!(vm.registers[0], 1005);
    
    // Reset the vm
    vm.clear();

    // Deal damage
    let program = vec![
      OpCode::LOAD_INT as u8, 0, 0, 0, 0, 0,
      OpCode::LOAD_INT as u8, 1, 0, 0, 0, 70,
      OpCode::SYS_CALL as u8, 1,
      OpCode::HLT as u8,
    ];

    vm.load(program,);
    vm.run();

    assert_eq!(world.get_component_mut::<Health>(0,).unwrap().current, 4);
  }
}
