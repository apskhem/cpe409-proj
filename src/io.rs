use std::fs;

use crate::store::Store;

fn parse_file_buffer(input: &str) -> Result<Vec<u32>, String> {
  if input.len() % 32 != 0 {
    Err("Unexpected formation of binary instruction input".into())
  } else {
    let arr = input
      .chars()
      .collect::<Vec<char>>()
      .chunks_exact(32)
      .map(|chars| -> String {
        chars.iter().collect()
      })
      .map(|raw_instr| {
        u32::from_str_radix(&raw_instr, 2).unwrap()
      })
      .collect();

    Ok(arr)
  }
}

pub fn load_memory(mem: Store<u32>, path: &str) -> Result<Store<u32>, String> {
  fs::read_to_string(path)
    .map_err(|err| err.to_string())
    .and_then(|text| {
      parse_file_buffer(&text)
    })
    .and_then(|parsed_arr| {
      Ok(mem.write_all(0, &parsed_arr))
    })
}
