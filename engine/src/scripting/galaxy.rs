// To Do:
// - Document OpCodes
// -Evaluate if the SET opcode is needed

//Define opcodes
#[allow(non_camel_case_types)]
#[allow(unused)]
enum OpCode {
  /// Halt the execution of code.
  HLT,
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
  SET,
  JUMP,
  /// Jump if zero.
  JZ,
  /// Jump if not zero.
  JNZ,
  /// Load 1 byte into a register.
  LOADU8,
  /// Load 2 bytes into a register.
  LOADU16,
  /// Load 4 bytes into a register.
  LOADU32,
  CALL,
  SYS_CALL
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
  eax:u32,
  /// Counter (register) holds the number of times a process is to be repeated.
  ecx:u32,
  /// Float registers, hold floats.
  float:[f32; 32],
  /// General purpose registers.
  registers:[[u8; 4]; 10],
  /// Heap memory.
  mem:Vec<u8>
}

#[allow(unused)]
impl Tardis {
  fn decode(&mut self) {}
  ///Fetch the next byte.
  fn fetch_u8(&mut self) {}
  ///Fetch the next 4 bytes.
  fn fetch_u32(&mut self) {
    //fetch a [u8;4] from the program and transmute it into a u32
  }
  fn eval(&mut self) {}
}
