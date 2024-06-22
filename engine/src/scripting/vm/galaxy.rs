use super::errors::VMError;
use nina::world::World;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[no_mangle]
pub extern "C" fn get_health(world:*mut World,) {}

// give the VM a globals field
// store a world pointer in that

// alternatively make the functions into a C ABI and call them from a header?

// To Do:
// - Make the opcodes constants?
// - Add support for arrays
// - Add a better error for if an op is not recognized than just unwrapping
// - Tail call optimization in the compiler?

// Refactor:
// - Instead of to/from bits just use direct transmutes since compilation should
//   take care of the same checks they take care of? I don't think the above is
//   true actually.
// - Should MOVE be renamed copy?

// ok bytecode functions are weird you push the parameters before calling the
// function and then the function uses them figured it out

// the other weird thing is so functions can be reused but the compiler would
// need to like know which registers they expect their arguments to be in

// vs stack based you just pop them onto the stack in the proper order and then
// the function knows ok param 3 is the first thing off the stack param 2 is the
// second, etc ok so I guess I get the theory now

pub struct StackFrame {
  ///The function's return address.
  lr:usize,
  /// Base register of the caller.
  caller_base:usize,
  /// The base register of the function.
  ///
  /// This is the first register the function uses.
  /// All other registers used in the function are calculated from this base +
  /// some offset.
  function_base:usize,
  /// Number of values to copy from the registers starting at the
  /// `function_base` to the registers starting at the `caller_base`.
  return_num:usize,
}

//Define opcodes
#[allow(non_camel_case_types)]
#[allow(unused)]
#[derive(FromPrimitive, Debug, PartialEq, Clone, Copy,)]
pub enum OpCode {
  /// Halt the execution of code.
  HLT,
  /// Takes 2 argument:
  /// - a, the dest register.
  /// - b, the byte.
  ///
  /// Loads b into a.
  LOADU8,
  /// Takes 2 arguments:
  /// - a, the dest register.
  /// - b, the integer.
  ///
  /// Loads b into a.
  LOAD_INT,
  /// Takes 2 arguments:
  /// - a, the dest register.
  /// - b, the float.
  ///
  /// Loads b into a.
  LOAD_FLOAT,
  /// Takes 2 arguments:
  /// - a, the dest register.
  /// - b, the src register.
  ///
  /// Copy the value in b into a.
  MOVE,
  /// Takes 2 arguments:
  /// - a, the integer stored in the register of index a.
  /// - b, the integer stored in the register of index b.
  ///
  /// Store a * b in [`Tardis::EAX`].
  MULT,
  /// Takes 2 arguments:
  /// - a, the integer stored in the register of index a.
  /// - b, the integer stored in the register of index b.
  ///
  /// Store a / b in [`Tardis::EAX`].
  DIV,
  /// Takes 2 arguments:
  /// - a, the integer stored in the register of index a.
  /// - b, the integer stored in the register of index b.
  ///
  /// Store a + b in [`Tardis::EAX`].
  ADD,
  /// Takes 2 arguments:
  /// - a, the integer stored in the register of index a.
  /// - b, the integer stored in the register of index b.
  ///
  /// Store a - b in [`Tardis::EAX`].
  SUB,
  /// Takes 2 arguments:
  /// - a, the integer stored in the register of index a.
  /// - b, the integer stored in the register of index b.
  ///
  /// Store a^b in [`Tardis::EAX`].
  POW,
  /// Takes 2 arguments:
  /// - a, the float stored in the register of index a.
  /// - b, the float stored in the register of index b.
  ///
  /// Store a * b in [`Tardis::EAX`].
  F_MULT,
  /// Takes 2 arguments:
  /// - a, the float stored in the register of index a.
  /// - b, the float stored in the register of index b.
  ///
  /// Store a / b in [`Tardis::EAX`].
  F_DIV,
  /// Takes 2 arguments:
  /// - a, the float stored in the register of index a.
  /// - b, the float stored in the register of index b.
  ///
  /// Store a + b in [`Tardis::EAX`].
  F_ADD,
  /// Takes 2 arguments:
  /// - a, the float stored in the register of index a.
  /// - b, the float stored in the register of index b.
  ///
  /// Store a - b in [`Tardis::EAX`].
  F_SUB,
  /// Takes 2 arguments:
  /// - a, the float stored in the register of index a.
  /// - b, the float stored in the register of index b.
  ///
  /// Store a^b in [`Tardis::EAX`].
  F_POW,
  /// Takes 2 arguments:
  /// - a, the float stored in the register of index a.
  /// - b, the float stored in the register of index b.
  ///
  /// Store the result of a == b in [`Tardis::EQ`].
  EQUAL,
  /// Takes 2 arguments:
  /// - a, first register.
  /// - b, second register.
  ///
  /// Store the result of a != b in [`Tardis::EQ`].
  NOT_EQUAL,
  /// Takes 2 arguments:
  /// - a, first register.
  /// - b, second register.
  ///
  /// Store the result of a > b in [`Tardis::EQ`].
  GREATER,
  /// Takes 2 arguments:
  /// - a, first register.
  /// - b, second register.
  ///
  /// Store the result of a < b in [`Tardis::EQ`].
  LESS,
  /// Takes 2 arguments:
  /// - a, first register.
  /// - b, second register.
  ///
  /// Store the result of a => b in [`Tardis::EQ`].
  GREATER_EQUAL,
  /// Takes 2 arguments:
  /// - a, first register.
  /// - b, second register.
  ///
  /// Store the result of a <= b in [`Tardis::EQ`].
  LESS_EQUAL,
  /// Takes 1 argmumet:
  /// - a, the new `pc`value.
  JUMP,
  /// Takes 2 arguments:
  /// - a, the register to check.
  /// - b, the new `pc`value if a == 0.
  JZ,
  /// Takes 2 arguments:
  /// - a, the register to check.
  /// - b, the new `pc`value if a != 0.
  JNZ,
  /// Takes 5 arguments:
  /// - a, the memory address of the function.
  /// - b, the base register of the function.
  /// - c, the base register of the caller.
  /// - d, the number of args the function accepts.
  /// - e, the number of args the function returns.
  ///
  /// Sets the `pc` to the value of a.
  /// Stores a [`StackFrame`] on the call stack.
  CALL,
  /// Pops the most recent [`StackFrame`].
  ///
  /// Sets the `pc` to the `StackFrame`'s `lr`.
  ///
  /// Moves any return values into the caller's registers.
  RETURN,
  SYS_CALL,
  /// A noop.
  NOOP,
}

