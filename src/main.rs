use std::env;

use common::ErrMsg;

mod common;
mod instruction;
mod interpreter;
mod io;
mod simulator;
mod state;
mod store;

pub const MEMORY_SIZE: usize = 65536;
pub const REGISTER_COUNT: usize = 8;

fn main() -> Result<(), ErrMsg> {
  let args: Vec<String> = env::args().collect();

  args.get(1)
    .ok_or_else(|| "Need a source file to be executed".into())
    .and_then(|in_file_name| {
      let mem = store::Store::<u32>::new(MEMORY_SIZE);
      let regs = store::Store::<i32>::new(REGISTER_COUNT);

      io::load_memory(mem, in_file_name)
        .and_then(|mem| simulator::simulate(mem, regs))
    })
}
