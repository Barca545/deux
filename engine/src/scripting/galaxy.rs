// To Do:
// - Document OpCodes
// -Evaluate if the SET opcode is needed
// -Make the opcodes constants?

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::mem::transmute;

//Define opcodes
#[allow(non_camel_case_types)]
#[allow(unused)]
#[derive(FromPrimitive, Debug, PartialEq)]
enum OpCode {
  /// Halt the execution of code.
  HLT,
  /// Load an integer into the target register.
  LOAD_INT,
  /// Load a float into the target register.
  LOAD_FLOAT,
  /// Move a value between registers.
  MOVE,
  /// Perform multiplication on the values in two registers.
  /// Store the result in [`Tardis`]'s `eax` register.
  MULT,
  /// Perform division on the values in two registers.
  /// Store the result in [`Tardis`]'s `eax` register.
  DIV,
  /// Perform addition on the values in two registers.
  /// Store the result in [`Tardis`]'s `eax` register.
  ADD,
  /// Perform subtraction on the values in two registers.
  /// Store the result in [`Tardis`]'s `eax` register.
  SUB,
  /// Exponentiate a by b.
  /// Store the result in [`Tardis`]'s `eax` register.
  POW,
  /// Perform an == operation on the values in two registers.
  /// Store the result in [`Tardis`]'s `eq` register.
  EQUAL,
  /// Perform an != operation on the values in two registers.
  /// Store the result in [`Tardis`]'s `eq` register.
  NOT_EQUAL,
  /// Perform an > operation on the values in two registers.
  /// Store the result in [`Tardis`]'s `eq` register.
  GREATER,
  /// Perform an < operation on the values in two registers.
  /// Store the result in [`Tardis`]'s `eq` register.
  LESS,
  /// Perform an >= operation on the values in two registers.
  /// Store the result in [`Tardis`]'s `eq` register.
  GREATER_EQUAL,
  /// Perform an <= operation on the values in two registers.
  /// Store the result in [`Tardis`]'s `eq` register.
  LESS_EQUAL,
  /// Unconditional jump.
  JUMP,
  /// Jump if zero.
  JZ,
  /// Jump if not zero.
  JNZ,
  /// Load 1 byte into a register.
  LOADU8,
  /// Load 4 bytes into a register.
  LOADU32,
  CALL,
  RETURN,
  SYS_CALL
}

#[derive(PartialEq)]
pub enum LoopControl {
  Continue,
  Break
}

//Define arguments

///VM for running Galaxy bytecode.
#[derive(Default)]
#[allow(unused)]
struct Tardis {
  /// The program counter indicates the next instruction to execute.
  pc:usize,
  /// Program bytecode.
  program:Vec<u8>,
  /// Register which stores the result of the last equality operation.
  eq:bool,
  /// Link register, holds the return address of the current function call.
  lr:usize,
  /// Accumulator (register), holds the result of arithmetic operations.
  eax:f32,
  /// Counter (register) holds the number of times a process is to be repeated.
  ecx:u32,
  /// Float registers, hold floats.
  float:[f32; 32],
  /// General purpose registers.
  registers:[u32; 10],
  /// Heap memory.
  mem:Vec<u8>
}

#[allow(unused)]
impl Tardis {
  pub fn new() -> Self {
    Tardis::default()
  }

  fn decode(&mut self) -> (OpCode) {
    let op = FromPrimitive::from_u8(self.program[self.pc]).unwrap();
    self.pc += 1;
    (op)
  }

