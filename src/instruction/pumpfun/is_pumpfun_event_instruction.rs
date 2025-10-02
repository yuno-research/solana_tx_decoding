use solana_central::constants::PUMP_CONSTANTS;
use solana_central::types::instruction::Instruction;

/**
Given a solana instruction, check if it is a pumpfun event instruction.
*/
pub fn is_pumpfun_event_instruction(instruction: &Instruction) -> bool {
  // check data discriminator
  for i in 0..PUMP_CONSTANTS.bonding_curve_event_discriminator.len() {
    if instruction.data[i] != PUMP_CONSTANTS.bonding_curve_event_discriminator[i] {
      return false;
    }
  }
  // check program id matches
  if instruction.program_id != PUMP_CONSTANTS.bonding_curve_program {
    return false;
  }
  // The only account that should be interacted with here is the event authority
  if instruction.accounts.len() < 1
    || instruction.accounts[0] != PUMP_CONSTANTS.bonding_curve_event_authority
  {
    return false;
  }
  // All checks for bonding curve event passed
  true
}
