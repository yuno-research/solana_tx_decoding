use solana_central::Instruction;
use solana_central::constants::RAYDIUM_CONSTANTS;

/// Determine if a Solana instruction is a Raydium Cpmm/AmmV5 swap instruction. Checks program ID,
/// data length, account count, and instruction discriminator to identify Raydium CPMM swap
/// instructions.
pub fn is_raydium_cpmm_swap_instruction(instruction: &Instruction) -> bool {
  if instruction.data.len() < 24 {
    return false;
  }
  if instruction.accounts.len() < 13 {
    return false;
  }
  if instruction.tx_account_keys[instruction.program_id_index as usize]
    != RAYDIUM_CONSTANTS.cpmm_program
  {
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
