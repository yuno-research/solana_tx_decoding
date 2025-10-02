use solana_central::constants::PUMP_CONSTANTS;
use solana_central::types::instruction::Instruction;
use solana_central::types::swap_direction::SwapDirection;

/**
This is to verify the swap instruction itself and not the swap event yielded by it
*/
pub fn is_pumpswap_swap_instruction(instruction: &Instruction) -> (bool, SwapDirection) {
  if instruction.data.len() < 24 {
    return (false, SwapDirection::AToB);
  }
  let discriminator = &instruction.data[0..8];
  // Discriminator has to be either buy or sell
  if discriminator == PUMP_CONSTANTS.buy_instruction_discriminator {
    return (true, SwapDirection::BToA);
  } else if discriminator == PUMP_CONSTANTS.sell_instruction_discriminator {
    return (true, SwapDirection::AToB);
  } else {
    return (false, SwapDirection::AToB);
  }
}
