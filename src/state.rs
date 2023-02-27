use crate::common::ErrMsg;
use crate::instruction::Opcode;
use crate::store::Store;

pub struct State {
  pub mem: Store<u32>,
  pub regs: Store<i32>,
  pub pc: i32,
  pub counter: i32,
  pub opcode: Option<Opcode>
}

impl State {
  pub fn sanitize_regs(self) -> Self {
    Self {
      regs: self.regs.write(0, 0),
      ..self
    }
  }
}

impl State {
  pub fn print(&self) -> Result<(), ErrMsg> {
    println!("@@@");
    println!("state");
    println!("\tpc {}", self.pc);
    println!("\tmemory:");

    if let Some(idx) = self.mem.find_last_non_zero_index() {
      for i in 0..=idx {
        println!("\t\tmemory[ {} ] {}", i, self.mem.at(i)?);
      }
    }

    println!("\tregisters:");

    for i in 0..8usize {
      println!("\t\treg[ {} ] {}", i, self.regs.at(i)?);
    }

    println!("end state");

    Ok(())
  }

  pub fn print_final(&self) -> Result<(), ErrMsg> {
    println!("halted");
    println!("total of {} instructions executed", self.counter);
    println!("final state of machine: ");
    
    self.print()
  }

  pub fn print_mem(&self) -> Result<(), ErrMsg> {
    if let Some(idx) = self.mem.find_last_non_zero_index() {
      for i in 0..=idx {
        println!("memory[ {} ] {}", i, self.mem.at(i)?);
      }
    }

    Ok(())
  }
}