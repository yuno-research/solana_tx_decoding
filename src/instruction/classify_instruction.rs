use crate::instruction::pumpfun::is_pf_bonding_curve_create_instruction::is_pf_bonding_curve_create_instruction;
use crate::instruction::pumpfun::is_pumpfun_event_instruction::is_pumpfun_event_instruction;
use crate::instruction::pumpswap::is_pumpswap_swap_instruction::is_pumpswap_swap_instruction;
use crate::instruction::raydium::is_raydium_ammv4_swap_instruction::is_raydium_ammv4_swap_instruction;
use crate::instruction::raydium::is_raydium_cpmm_swap_instruction::is_raydium_cpmm_swap_instruction;
use crate::instruction::raydium::is_raydium_launchpad_swap_instruction::is_raydium_launchpad_swap_instruction;
use crate::types::instruction_type::InstructionType;
use solana_central::Instruction;
use solana_central::SwapDirection;

/**
Instruction classifier and because some classification functions also yield other info like swap
direction this returns a tuple. A dummy value will be used for those functions that don't return
the info like SwapDirection but InstructionType will always be correct or null.

The functions that return SwapDirection are:
- is pumpswap swap instruction
- is raydium launchpad swap instruction
*/
pub fn classify_instruction(instruction: &Instruction) -> (InstructionType, SwapDirection) {
  if is_raydium_ammv4_swap_instruction(instruction) {
    return (InstructionType::RaydiumAmmV4Swap, SwapDirection::AToB);
  } else if is_raydium_cpmm_swap_instruction(instruction) {
    return (InstructionType::RaydiumCpmmSwap, SwapDirection::AToB);
  }

  // These 2 functions return swap direction, use it in result instead of dummy value
  let (is_raydium_launchpad_swap, swap_direction) =
    is_raydium_launchpad_swap_instruction(instruction);
  if is_raydium_launchpad_swap {
    return (InstructionType::RaydiumLaunchpadSwap, swap_direction);
  }
  let (is_pumpswap_swap, swap_direction) = is_pumpswap_swap_instruction(instruction);
  if is_pumpswap_swap {
    return (InstructionType::PumpswapSwap, swap_direction);
  }
  // Dummy values
  else if is_pumpfun_event_instruction(instruction) {
    return (InstructionType::PfBondingCurveSwap, SwapDirection::AToB);
  } else if is_pf_bonding_curve_create_instruction(instruction) {
    return (InstructionType::PfBondingCurveCreate, SwapDirection::AToB);
  } else {
    return (InstructionType::None, SwapDirection::AToB);
  }
}