#[derive(PartialEq,)]
pub enum LoopControl {
  Continue,
  Break,
}

//Define arguments

///VM for running Galaxy bytecode.
///
/// Tardis is big endian.
#[allow(unused)]
struct Tardis {
  ///Global variables
  globals:Vec<u8,>,
  /// The program counter indicates the next instruction to execute.
  pc:usize,
  /// Program bytecode.
  program:Vec<u8,>,
  /// The VM's registers:
  /// - Float registers range from R0-R31.
  /// - Eq register is R32.
  /// - EAX is R33.
  /// - ECX is R34.
  /// - General purpose registers are R35-R255.
  registers:[u32; 255],
  callstack:Vec<StackFrame,>,
  /// Heap memory.
  mem:Vec<u8,>,
}

#[allow(unused)]
impl Tardis {
  pub fn new() -> Self {
    Tardis {
      globals:Vec::new(),
      pc:0,
      program:Vec::new(),
      registers:[0; 255],
      callstack:Vec::new(),
      mem:Vec::new(),
    }
  }

  fn decode(&mut self,) -> (OpCode) {
    let op_byte = self.program[self.pc];
    let op = FromPrimitive::from_u8(op_byte,).ok_or(VMError::UnrecognizedOpCode(op_byte,),).unwrap();
    self.pc += 1;
    (op)
  }

