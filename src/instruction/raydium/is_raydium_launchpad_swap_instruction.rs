use solana_central::constants::RAYDIUM_CONSTANTS;
use solana_central::types::instruction::Instruction;
use solana_central::types::swap_direction::SwapDirection;

/**
Determine whether or not a Solana instruction is a swap instruction of either a buy or sell on the
Raydium Launchpad protocol.
*/
pub fn is_raydium_launchpad_swap_instruction(instruction: &Instruction) -> (bool, SwapDirection) {
  if instruction.data.len() < 32 {
    return (false, SwapDirection::AToB);
  }
  if instruction.accounts.len() < 14 {
    return (false, SwapDirection::AToB);
  }
  if instruction.program_id != RAYDIUM_CONSTANTS.launchpad_program {
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
