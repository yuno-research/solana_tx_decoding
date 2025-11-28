use borsh::BorshDeserialize;
use solana_central::Instruction;
use solana_central::Pools;
use solana_central::SwapDirection;
use solana_central::SwapTx;
use solana_central::constants::LAMPORTS_PER_SOL;
use solana_central::protocol_idls::raydium::LaunchpadTradeEventIdl;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use std::collections::HashSet;

/**
Given the swap instruction and its corresponding event, process swap instruction if it is relevant
signer, update market state, etc.

The reason why both the instruction and the event are needed is because the platform config cannot
be derived from only the data given in the event.
*/
pub fn process_raydium_launchpad_swap_instruction(
  instruction: &Instruction,
  event: &Instruction,
  direction: SwapDirection,
  block_time: u64,
  slot: u64,
  index: u64,
  atomic_instruction_index: u8,
  signers: &HashSet<Pubkey>,
  signature: &Signature,
) -> SwapTx {
  let token_a_address = instruction.tx_account_keys[instruction.accounts[9] as usize];
  let token_b_address = instruction.tx_account_keys[instruction.accounts[10] as usize];
  let market_address = instruction.tx_account_keys[instruction.accounts[4] as usize];
  let swap_event = LaunchpadTradeEventIdl::try_from_slice(&event.data).unwrap();

  let swapped_amount_in = swap_event.amount_in;
  let swapped_amount_received = swap_event.amount_out;
  let total_swap_fee = (swap_event.protocol_fee
    + swap_event.platform_fee
    + swap_event.share_fee
    + swap_event.creator_fee) as u128;
  let fee_fraction_lp;

  // If sell, charged on way out, if buy, charged on way in
  if direction == SwapDirection::AToB {
    fee_fraction_lp = (total_swap_fee * LAMPORTS_PER_SOL
      / (swapped_amount_received as u128 + total_swap_fee)) as u64;
  } else {
    fee_fraction_lp = (total_swap_fee * LAMPORTS_PER_SOL / swapped_amount_in as u128) as u64;
  }

  let pool_token_a_vault_amount = swap_event.virtual_base - swap_event.real_base_after;
  let pool_token_b_vault_amount = swap_event.virtual_quote + swap_event.real_quote_after;

  let price_a_b_lp =
    pool_token_a_vault_amount as u128 * LAMPORTS_PER_SOL / pool_token_b_vault_amount as u128;
  let price_b_a_lp =
    pool_token_b_vault_amount as u128 * LAMPORTS_PER_SOL / pool_token_a_vault_amount as u128;

  SwapTx {
    pool: Pools::RaydiumLaunchpad,
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
