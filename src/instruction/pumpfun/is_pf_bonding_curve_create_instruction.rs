use solana_central::constants::PUMP_CONSTANTS;
use solana_central::Instruction;

/// Determine if a Solana instruction is a Pumpfun bonding curve creation instruction. Checks data
/// length, account count, discriminator, and program ID to identify Pumpfun bonding curve creation
/// instructions.
pub fn is_pf_bonding_curve_create_instruction(instruction: &Instruction) -> bool {
  /*
  Data can be of any unbounded size due to string in it, but the minimum size will be 40 since it
  has to contain the creator address pubkey and the instruction discriminator
  */
  if instruction.data.len() < 40 {
    return false;
  }
  // Length of accounts is minimum 14 for this instruction
  if instruction.accounts.len() < 14 {
    return false;
  }
  // check data discriminator
  for i in 0..PUMP_CONSTANTS.bonding_curve_create_instruction_discriminator.len() {
    if instruction.data[i] != PUMP_CONSTANTS.bonding_curve_create_instruction_discriminator[i] {
      return false;
    }
  }
  // check program id matches
  if instruction.tx_account_keys[instruction.program_id_index as usize] != PUMP_CONSTANTS.bonding_curve_program {
    return false;
  }
  // All checks for bonding curve create instruction passed
  true
}
