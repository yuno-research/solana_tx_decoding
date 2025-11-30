use solana_central::Instruction;
use solana_central::SwapDirection;
use solana_central::constants::RAYDIUM_CONSTANTS;

/// Determine if a Solana instruction is a Raydium launchpad swap instruction. Checks program ID,
/// data length, account count, and instruction discriminator to identify Raydium launchpad swap
/// instructions. Returns the swap direction identified by the discriminator.
pub fn is_raydium_launchpad_swap_instruction(instruction: &Instruction) -> (bool, SwapDirection) {
  if instruction.data.len() < 32 {
    return (false, SwapDirection::AToB);
  }
  if instruction.accounts.len() < 14 {
    return (false, SwapDirection::AToB);
  }
  if instruction.tx_account_keys[instruction.program_id_index as usize]
    != RAYDIUM_CONSTANTS.launchpad_program
  {
    return (false, SwapDirection::AToB);
  }
  // Discriminator has to match one of the launchpad swap discriminators
  if instruction.data[0..8] == RAYDIUM_CONSTANTS.launchpad_swap_discriminators[0] {
    return (true, SwapDirection::BToA);
  } else if instruction.data[0..8] == RAYDIUM_CONSTANTS.launchpad_swap_discriminators[1] {
    return (true, SwapDirection::AToB);
  } else if instruction.data[0..8] == RAYDIUM_CONSTANTS.launchpad_swap_discriminators[2] {
    return (true, SwapDirection::BToA);
  } else if instruction.data[0..8] == RAYDIUM_CONSTANTS.launchpad_swap_discriminators[3] {
    return (true, SwapDirection::AToB);
  } else {
    return (false, SwapDirection::AToB);
  }
}
