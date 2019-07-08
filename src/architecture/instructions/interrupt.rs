use crate::architecture::model::state::State;
use crate::architecture::model::instruction::Instruction;

/// # Interrupt Instructions:
///
/// Instructions which operate directly upon the Interrupt Enable
/// flip-flop INTE.

pub fn enable(state: &mut State, instruction: &Instruction) -> bool {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    true
}

pub fn disable(state: &mut State, instruction: &Instruction) -> bool {
    println!("ERROR: {} has not been implemented!", instruction.to_string());
    true
}