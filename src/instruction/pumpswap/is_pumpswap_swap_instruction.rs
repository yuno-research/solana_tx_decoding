use solana_central::Instruction;
use solana_central::SwapDirection;
use solana_central::constants::PUMP_CONSTANTS;

/**
This is to verify the swap instruction itself and not the swap event yielded by it
*/
pub fn is_pumpswap_swap_instruction(instruction: &Instruction) -> (bool, SwapDirection) {
  // Data length should be 24 for buy and sell, 25 for buy exact quote in
  if instruction.data.len() < 24 {
    return (false, SwapDirection::AToB);
  }
  if instruction.tx_account_keys[instruction.program_id_index as usize]
    != PUMP_CONSTANTS.pump_swap_program
  {
    return (false, SwapDirection::AToB);
  }
  let discriminator = &instruction.data[0..8];
  // Discriminator has to be either buy or sell
  if discriminator == PUMP_CONSTANTS.buy_instruction_discriminator {
    return (true, SwapDirection::BToA);
  } else if discriminator == PUMP_CONSTANTS.sell_instruction_discriminator {
    return (true, SwapDirection::AToB);
  } else if discriminator == PUMP_CONSTANTS.pumpswap_buy_exact_quote_in_instruction_discriminator {
    return (true, SwapDirection::BToA);
  } else {
    return (false, SwapDirection::AToB);
  }
}
