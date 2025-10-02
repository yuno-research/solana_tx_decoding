use crate::protocol_idls::pumpswap::{PumpswapBuyEventIdl, PumpswapSellEventIdl};
use borsh::BorshDeserialize;
use solana_central::constants::LAMPORTS_PER_SOL;
use solana_central::types::instruction::Instruction;
use solana_central::types::pools::Pools;
use solana_central::types::swap_direction::SwapDirection;
use solana_central::types::swap_tx::SwapTx;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use std::collections::HashSet;

/**
Given a solana instruction, check if it is a pumpswap swap event and then process it accordingly.
*/
pub fn process_pumpswap_swap_instruction(
  instruction: &Instruction,
  swap_event_instruction: &Instruction,
  direction: SwapDirection,
  block_time: u64,
  slot: u64,
  index: u64,
  atomic_instruction_index: u64,
  signers: &HashSet<Pubkey>,
  signature: &Signature,
) -> SwapTx {
  let token_a_address = instruction.accounts[3];
  let token_b_address = instruction.accounts[4];
  let market_address = instruction.accounts[0];

  let swapped_amount_in;
  let swapped_amount_received;
  let pool_token_a_vault_amount;
  let pool_token_b_vault_amount;
  let fee_fraction_lp;

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
    fee_fraction_lp =
      (total_fee as u128 * LAMPORTS_PER_SOL / (swapped_amount_received + total_fee) as u128) as u64;
  } else {
    let decoded_event = PumpswapBuyEventIdl::try_from_slice(&swap_event_instruction.data).unwrap();
    // This value included creator fee and lp fee and protocol fee and amount swapped in
    swapped_amount_in = decoded_event.user_quote_amount_in;
    swapped_amount_received = decoded_event.base_amount_out;
    // into the pool is not added the protocol fee, or creator fee.
    pool_token_b_vault_amount =
      decoded_event.pool_quote_token_reserves + decoded_event.quote_amount_in_with_lp_fee;
    pool_token_a_vault_amount = decoded_event.pool_base_token_reserves - swapped_amount_received;
    let total_fee =
      decoded_event.lp_fee + decoded_event.protocol_fee + decoded_event.coin_creator_fee;
    // amount in is total user transfers in, don't include fee in denominator
    fee_fraction_lp = (total_fee as u128 * LAMPORTS_PER_SOL / (swapped_amount_in) as u128) as u64;
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
