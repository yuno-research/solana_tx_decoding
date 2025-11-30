use crate::instruction::classify_instruction::classify_instruction;
use crate::instruction::pumpfun::process_pf_bonding_curve_create_instruction::process_pf_bonding_curve_create_instruction;
use crate::instruction::pumpswap::process_pumpswap_swap_instruction::process_pumpswap_swap_instruction;
use crate::instruction::raydium::process_raydium_ammv4_swap_instruction::process_raydium_ammv4_swap_instruction;
use crate::instruction::raydium::process_raydium_cpmm_swap_instruction::process_raydium_cpmm_swap_instruction;
use crate::instruction::raydium::process_raydium_launchpad_swap_instruction::process_raydium_launchpad_swap_instruction;
use crate::tx::inner_instructions_loop::inner_instructions_loop;
use crate::types::instruction_type::InstructionType;
use crate::types::token_creation::TokenCreation;
use solana_central::Instruction;
use solana_central::SwapTx;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use std::collections::HashMap;
use std::collections::HashSet;
use tokio::sync::broadcast::Sender;

/// Process top-level instructions in a transaction. Iterates through top-level instructions,
/// classifies them, and processes swap/creation instructions. Also calls `inner_instructions_loop`
/// to process the inner instructions that belong to each top level instruction.
pub fn top_level_instructions_loop(
  top_level_instructions: &Vec<Instruction>,
  inner_instructions: &HashMap<u8, Vec<Instruction>>,
  account_keys: &Vec<Pubkey>,
  ta_mint: &HashMap<u8, Pubkey>,
  running_token_balances: &mut HashMap<u8, u64>,
  swap_tx_sender: &Sender<SwapTx>,
  token_create_sender: &Sender<TokenCreation>,
  block_time: u64,
  slot: u64,
  index: u64,
  signers: &HashSet<Pubkey>,
  signature: &Signature,
) {
  let mut atomic_instruction_index = 0;
  for (instr_index, instruction) in top_level_instructions.iter().enumerate() {
    let instr_index = instr_index as u8;
    let (instruction_type, swap_direction) = classify_instruction(&instruction);
    // println!("Instruction type: {:?}", instruction_type);
    // println!("Instruction type: {:?}", instruction_type);
    // println!("Atomic instruction index: {:?}", atomic_instruction_index);
    if instruction_type == InstructionType::None {
      // Bump by 1, if its not none it will be bumped by 1 again adn the length of the inners
      atomic_instruction_index += 1;
      if let Some(inner_instructions) = inner_instructions.get(&instr_index) {
        inner_instructions_loop(
          inner_instructions,
          account_keys,
          ta_mint,
          running_token_balances,
          swap_tx_sender,
          token_create_sender,
          block_time,
          slot,
          index,
          &mut atomic_instruction_index,
          signers,
          signature,
        );
      }
    } else if instruction_type == InstructionType::RaydiumLaunchpadSwap {
      let event = &inner_instructions.get(&instr_index).unwrap()[0];
      let swap_tx = process_raydium_launchpad_swap_instruction(
        instruction,
        event,
        swap_direction,
        block_time,
        slot,
        index,
        atomic_instruction_index,
        signers,
        signature,
      );
      let _ = swap_tx_sender.send(swap_tx);
    } else if instruction_type == InstructionType::RaydiumCpmmSwap {
      let transfers = inner_instructions.get(&instr_index).unwrap();
      let swap_tx = process_raydium_cpmm_swap_instruction(
        instruction,
        transfers,
        running_token_balances,
        block_time,
        slot,
        index,
        atomic_instruction_index,
        signers,
        signature,
      );
      let _ = swap_tx_sender.send(swap_tx);
    } else if instruction_type == InstructionType::RaydiumAmmV4Swap {
      let transfers = inner_instructions.get(&instr_index).unwrap();
      let swap_tx = process_raydium_ammv4_swap_instruction(
        instruction,
        transfers,
        ta_mint,
        running_token_balances,
        block_time,
        slot,
        index,
        atomic_instruction_index,
        signers,
        signature,
      );
      let _ = swap_tx_sender.send(swap_tx);
    } else if instruction_type == InstructionType::PumpswapSwap {
      let event = inner_instructions
        .get(&instr_index)
        .unwrap()
        .last()
        .unwrap();
      let swap_tx = process_pumpswap_swap_instruction(
        instruction,
        event,
        swap_direction,
        block_time,
        slot,
        index,
        atomic_instruction_index,
        signers,
        signature,
      );
      let _ = swap_tx_sender.send(swap_tx);
    } else if instruction_type == InstructionType::PfBondingCurveCreate {
      let creation = process_pf_bonding_curve_create_instruction(
        instruction,
        block_time,
        slot,
        index,
        atomic_instruction_index,
        signature,
      );
      let _ = token_create_sender.send(creation);
    }
    // Add to atomic instruction index if not None since the top level swaps don't iterate through
    if instruction_type != InstructionType::None {
      let a = inner_instructions.get(&instr_index).unwrap().len() as u8;
      // println!("Inner instructions length: {:?}", a);
      atomic_instruction_index += a + 1;
    }
    // Pf bonding curve is not in here. Its not possible for a pf swap event be in a top level instruction
  }
}
