use solana_central::Instruction;
use solana_central::constants::RAYDIUM_CONSTANTS;

/**
Determine whether or not a Solana instruction is a swap instruction of either a buy or sell on the
Raydium AmmV4 protocol.
*/
pub fn is_raydium_ammv4_swap_instruction(instruction: &Instruction) -> bool {
  if instruction.data.len() < 17 {
    return false;
  }
  // Raydium ammv4 swap instructions can have 17 or 18 accounts, so minimum 17
  if instruction.accounts.len() < 17 {
    return false;
  }
  if instruction.tx_account_keys[instruction.program_id_index as usize]
    != RAYDIUM_CONSTANTS.amm_program
  {
    return false;
  }
  // Discriminator has to match one of the ammv4 discriminators
  if !(instruction.data[0] == RAYDIUM_CONSTANTS.ammv4_swap_discriminators[0]
    || instruction.data[0] == RAYDIUM_CONSTANTS.ammv4_swap_discriminators[1])
  {
    return false;
  }
  true
}
