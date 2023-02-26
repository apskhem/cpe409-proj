use crate::instruction::Opcode;
use crate::store::Store;

pub struct State {
  pub mem: Store<u32>,
  pub regs: Store<i32>,
  pub pc: i32,
  pub opcode: Option<Opcode>
}