  fn execute(&mut self, op:OpCode) -> LoopControl {
    match op {
      OpCode::HLT => LoopControl::Break,
      OpCode::LOAD_INT => self.LOAD_INT(),
      OpCode::LOAD_FLOAT => self.LOAD_FLOAT(),
      OpCode::MOVE => self.MOVE(),
      OpCode::MULT => self.MULT(),
      OpCode::DIV => self.DIV(),
      OpCode::ADD => self.ADD(),
      OpCode::SUB => self.SUB(),
      OpCode::POW => self.POW(),
      OpCode::EQUAL => self.EQUAL(),
      OpCode::NOT_EQUAL => self.NOT_EQUAL(),
      OpCode::GREATER => self.GREATER(),
      OpCode::LESS => self.LESS(),
      OpCode::GREATER_EQUAL => self.GREATER_EQUAL(),
      OpCode::LESS_EQUAL => self.LESS_EQUAL(),
      OpCode::JUMP => self.JUMP(),
      OpCode::JZ => self.JZ(),
      OpCode::JNZ => self.JNZ(),
      OpCode::LOADU8 => self.LOADU8(),
      OpCode::LOADU32 => self.LOADU32(),
      OpCode::CALL => todo!(),
      OpCode::RETURN => todo!(),
      OpCode::SYS_CALL => todo!()
    }
  }

  pub fn run(&mut self) {
    loop {
      let op = self.decode();
      match self.execute(op) {
        LoopControl::Break => break,
        LoopControl::Continue => continue
      }
    }
  }

  ///Load a program into the [`Tardis`]'s `program` slot.
  pub fn load(&mut self, program:Vec<u8>) {
    self.program = program;
  }

  ///Empty the [`Tardis`]'s `program` slot.
  pub fn clear(&mut self) {
    self.program = Vec::new();
  }

  ///Fetch the next byte.
  fn fetch_u8(&mut self) -> u8 {
    // Fetch the next byte in the program
    let num = self.program[self.pc];
    // Increment the pc
    self.pc += 1;
    num
  }

  ///Fetch the next 4 bytes as a u32.
  fn fetch_u32(&mut self) -> u32 {
    // Fetch the next four bytes in the program as a [u8;4]
    // and transmute them into a u32
    let num = unsafe {
      transmute::<[u8; 4], u32>([
        self.program[self.pc],
        self.program[self.pc + 1],
        self.program[self.pc + 2],
        self.program[self.pc + 3]
      ])
    };
    // Increment the pc
    self.pc += 4;
    num
  }

  ///Fetch the next 4 bytes as a f32.
  fn fetch_f32(&mut self) -> f32 {
    // Fetch the next four bytes in the program as a [u8;4]
    // and transmute them into a f32
    let num = unsafe {
      transmute::<[u8; 4], f32>([
        self.program[self.pc],
        self.program[self.pc + 1],
        self.program[self.pc + 2],
        self.program[self.pc + 3]
      ])
    };
    // Increment the pc
    self.pc += 4;
    num
  }
}

// Opcode implementation block.
#[allow(non_snake_case)]
impl Tardis {
  pub fn LOAD_INT(&mut self) -> LoopControl {
    // Get the arguments
    let register = self.fetch_u8() as usize;
    let int = self.fetch_u32();

    // Perform the operation
    self.registers[register] = int;
    LoopControl::Continue
  }

  pub fn LOAD_FLOAT(&mut self) -> LoopControl {
    // Get the arguments
    let register = self.fetch_u8() as usize;
    let float = self.fetch_f32();

    // Perform the operation
    self.float[register] = float;

    LoopControl::Continue
  }

  fn MOVE(&mut self) -> LoopControl {
    self.registers[self.fetch_u8() as usize] = self.registers[self.fetch_u8() as usize];
    LoopControl::Continue
  }

  fn MULT(&mut self) -> LoopControl {
    // Get the arguments
    let a = self.registers[self.fetch_u8() as usize];
    let b = self.registers[self.fetch_u8() as usize];
    let c = self.fetch_u8() as usize;

    // Perform the operation
    self.registers[c] = a * b;
    LoopControl::Continue
  }

  fn DIV(&mut self) -> LoopControl {
    // Get the arguments
    let a = self.registers[self.fetch_u8() as usize];
    let b = self.registers[self.fetch_u8() as usize];
    let c = self.fetch_u8() as usize;

    // Perform the operation
    self.registers[c] = a / b;
    LoopControl::Continue
  }

