use crate::state::State;
use crate::instruction::{Instruction, Opcode};
use crate::common::ErrMsg;

pub fn interpret(state: State) -> Result<State, ErrMsg> {
  let State {
    mem,
    regs,
    pc,
    counter,
    ..
  } = state;

  let bin_instr = mem.at(pc as usize)?;

  let Instruction {
    opcode: next_opcode,
    rs1,
    rs2,
    rd,
    offset,
    ..
  } = Instruction::try_from(bin_instr)?;

  let rs1_val = regs.at(rs1 as usize)?;
  let rs2_val = regs.at(rs2 as usize)?;

  let new_state = match next_opcode {
    Opcode::Add => {
      State {
        mem,
        regs: regs.write(rd as usize, rs1_val + rs2_val),
        pc: pc + 1,
        counter: counter + 1,
        opcode: Some(next_opcode)
      }
    },
    Opcode::Nand => {
      State {
        mem,
        regs: regs.write(rd as usize, !(rs1_val & rs2_val)),
        pc: pc + 1,
        counter: counter + 1,
        opcode: Some(next_opcode)
      }
    },
    Opcode::LoadWord => {
      let lw_idx = rs1_val as i16 + offset;
      let mem_val = mem.at(lw_idx as usize)? as i32;

      State {
        mem,
        regs: regs.write(rs2 as usize, mem_val),
        pc: pc + 1,
        counter: counter + 1,
        opcode: Some(next_opcode)
      }
    },
    Opcode::SaveWord => {
      let sw_idx = rs1_val as i16 + offset;
      let reg_val = regs.at(rs2 as usize)? as u32;

      State {
        mem: mem.write(sw_idx as usize, reg_val),
        regs,
        pc: pc + 1,
        counter: counter + 1,
        opcode: Some(next_opcode)
      }
    },
    Opcode::BranchIfEqual => {
      let op_pc = if rs1_val == rs2_val { offset as i32 } else { 0 };

      State {
        mem,
        regs,
        pc: pc + 1 + op_pc,
        counter: counter + 1,
        opcode: Some(next_opcode)
      }
    },
    Opcode::JumpAndLinkRegister => {
      State {
        mem,
        regs: regs.write(rs2 as usize, pc + 1),
        pc: rs1_val,
        counter: counter + 1,
        opcode: Some(next_opcode)
      }
    },
    Opcode::Halt | Opcode::Noop => {
      State {
        mem,
        regs,
        pc: pc + 1,
        counter: counter + 1,
        opcode: Some(next_opcode)
      }
    },
  };

  Ok(new_state)
}