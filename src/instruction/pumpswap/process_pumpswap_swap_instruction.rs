use solana_central::protocol_idls::pumpswap::{
  PumpswapBuyEventIdl, PumpswapSellEventIdl,
};
use borsh::BorshDeserialize;
use solana_central::constants::LAMPORTS_PER_SOL;
use solana_central::Instruction;
use solana_central::Pools;
use solana_central::SwapDirection;
use solana_central::SwapTx;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use std::collections::HashSet;

/// Process a Pumpswap swap instruction and create a SwapTx. Assumes the instruction has been
/// validated as a valid Pumpswap swap. Needs the event instruction that follows the swap
/// instruction to extract swap details.
pub fn process_pumpswap_swap_instruction(
  instruction: &Instruction,
  swap_event_instruction: &Instruction,
  direction: SwapDirection,
  block_time: u64,
  slot: u64,
  index: u64,
  atomic_instruction_index: u8,
  signers: &HashSet<Pubkey>,
  signature: &Signature,
) -> SwapTx {
  let token_a_address = instruction.tx_account_keys[instruction.accounts[3] as usize];
  let token_b_address = instruction.tx_account_keys[instruction.accounts[4] as usize];
  let market_address = instruction.tx_account_keys[instruction.accounts[0] as usize];

  let swapped_amount_in;
  let swapped_amount_received;
  let pool_token_a_vault_amount;
  let pool_token_b_vault_amount;
  let fee_fraction_lp;

  let event_len = swap_event_instruction.data.len();

  // Sell instruction by pumpswap
  if direction == SwapDirection::AToB {
    let decoded_event = PumpswapSellEventIdl::try_from_slice(&swap_event_instruction.data).unwrap();

    swapped_amount_in = decoded_event.base_amount_in;
    // user quote amount out is the amount of quote token the user actually received
    swapped_amount_received = decoded_event.user_quote_amount_out;
    // Token vault balances in the event are confirmed to be pre balances
    pool_token_a_vault_amount = decoded_event.pool_base_token_reserves + swapped_amount_in;
    // Out of the token vault comes out the user owed, protocol fee, and creator fee
    pool_token_b_vault_amount = decoded_event.pool_quote_token_reserves
      - swapped_amount_received
      - decoded_event.protocol_fee
      - decoded_event.coin_creator_fee;
    let total_fee =
      decoded_event.lp_fee + decoded_event.protocol_fee + decoded_event.coin_creator_fee;
    // amount out is only what user receives out
    // Divison by zero check for zero swapped amounts
    fee_fraction_lp = if swapped_amount_received + total_fee == 0 {
      0
    } else {
      (total_fee as u128 * LAMPORTS_PER_SOL / (swapped_amount_received + total_fee) as u128) as u64
    };
  }
  // Buy instruction by pumpswap - check event length for backward compatibility
  else {

    if 
    // buy
    event_len == 416 || 
    // old without ix_name
    event_len == 401 || 
    // buy_exact_quote_in event length
    event_len == 431 {
      let decoded_event =
        PumpswapBuyEventIdl::try_from_slice(&swap_event_instruction.data[..401]).unwrap();
      if event_len == 431 {
        // Buy exact quote in events have quote amount is as the total amount the user swaps in.
        // Flipped around garbage protocol
        swapped_amount_in = decoded_event.quote_amount_in;
      }
      else {
        swapped_amount_in = decoded_event.user_quote_amount_in;
      }
      swapped_amount_received = decoded_event.base_amount_out;
      pool_token_b_vault_amount =
        decoded_event.pool_quote_token_reserves + decoded_event.quote_amount_in_with_lp_fee;
      pool_token_a_vault_amount = decoded_event.pool_base_token_reserves - swapped_amount_received;
      let total_fee =
        decoded_event.lp_fee + decoded_event.protocol_fee + decoded_event.coin_creator_fee;
      fee_fraction_lp = if swapped_amount_in == 0 {
        0
      } else {
        (total_fee as u128 * LAMPORTS_PER_SOL / swapped_amount_in as u128) as u64
      };
    }
    else {
      panic!(
        "Pumpswap: Found a buy event, but data length {} is not recognized (expected 401 or 416), tx signature: {}",
        event_len, signature
      );
    }
  }

  let price_a_b_lp =
    pool_token_a_vault_amount as u128 * LAMPORTS_PER_SOL / pool_token_b_vault_amount as u128;
  let price_b_a_lp =
    pool_token_b_vault_amount as u128 * LAMPORTS_PER_SOL / pool_token_a_vault_amount as u128;

  SwapTx {
    pool: Pools::PumpswapAmm,
    direction,
    block_time,
    slot,
    index,
    atomic_instruction_index,
    fee_fraction_lp,
    swapped_amount_in,
    swapped_amount_received,
    pool_token_a_vault_amount,
    pool_token_b_vault_amount,
    price_a_b_lp,
    price_b_a_lp,
    token_a_address,
    token_b_address,
    market_address,
    signature: signature.clone(),
    signers: signers.clone(),
  }
}