  fn execute(&mut self, op:OpCode,) -> LoopControl {
    match op {
      OpCode::HLT => LoopControl::Break,
      OpCode::LOADU8 => self.LOADU8(),
      OpCode::LOAD_INT => self.LOAD_INT(),
      OpCode::LOAD_FLOAT => self.LOAD_FLOAT(),
      OpCode::MOVE => self.MOVE(),
      OpCode::MULT => self.MULT(),
      OpCode::DIV => self.DIV(),
      OpCode::ADD => self.ADD(),
      OpCode::SUB => self.SUB(),
      OpCode::POW => self.POW(),
      OpCode::F_MULT => self.F_MULT(),
      OpCode::F_DIV => self.F_DIV(),
      OpCode::F_ADD => self.F_ADD(),
      OpCode::F_SUB => self.F_SUB(),
      OpCode::F_POW => self.F_POW(),
      OpCode::EQUAL => self.EQUAL(),
      OpCode::NOT_EQUAL => self.NOT_EQUAL(),
      OpCode::GREATER => self.GREATER(),
      OpCode::LESS => self.LESS(),
      OpCode::GREATER_EQUAL => self.GREATER_EQUAL(),
      OpCode::LESS_EQUAL => self.LESS_EQUAL(),
      OpCode::JUMP => self.JUMP(),
      OpCode::JZ => self.JZ(),
      OpCode::JNZ => self.JNZ(),
      OpCode::CALL => self.CALL(),
      OpCode::RETURN => self.RETURN(),
      OpCode::SYS_CALL => todo!(),
      OpCode::NOOP => LoopControl::Continue,
    }
  }

  pub fn run(&mut self,) {
    loop {
      let op = self.decode();
      match self.execute(op,) {
        LoopControl::Break => break,
        LoopControl::Continue => continue,
      }
    }
  }

  ///Load a program into the [`Tardis`]'s `program` slot.
  pub fn load(&mut self, program:Vec<u8,>,) {
    self.program = program;
  }

  ///Empty the [`Tardis`]'s `program` slot.
  pub fn clear(&mut self,) {
    self.program = Vec::new();
  }

  ///Fetch the next byte from the `program`.
  fn get_u8(&mut self,) -> u8 {
    // Fetch the next byte in the program
    let num = self.program[self.pc];
    // Increment the pc
    self.pc += 1;
    num
  }

