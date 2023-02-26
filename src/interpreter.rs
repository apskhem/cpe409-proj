use crate::{state::State, instruction::{Instruction, Opcode}};

pub fn interpret(state: State) -> Result<State, String> {
  let State {
    mem,
    regs,
    pc,
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
        opcode: Some(next_opcode)
      }
    },
    Opcode::Nand => {
      State {
        mem,
        regs: regs.write(rd as usize, !(rs1_val & rs2_val)),
        pc: pc + 1,
        opcode: Some(next_opcode)
      }
    },
    Opcode::Lw => {
      let lw_idx = rs1_val as i16 + offset;
      let new_val = mem.at(lw_idx as usize)? as i32;

      State {
        mem,
        regs: regs.write(rd as usize, new_val),
        pc: pc + 1,
        opcode: Some(next_opcode)
      }
    },
    Opcode::Sw => {
      let sw_idx = rs1_val as i16 + offset;

      State {
        mem: mem.write(sw_idx as usize, regs.at(rs2 as usize)? as u32),
        regs,
        pc: pc + 1,
        opcode: Some(next_opcode)
      }
    },
    Opcode::Beq => {
      let op_pc = if rs1_val == rs2_val { offset as i32 } else { 0 };

      State {
        mem,
        regs,
        pc: pc + 1 + op_pc,
        opcode: Some(next_opcode)
      }
    },
    Opcode::Jalr => {
      State {
        mem,
        regs: regs.write(rs2 as usize, pc + 1),
        pc: rs1_val,
        opcode: Some(next_opcode)
      }
    },
    Opcode::Halt | Opcode::Noop => {
      State {
        mem,
        regs,
        pc: pc + 1,
        opcode: Some(next_opcode)
      }
    },
  };

  Ok(new_state)
}