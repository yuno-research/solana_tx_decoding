use crate::instruction::classify_instruction::classify_instruction;
use crate::instruction::pumpswap::process_pumpswap_swap_instruction::process_pumpswap_swap_instruction;
use crate::instruction::raydium::process_raydium_ammv4_swap_instruction::process_raydium_ammv4_swap_instruction;
use crate::instruction::raydium::process_raydium_cpmm_swap_instruction::process_raydium_cpmm_swap_instruction;
use crate::instruction::raydium::process_raydium_launchpad_swap_instruction::process_raydium_launchpad_swap_instruction;
use crate::types::instruction_type::InstructionType;
use solana_central::constants::PUMP_CONSTANTS;
use solana_central::types::instruction::Instruction;
use solana_central::types::swap_tx::SwapTx;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use std::collections::HashMap;
use std::collections::HashSet;
use tokio::sync::broadcast::Sender;

pub fn inner_instructions_loop(
  inner_instructions: &Vec<Instruction>,
  account_keys: &Vec<Pubkey>,
  ta_mint: &HashMap<u8, Pubkey>,
  running_token_balances: &mut HashMap<u8, u64>,
  swap_tx_sender: &Sender<SwapTx>,
  block_time: u64,
  slot: u64,
  index: u64,
  atomic_instruction_index: &mut u8,
  signers: &HashSet<Pubkey>,
  signature: &Signature,
) {
  // Check inner instructions if they exist
  let mut instr_index = 0;
  while instr_index < inner_instructions.len() {
    let instruction = &inner_instructions[instr_index];

    let (instruction_type, swap_direction) = classify_instruction(&instruction);
    if instruction_type == InstructionType::None {
      instr_index += 1;
      continue;
    } else if instruction_type == InstructionType::RaydiumLaunchpadSwap {
      // Event instruction is first instruction after the swap
      let event = &inner_instructions[instr_index + 1];
      let swap_tx = process_raydium_launchpad_swap_instruction(
        instruction,
        event,
        swap_direction,
        block_time,
        slot,
        index,
        *atomic_instruction_index,
        signers,
        signature,
      );
      let _ = swap_tx_sender.send(swap_tx);
    } else if instruction_type == InstructionType::RaydiumCpmmSwap {
      // The transfers are the two instructions immediately after the swap
      let transfers = &inner_instructions[instr_index + 1..instr_index + 3];
      let swap_tx = process_raydium_cpmm_swap_instruction(
        instruction,
        transfers,
        running_token_balances,
        block_time,
        slot,
        index,
        *atomic_instruction_index,
        signers,
        signature,
      );
      let _ = swap_tx_sender.send(swap_tx);
    } else if instruction_type == InstructionType::RaydiumAmmV4Swap {
      // The transfers are the two instructions immediately after the swap
      let transfers = &inner_instructions[instr_index + 1..instr_index + 3];
      let swap_tx = process_raydium_ammv4_swap_instruction(
        instruction,
        transfers,
        ta_mint,
        running_token_balances,
        block_time,
        slot,
        index,
        *atomic_instruction_index,
        signers,
        signature,
      );
      let _ = swap_tx_sender.send(swap_tx);
    } else if instruction_type == InstructionType::PumpswapSwap {
      // To find this event, look from the instructions following the swap until we find one that is for the pumpswap program
      let mut event = None;
      for i in instr_index + 1..inner_instructions.len() {
        if account_keys[inner_instructions[i].program_id_index as usize]
          == PUMP_CONSTANTS.pump_swap_program
        {
          event = Some(&inner_instructions[i]);
          break;
        }
      }
      let swap_tx = process_pumpswap_swap_instruction(
          instruction,
          event.expect(&format!("inner_instructions_loop: Unable to find pumpswap event following inner instruction swap, signature: {}", signature.to_string())),
          swap_direction,
          block_time,
          slot,
          index,
          *atomic_instruction_index,
          signers,
          signature,
        );
      let _ = swap_tx_sender.send(swap_tx);
    }
    // Pf bonding curve is not in here. Its not possible for a pf swap event be in a top level instruction
    //
    *atomic_instruction_index += 1;
  }
}
