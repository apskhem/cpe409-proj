use crate::interpreter::interpret;
use crate::state::State;
use crate::store::Store;
use crate::instruction::Opcode;

const SAFE_BREAK_CALL: i32 = 20000;

pub fn simulate(mem: Store<u32>, regs: Store<i32>) -> Result<(), String> {
  let initial_state = State {
    mem,
    regs,
    pc: 0,
    opcode: None
  };

  let final_state = (0..).try_fold(initial_state, |state, exec_count| {
    if state.opcode == Some(Opcode::Halt) || SAFE_BREAK_CALL < exec_count {
      Err(None)
    } else {
      interpret(state)
        .map(|state| {
          State {
            regs: state.regs.write(0, 0),
            ..state
          }
        })
        .map_err(Some)
    }
  });

  final_state
    .err()
    .flatten()
    .map_or(Ok(()), Err)
}