  fn ADD(&mut self) -> LoopControl {
    // Get the arguments
    let a = self.registers[self.fetch_u8() as usize];
    let b = self.registers[self.fetch_u8() as usize];
    let c = self.fetch_u8() as usize;

    // Perform the operation
    self.registers[c] = a + b;
    LoopControl::Continue
  }

  fn SUB(&mut self) -> LoopControl {
    // Get the arguments
    let a = self.registers[self.fetch_u8() as usize];
    let b = self.registers[self.fetch_u8() as usize];
    let c = self.fetch_u8() as usize;

    // Perform the operation
    self.registers[c] = a - b;
    LoopControl::Continue
  }

  fn POW(&mut self) -> LoopControl {
    // Get the arguments
    let a = self.registers[self.fetch_u8() as usize];
    let b = self.registers[self.fetch_u8() as usize];
    let c = self.fetch_u8() as usize;

    // Perform the operation
    self.registers[c] = a.pow(b);
    LoopControl::Continue
  }

  fn EQUAL(&mut self) -> LoopControl {
    // Get the arguments
    let a = self.registers[self.fetch_u8() as usize];
    let b = self.registers[self.fetch_u8() as usize];

    // Perform the operation
    self.eq = a == b;
    LoopControl::Continue
  }

  fn NOT_EQUAL(&mut self) -> LoopControl {
    // Get the arguments
    let a = self.registers[self.fetch_u8() as usize];
    let b = self.registers[self.fetch_u8() as usize];

    // Perform the operation
    self.eq = a != b;
    LoopControl::Continue
  }

  fn GREATER(&mut self) -> LoopControl {
    // Get the arguments
    let a = self.registers[self.fetch_u8() as usize];
    let b = self.registers[self.fetch_u8() as usize];

    // Perform the operation
    self.eq = a > b;
    LoopControl::Continue
  }

  fn LESS(&mut self) -> LoopControl {
    // Get the arguments
    let a = self.registers[self.fetch_u8() as usize];
    let b = self.registers[self.fetch_u8() as usize];

    // Perform the operation
    self.eq = a < b;
    LoopControl::Continue
  }

  fn GREATER_EQUAL(&mut self) -> LoopControl {
    // Get the arguments
    let a = self.registers[self.fetch_u8() as usize];
    let b = self.registers[self.fetch_u8() as usize];

    // Perform the operation
    self.eq = a >= b;
    LoopControl::Continue
  }

  fn LESS_EQUAL(&mut self) -> LoopControl {
    // Get the arguments
    let a = self.registers[self.fetch_u8() as usize];
    let b = self.registers[self.fetch_u8() as usize];

    // Perform the operation
    self.eq = a <= b;
    LoopControl::Continue
  }

  fn JUMP(&mut self) -> LoopControl {
    self.pc = self.fetch_u8() as usize;
    LoopControl::Continue
  }

  fn JZ(&mut self) -> LoopControl {
    if self.registers[self.fetch_u8() as usize] == 0 {
      self.pc = self.fetch_u8() as usize;
    }
    LoopControl::Continue
  }

  fn JNZ(&mut self) -> LoopControl {
    if self.registers[self.fetch_u8() as usize] != 0 {
      self.pc = self.fetch_u8() as usize;
    }
    LoopControl::Continue
  }

  fn LOADU8(&mut self) -> LoopControl {
    self.registers[self.fetch_u8() as usize] = self.fetch_u8() as u32;
    LoopControl::Continue
  }

  fn LOADU32(&mut self) -> LoopControl {
    self.registers[self.fetch_u8() as usize] = self.fetch_u32() as u32;
    LoopControl::Continue
  }

  // CALL,
  // RETURN
  // SYS_CALL
}

#[cfg(test)]
mod test {
  use super::Tardis;
  use crate::scripting::galaxy::OpCode;

  #[test]
  fn decode_works() {
    let mut vm = Tardis::new();
    let program = vec![0, 1];
    vm.load(program);
    let op = vm.decode();
    assert_eq!(op, OpCode::HLT);
  }

  #[test]
  fn test_operations() {
    let mut vm = Tardis::new();
    let program = vec![0, 1];
  }
}
