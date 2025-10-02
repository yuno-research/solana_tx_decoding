use crate::protocol_idls::pumpfun::PfTradeEventIdlCurrent;
use crate::protocol_idls::pumpfun::PfTradeEventIdlOld;
use borsh::BorshDeserialize;
use solana_central::constants::LAMPORTS_PER_SOL;
use solana_central::constants::{PUMP_CONSTANTS, TOKENS};
use solana_central::pumpfun::derive_bonding_curve::derive_bonding_curve;
use solana_central::types::instruction::Instruction;
use solana_central::types::pools::Pools;
use solana_central::types::swap_direction::SwapDirection;
use solana_central::types::swap_tx::SwapTx;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use std::collections::HashSet;

/**
PF bonding curve events contain all you need to build the SwapTx type. This function assumed that
the instruction has already been validated as a valid pumpfun event instruction.
*/
pub fn process_pumpfun_event_instruction(
  instruction: &Instruction,
  atomic_instruction_index: u64,
  index: u64,
  slot: u64,
  block_time: u64,
  signature: &Signature,
  signers: &HashSet<Pubkey>,
) -> SwapTx {
  let token_address;
  let is_buy;
  let fee_fraction_lp;
  let sol_amount;
  let total_fee;
  let token_amount;
  let pool_token_a_vault_amount;
  let pool_token_b_vault_amount;
  let virtual_sol_reserves;
  let virtual_token_reserves;

  // The new event post creator fee update
  if instruction.data.len() == 266 {
    let decoded_layout =
      PfTradeEventIdlCurrent::try_from_slice(instruction.data.as_slice()).unwrap();
    token_address = decoded_layout.mint;
    is_buy = decoded_layout.is_buy;
    sol_amount = decoded_layout.sol_amount;
    total_fee = decoded_layout.creator_fee + decoded_layout.fee;
    token_amount = decoded_layout.token_amount;
    pool_token_a_vault_amount = decoded_layout.real_token_reserves;
    pool_token_b_vault_amount = decoded_layout.real_sol_reserves;
    virtual_sol_reserves = decoded_layout.virtual_sol_reserves;
    virtual_token_reserves = decoded_layout.virtual_token_reserves;
    fee_fraction_lp =
      (decoded_layout.fee_basis_points + decoded_layout.creator_fee_basis_points) * 100000;
  }
  // The old creator fee event pre creator fee update
  else if instruction.data.len() == 137 {
    let decoded_layout = PfTradeEventIdlOld::try_from_slice(instruction.data.as_slice()).unwrap();
    token_address = decoded_layout.mint;
    is_buy = decoded_layout.is_buy;
    sol_amount = decoded_layout.sol_amount;
    token_amount = decoded_layout.token_amount;
    pool_token_a_vault_amount =
      decoded_layout.virtual_token_reserves - PUMP_CONSTANTS.bc_init_virtual_token_reserve_diff;
    pool_token_b_vault_amount =
      decoded_layout.virtual_sol_reserves - PUMP_CONSTANTS.bc_init_virtual_sol_reserves;
    virtual_sol_reserves = decoded_layout.virtual_sol_reserves;
    virtual_token_reserves = decoded_layout.virtual_token_reserves;
    /*
    For old events, the fee fraction was always 1%, and an offset was used to go from virtual to
    real reserves for token and sol
    */
    fee_fraction_lp = 10000000;
    // Fee was 1% of sol amount involved both on the way in added on and on the way out subtracted off
    total_fee = sol_amount / 100;
  } else {
    panic!(
      "Pumpfun: Found a swap event, but data length is not recognized, tx signature: {}",
      signature
    );
  }

  let market_address = derive_bonding_curve(&token_address);
  let direction;
  let swapped_amount_in;
  let swapped_amount_received;

  if is_buy {
    direction = SwapDirection::BToA;
    swapped_amount_in = sol_amount + total_fee;
    swapped_amount_received = token_amount;
  } else {
    direction = SwapDirection::AToB;
    swapped_amount_in = token_amount;
    swapped_amount_received = sol_amount - total_fee;
  }

  SwapTx {
    pool: Pools::PfBondingCurve,
    direction,
    block_time,
    slot,
    index,
    atomic_instruction_index,
    fee_fraction_lp,

    // If swap is from A to B, then this is in terms of A tokens spent in
    swapped_amount_in,
    // If swap is from A to b, then this is in terms fo B tokens recieved
    swapped_amount_received,
    // Quantity of token in vault AFTER swap is completed
    pool_token_a_vault_amount,
    pool_token_b_vault_amount,
    price_a_b_lp: (virtual_token_reserves as u128 * LAMPORTS_PER_SOL / virtual_sol_reserves as u128)
      as u64,
    price_b_a_lp: (virtual_sol_reserves as u128 * LAMPORTS_PER_SOL / virtual_token_reserves as u128)
      as u64,

    token_a_address: token_address,
    token_b_address: TOKENS.wsol,
    market_address,
    signature: signature.clone(),
    signers: signers.clone(),
  }
}
