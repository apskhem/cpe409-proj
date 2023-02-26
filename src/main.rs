use std::env;

mod instruction;
mod interpreter;
mod io;
mod simulator;
mod state;
mod store;

const MEMORY_SIZE: usize = 65536;
const REGISTER_COUNT: usize = 8;

fn main() -> Result<(), String> {
  let args: Vec<String> = env::args().collect();

  match args.get(1) {
    Some(in_file_name) => {
      let mem = store::Store::<u32>::new(MEMORY_SIZE);
      let regs = store::Store::<i32>::new(REGISTER_COUNT);
        
      let mem = io::load_memory(mem, in_file_name)?;

      simulator::simulate(mem, regs)
    },
    None => {
      Err("Need a source file to be executed.".into())
    }
  }
}
