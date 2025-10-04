use solana_central::constants::RAYDIUM_CONSTANTS;
use solana_central::types::instruction::Instruction;

/**
Determine whether or not a Solana instruction is a swap instruction of either a buy or sell on the
Raydium CPMM protocol.
*/
pub fn is_raydium_cpmm_swap_instruction(instruction: &Instruction) -> bool {
  if instruction.data.len() < 24 {
    return false;
  }
  if instruction.accounts.len() < 13 {
    return false;
  }
  if instruction.tx_account_keys[instruction.program_id_index as usize] != RAYDIUM_CONSTANTS.cpmm_program {
    return false;
  }
  // Discriminator has to match one of the cpmm swap discriminators
  if !(instruction.data[0..8] == RAYDIUM_CONSTANTS.cpmm_swap_discriminators[0]
    || instruction.data[0..8] == RAYDIUM_CONSTANTS.cpmm_swap_discriminators[1])
  {
    return false;
  }
  true
}
