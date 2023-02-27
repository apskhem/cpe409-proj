use std::ops::RangeInclusive;

use crate::common::ErrMsg;

#[derive(PartialEq, Eq)]
pub enum Opcode {
  /// R-type
  Add,
  /// R-type
  Nand,
  /// I-type
  Lw,
  /// I-type
  Sw,
  /// I-type
  Beq,
  /// J-type
  Jalr,
  /// O-type
  Halt,
  /// O-type
  Noop,
}

impl Opcode {
  fn convert_raw(raw_code: u32) -> Result<Opcode, ErrMsg> {
    match raw_code {
      0b000 => Ok(Opcode::Add),
      0b001 => Ok(Opcode::Nand),
      0b010 => Ok(Opcode::Lw),
      0b011 => Ok(Opcode::Sw),
      0b100 => Ok(Opcode::Beq),
      0b101 => Ok(Opcode::Jalr),
      0b110 => Ok(Opcode::Halt),
      0b111 => Ok(Opcode::Noop),
      _ => Err("Unknown instruction opcode".into())
    }
  }
}

pub struct Instruction {
  pub opcode: Opcode,
  pub rs1: u32,
  pub rs2: u32,
  pub rd: u32,
  pub offset: i16,
  pub raw: u32,
}

impl TryFrom<u32> for Instruction {
  type Error = ErrMsg;

  fn try_from(raw: u32) -> Result<Self, Self::Error> {
    let raw_code = get_bin_range(raw, 22..=24);

    let tmp = Instruction {
      opcode: Opcode::convert_raw(raw_code)?,
      rs1: get_bin_range(raw, 19..=21),
      rs2: get_bin_range(raw, 16..=18),
      rd: get_bin_range(raw, 0..=2),
      offset: raw as i16,
      raw
    };

    Ok(tmp)
  }
}

fn get_bin_range(raw: u32, range: RangeInclusive<i32>) -> u32 {
  let sll = 32 - (range.end() + 1);
  let srl = sll + range.start();

  raw
    .wrapping_shl(sll as u32)
    .wrapping_shr(srl as u32)
}