  ///Fetch the next 4 bytes from the `program`as a u32.
  fn get_u32(&mut self,) -> u32 {
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

  ///Fetch the next 4 bytes from the `program` as a f32.
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
impl Tardis {
  ///Index of the register which stores the result of the last equality
  /// operation.
  const EQ:usize = 32;

  /// Index of the accumulator register, holds the result of arithmetic
  /// operations.
  const EAX:usize = 33;

  /// Index of the counter register which holds the number of times a process is
  /// to be repeated.
  const ECX:usize = 34;

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
    let a = self.registers[self.get_u8() as usize];
    let b = self.registers[self.get_u8() as usize];

    // Perform the operation
    self.registers[Self::EAX] = a * b;
    LoopControl::Continue
  }

  fn DIV(&mut self,) -> LoopControl {
    // Get the arguments
    let a = self.registers[self.get_u8() as usize];
    let b = self.registers[self.get_u8() as usize];

    // Perform the operation
    self.registers[Self::EAX] = a / b;
    LoopControl::Continue
  }

  fn ADD(&mut self,) -> LoopControl {
    // Get the arguments
    let a = self.registers[self.get_u8() as usize];
    let b = self.registers[self.get_u8() as usize];

    // Perform the operation
    self.registers[Self::EAX] = a + b;
    LoopControl::Continue
  }

  fn SUB(&mut self,) -> LoopControl {
    // Get the arguments
    let a = self.registers[self.get_u8() as usize];
    let b = self.registers[self.get_u8() as usize];

    // Perform the operation
    self.registers[Self::EAX] = a - b;
    LoopControl::Continue
  }

  fn POW(&mut self,) -> LoopControl {
    // Get the arguments
    let a = self.registers[self.get_u8() as usize];
    let b = self.registers[self.get_u8() as usize];

    // Perform the operation
    self.registers[Self::EAX] = a.pow(b,);
    LoopControl::Continue
  }

  fn F_MULT(&mut self,) -> LoopControl {
    // Get the arguments
    let a = f32::from_bits(self.registers[self.get_u8() as usize],);
    let b = f32::from_bits(self.registers[self.get_u8() as usize],);

    // Perform the operation
    self.registers[Self::EAX] = (a * b).to_bits();
    LoopControl::Continue
  }

  fn F_DIV(&mut self,) -> LoopControl {
    // Get the arguments
    let a = f32::from_bits(self.registers[self.get_u8() as usize],);
    let b = f32::from_bits(self.registers[self.get_u8() as usize],);

    // Perform the operation
    self.registers[Self::EAX] = (a / b).to_bits();
    LoopControl::Continue
  }

  fn F_ADD(&mut self,) -> LoopControl {
    // Get the arguments
    let a = f32::from_bits(self.registers[self.get_u8() as usize],);
    let b = f32::from_bits(self.registers[self.get_u8() as usize],);

    // Perform the operation
    self.registers[Self::EAX] = (a + b).to_bits();
    LoopControl::Continue
  }

  fn F_SUB(&mut self,) -> LoopControl {
    // Get the arguments
    let a = f32::from_bits(self.registers[self.get_u8() as usize],);
    let b = f32::from_bits(self.registers[self.get_u8() as usize],);

    // Perform the operation
    self.registers[Self::EAX] = (a - b).to_bits();
    LoopControl::Continue
  }

  fn F_POW(&mut self,) -> LoopControl {
    // Get the arguments
    let a = f32::from_bits(self.registers[self.get_u8() as usize],);
    let b = f32::from_bits(self.registers[self.get_u8() as usize],);

    // Perform the operation
    self.registers[Self::EAX] = a.powf(b,).to_bits();
    LoopControl::Continue
  }

  fn EQUAL(&mut self,) -> LoopControl {
    // Get the arguments
    let a = f32::from_bits(self.registers[self.get_u8() as usize],);
    let b = f32::from_bits(self.registers[self.get_u8() as usize],);

    // Perform the operation and store it in the registers as a u32
    self.registers[Self::EQ] = (a == b) as u32;
    LoopControl::Continue
  }

  fn NOT_EQUAL(&mut self,) -> LoopControl {
    // Get the arguments
    let a = self.registers[self.get_u8() as usize];
    let b = self.registers[self.get_u8() as usize];

    // Perform the operation
    self.registers[Self::EQ] = (a != b) as u32;
    LoopControl::Continue
  }

  fn GREATER(&mut self,) -> LoopControl {
    // Get the arguments
    let a = self.registers[self.get_u8() as usize];
    let b = self.registers[self.get_u8() as usize];

    // Perform the operation
    self.registers[Self::EQ] = (a > b) as u32;
    LoopControl::Continue
  }

  fn LESS(&mut self,) -> LoopControl {
    // Get the arguments
    let a = self.registers[self.get_u8() as usize];
    let b = self.registers[self.get_u8() as usize];

    // Perform the operation
    self.registers[Self::EQ] = (a < b) as u32;
    LoopControl::Continue
  }

  fn GREATER_EQUAL(&mut self,) -> LoopControl {
    // Get the arguments
    let a = self.registers[self.get_u8() as usize];
    let b = self.registers[self.get_u8() as usize];

    // Perform the operation
    self.registers[Self::EQ] = (a >= b) as u32;
    LoopControl::Continue
  }

  fn LESS_EQUAL(&mut self,) -> LoopControl {
    // Get the arguments
    let a = self.registers[self.get_u8() as usize];
    let b = self.registers[self.get_u8() as usize];

    // Perform the operation
    self.registers[Self::EQ] = (a <= b) as u32;
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

  fn LOADU8(&mut self,) -> LoopControl {
    let register = self.get_u8() as usize;
    let byte = self.get_u8() as u32;
    self.registers[register] = byte;
    LoopControl::Continue
  }

  fn CALL(&mut self,) -> LoopControl {
    let function_address = self.get_u8() as usize;
    // Create a CallFrame and push it to the callstack
    let function_base = self.get_u8() as usize;
    let caller_base = self.get_u8() as usize;
    let num_args = self.get_u8() as usize;
    let return_num = self.get_u8() as usize;

    let frame = StackFrame {
      //Add one to the pc so it skips the return call
      lr:self.pc + 1,
      caller_base,
      function_base,
      return_num,
    };

    self.callstack.push(frame,);

    // Copy the args from the caller reg to the functions registers
    for arg_index in 0..num_args {
      self.registers[function_base + arg_index] = self.registers[caller_base + arg_index];
    }

    // Set the `pc` equal to the base
    self.pc = function_address;

    LoopControl::Continue
  }

  fn RETURN(&mut self,) -> LoopControl {
    // Get the stack frame
    let frame = self.callstack.pop().unwrap();

    // Move the returns from the function to the caller
    for arg_index in 0..frame.return_num {
      self.registers[frame.caller_base + arg_index] = self.registers[frame.function_base + arg_index];
    }

    // Set the pc to the return address
    self.pc = frame.lr;

    LoopControl::Continue
  }

  fn SYS_CALL(&mut self,) -> LoopControl {
    LoopControl::Continue
  }
}

#[cfg(test)]
mod test {
  use super::Tardis;
  use crate::scripting::vm::galaxy::OpCode;

  #[test]
  fn get_u8_and_get_u32_work() {
    let mut vm = Tardis::new();
    vm.load(vec![2, 0, 0, 0, 32],);

    assert_eq!(2, vm.get_u8());
    assert_eq!(32, vm.get_u32());
  }

  #[test]
  fn decode_works() {
    let mut vm = Tardis::new();
    let program = vec![0, 1];
    vm.load(program,);
    let op = vm.decode();
    assert_eq!(op, OpCode::HLT);
  }

  #[test]
  // TESTS: EQUAL, NOT_EQUAL, LESS, GREATER, GREATER_EQUAL, LESS_EQUAL, JUMP, JZ,
  // JNZ, LOADU8
  fn equality_jumps_and_loadu8_work() {
    let mut vm = Tardis::new();
    let mut program = vec![OpCode::LOAD_INT as u8, 35, 0, 0, 0, 15, OpCode::LOAD_INT as u8, 36, 0, 0, 0, 10];

    // TEST: JUMP
    program.extend_from_slice(&[OpCode::JUMP as u8, 15,],);
    program.resize(15, 0,);
    program.extend_from_slice(&[OpCode::JUMP as u8, 30,],);
    program.resize(30, 0,);

    // TEST: EQUAL & JZ

    // Equality check
    program.extend_from_slice(&[OpCode::EQUAL as u8, 35, 36,],);

    // JZ check
    program.extend_from_slice(&[OpCode::JZ as u8, Tardis::EQ as u8, 40,],);
    program.resize(40, 0,);

    // TEST: NOT_EQUAL & JNZ

    // Equality check
    program.extend_from_slice(&[OpCode::NOT_EQUAL as u8, 35, 36,],);

    // JNZ check
    program.extend_from_slice(&[OpCode::JNZ as u8, Tardis::EQ as u8, 50,],);
    program.resize(50, 0,);

    // TEST: LESS & JZ

    // Equality check
    program.extend_from_slice(&[OpCode::LESS as u8, 35, 36,],);
    assert_eq!(program[50], OpCode::LESS as u8);

    // Jump check
    program.extend_from_slice(&[OpCode::JZ as u8, Tardis::EQ as u8, 60,],);
    program.resize(60, 0,);

    // TEST: GREATER & JNZ

    // Equality check
    program.extend_from_slice(&[OpCode::GREATER as u8, 35, 36,],);

    // Jump check
    program.extend_from_slice(&[OpCode::JNZ as u8, Tardis::EQ as u8, 70,],);
    program.resize(70, 0,);

    // TEST: GREATER_EQUAL & JNZ

    // Equality check
    program.extend_from_slice(&[OpCode::GREATER_EQUAL as u8, 35, 36,],);

    // Jump check
    program.extend_from_slice(&[OpCode::JNZ as u8, Tardis::EQ as u8, 80,],);
    program.resize(80, 0,);

    // TEST: LESS_EQUAL & JZ

    // Equality check
    program.extend_from_slice(&[OpCode::LESS_EQUAL as u8, 35, 36,],);

    // Jump check
    program.extend_from_slice(&[OpCode::JZ as u8, Tardis::EQ as u8, 90,],);
    program.resize(90, 0,);

    // Calculation to test this is reached
    program.extend_from_slice(&[OpCode::LOADU8 as u8, 35, 9,],);
    program.extend_from_slice(&[OpCode::LOADU8 as u8, 36, 14,],);

    // Halt
    program.extend_from_slice(&[0,],);

    vm.load(program,);
    vm.run();

    assert_eq!(vm.registers[35], 9);
    assert_eq!(vm.registers[36], 14);
  }

  #[test]
  // TESTS: LOAD_INT, ADD, MULT, DIV, SUB, and POW, MOVE
  fn int_arithmetic_operations_work() {
    let mut vm = Tardis::new();
    let mut program = Vec::new();

    // Load int 5 into R32
    program.extend_from_slice(&[OpCode::LOAD_INT as u8, 35, 0, 0, 0, 5,],);

    // Load int 7 into R32
    program.extend_from_slice(&[OpCode::LOAD_INT as u8, 36, 0, 0, 0, 7,],);

    // Add R32 and R33
    program.extend_from_slice(&[OpCode::ADD as u8, 35, 36,],);

    // Move the addition from EAX into R32
    program.extend_from_slice(&[OpCode::MOVE as u8, 35, Tardis::EAX as u8,],);

    // TEST: MULT

    // Load int 5 into R33
    program.extend_from_slice(&[OpCode::LOAD_INT as u8, 36, 0, 0, 0, 5,],);

    // Mult R32 and R33
    program.extend_from_slice(&[OpCode::MULT as u8, 35, 36,],);

    // Move the multiplication from EAX into R32
    program.extend_from_slice(&[OpCode::MOVE as u8, 35, Tardis::EAX as u8,],);

    // TEST: DIV

    // Load int 10 into R33
    program.extend_from_slice(&[OpCode::LOAD_INT as u8, 36, 0, 0, 0, 10,],);

    // Div R32 and R33
    program.extend_from_slice(&[OpCode::DIV as u8, 35, 36,],);

    // Move the division from EAX into R32
    program.extend_from_slice(&[OpCode::MOVE as u8, 35, Tardis::EAX as u8,],);

    // TEST: SUB

    // Load int 3 into R33
    program.extend_from_slice(&[OpCode::LOAD_INT as u8, 36, 0, 0, 0, 3,],);

    // Sub R32 and R33
    program.extend_from_slice(&[OpCode::SUB as u8, 35, 36,],);

    // Move the multiplication from EAX into R32
    program.extend_from_slice(&[OpCode::MOVE as u8, 35, Tardis::EAX as u8,],);

    // TEST: POW

    // Load int 3 into R33
    program.extend_from_slice(&[OpCode::LOAD_INT as u8, 36, 0, 0, 0, 3,],);

    // Power R32 by R33
    program.extend_from_slice(&[OpCode::POW as u8, 35, 36,],);

    // Move the cube from EAX into R32
    program.extend_from_slice(&[OpCode::MOVE as u8, 35, Tardis::EAX as u8,],);

    program.extend_from_slice(&[0,],);

    vm.load(program,);

    vm.run();

    assert!(vm.registers[Tardis::EAX] == 27 && vm.registers[35] == 27);
  }

  #[test]
  // TESTS: LOAD_FLOAT, F_ADD, F_MULT, F_DIV, F_SUB, and F_POW, MOVE
  fn float_arithmetic_operations_work() {
    let mut vm = Tardis::new();
    let mut program = Vec::new();

    // TEST: F_ADD

    // Load float 12.5 into R0
    program.extend_from_slice(&[OpCode::LOAD_FLOAT as u8, 0,],);
    program.extend_from_slice(&12.5_f32.to_be_bytes(),);

    // Load float 7.5 into R1
    program.extend_from_slice(&[OpCode::LOAD_FLOAT as u8, 1,],);
    program.extend_from_slice(&7.5_f32.to_be_bytes(),);

    // Add R0 and R1
    program.extend_from_slice(&[OpCode::F_ADD as u8, 0, 1,],);

    // Move the addition from EAX into R0
    program.extend_from_slice(&[OpCode::MOVE as u8, 0, Tardis::EAX as u8,],);

    // TEST: F_MULT

    // Load float 5.0 into R1
    program.extend_from_slice(&[OpCode::LOAD_FLOAT as u8, 1,],);
    program.extend_from_slice(&5.0_f32.to_be_bytes(),);

    // Mult R0 and R0
    program.extend_from_slice(&[OpCode::F_MULT as u8, 0, 1,],);

    // Move the addition from EAX into R0
    program.extend_from_slice(&[OpCode::MOVE as u8, 0, Tardis::EAX as u8,],);

    // TEST: F_DIV

    // Load float 2.5 into R1
    program.extend_from_slice(&[OpCode::LOAD_FLOAT as u8, 1,],);
    program.extend_from_slice(&2.5_f32.to_be_bytes(),);

    // Div R0 and R1
    program.extend_from_slice(&[OpCode::F_DIV as u8, 0, 1,],);

    // Move the addition from EAX into R0
    program.extend_from_slice(&[OpCode::MOVE as u8, 0, Tardis::EAX as u8,],);

    // TEST: F_SUB

    // Load int 30.335 into R1
    program.extend_from_slice(&[OpCode::LOAD_FLOAT as u8, 1,],);
    program.extend_from_slice(&30.335_f32.to_be_bytes(),);

    // Sub R0 and R1
    program.extend_from_slice(&[OpCode::F_SUB as u8, 0, 1,],);

    // Move the addition from EAX into R0
    program.extend_from_slice(&[OpCode::MOVE as u8, 0, Tardis::EAX as u8,],);

    // TEST: F_POW

    // Load float 2.3 into R1
    program.extend_from_slice(&[OpCode::LOAD_FLOAT as u8, 1,],);
    program.extend_from_slice(&2.3_f32.to_be_bytes(),);

    // Power R0 by R1
    program.extend_from_slice(&[OpCode::F_POW as u8, 0, 1,],);

    // Move the addition from EAX into R0
    program.extend_from_slice(&[OpCode::MOVE as u8, 0, Tardis::EAX as u8,],);

    // Halt
    program.extend_from_slice(&[0,],);

    vm.load(program,);

    vm.run();

    assert_eq!(f32::from_bits(vm.registers[Tardis::EAX]), 184.48639);
    assert_eq!(f32::from_bits(vm.registers[0]), 184.48639);
  }
}

#[test]
// TESTS: CALL and RETURN
fn function_calling_works() {
  let mut vm = Tardis::new();
  let mut program = vec![];

  // Load 2 into R32 as the number to be cubed
  program.extend_from_slice(&[OpCode::LOAD_INT as u8, 35, 0, 0, 0, 2,],);
  // Call the cube function
  program.extend_from_slice(&[OpCode::CALL as u8, 50, 250, 35, 1, 1,],);
  program.extend_from_slice(&[OpCode::RETURN as u8,],);
  program.resize(50, 0,);

  // Cube Function Def:
  // A fancy cube function which uses a while loop to cube
  // a number and return the result

  // Load the int into the register
  program.extend_from_slice(&[OpCode::MOVE as u8, 251, 250,],);
  program.extend_from_slice(&[OpCode::LOADU8 as u8, Tardis::ECX as u8, 2,],);
  //Beginng of the loop
  program.extend_from_slice(&[OpCode::MULT as u8, 250, 251,],);
  program.extend_from_slice(&[OpCode::MOVE as u8, 250, Tardis::EAX as u8,],);
  program.extend_from_slice(&[OpCode::LOAD_INT as u8, 252, 0, 0, 0, 1,],);
  program.extend_from_slice(&[OpCode::SUB as u8, Tardis::ECX as u8, 252,],);
  program.extend_from_slice(&[OpCode::MOVE as u8, Tardis::ECX as u8, Tardis::EAX as u8,],);
  program.extend_from_slice(&[OpCode::JNZ as u8, Tardis::ECX as u8, 56,],); // End of the loop
  program.extend_from_slice(&[OpCode::RETURN as u8, OpCode::NOOP as u8, OpCode::NOOP as u8,],);

  vm.load(program,);
  vm.run();

  assert_eq!(vm.registers[35], 8);
}
