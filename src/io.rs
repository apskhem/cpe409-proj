use std::fs;

use crate::store::Store;
use crate::common::ErrMsg;

fn parse_file_buffer(input: &str) -> Result<Vec<u32>, ErrMsg> {
  if input.len() % 32 != 0 {
    Err("Unexpected formation of binary instruction input".into())
  } else {
    let res_arr = input
      .chars()
      .collect::<Vec<char>>()
      .chunks_exact(32)
      .map(|chars| chars.iter().collect::<String>())
      .map(|raw_instr| u32::from_str_radix(&raw_instr, 2))
      .collect::<Result<Vec<u32>, _>>();

    res_arr.map_err(|err| err.to_string())
  }
}

pub fn load_memory(mem: Store<u32>, path: &str) -> Result<Store<u32>, ErrMsg> {
  let text = fs::read_to_string(path).map_err(|err| err.to_string())?;
  let parsed_arr = parse_file_buffer(&text)?;
  
  Ok(mem.write_all(0, &parsed_arr))
}
