use solana_central::constants::PUMP_CONSTANTS;
use solana_central::types::instruction::Instruction;

/**
Given a solana instruction, check if it is a pumpfun event instruction.
*/
pub fn is_pumpfun_event_instruction(instruction: &Instruction) -> bool {
  // Smallest data length is 137, we'll give a bit tolerance
  if instruction.data.len() < 100 {
    return false;
  }
  // check data discriminator
  for i in 0..PUMP_CONSTANTS.bonding_curve_event_discriminator.len() {
    if instruction.data[i] != PUMP_CONSTANTS.bonding_curve_event_discriminator[i] {
      return false;
    }
  }
  // check program id matches
  if instruction.tx_account_keys[instruction.program_id_index as usize] != PUMP_CONSTANTS.bonding_curve_program {
    return false;
  }
  // The only account that should be interacted with here is the event authority
  if instruction.accounts.len() < 1
    || instruction.tx_account_keys[instruction.accounts[0] as usize] != PUMP_CONSTANTS.bonding_curve_event_authority
  {
    return false;
  }
  // All checks for bonding curve event passed
  true
}
