use crate::common::ErrMsg;
use crate::interpreter::interpret;
use crate::state::State;
use crate::store::Store;
use crate::instruction::Opcode;

const SAFE_BREAK_CALL: i32 = 3000;

pub fn simulate(mem: Store<u32>, regs: Store<i32>) -> Result<(), ErrMsg> {
  let initial_state = State {
    mem,
    regs,
    pc: 0,
    counter: 0,
    opcode: None
  };

  initial_state.print_mem()?;

  let final_state = (0..).try_fold(initial_state, |state, exec_count| {
    if state.opcode == Some(Opcode::Halt) || exec_count > SAFE_BREAK_CALL {
      state.print_final()?;
      
      Err(None)
    } else {
      state.print()?;

      interpret(state)
        .map(State::sanitize_regs)
        .map_err(Some)
    }
  });

  final_state
    .err()
    .flatten()
    .map_or(Ok(()), Err)
}
