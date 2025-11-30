use solana_central::Instruction;
use solana_central::Pools;
use solana_central::SwapDirection;
use solana_central::SwapTx;
use solana_central::constants::LAMPORTS_PER_SOL;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use std::collections::HashMap;
use std::collections::HashSet;

/// Process a Raydium AMMv4 swap instruction and create a SwapTx. Assumes the instruction has been
/// validated as a valid Raydium AMMv4 swap. Uses token transfer instructions that follow the swap
/// to determine swap amounts.
pub fn process_raydium_ammv4_swap_instruction(
  // The swap instruction itself
  instruction: &Instruction,
  // The 2 token transfers (into one vault, out of the other vault) that come after the swap
  // TODO this might have to be changed depending on whether rpc data returns parsed for this
  transfers: &[Instruction],
  ta_mint: &HashMap<u8, Pubkey>,
  running_token_balances: &mut HashMap<u8, u64>,
  block_time: u64,
  slot: u64,
  index: u64,
  atomic_instruction_index: u8,
  signers: &HashSet<Pubkey>,
  signature: &Signature,
) -> SwapTx {
  let market_address = instruction.tx_account_keys[instruction.accounts[1] as usize];

  // account indices change based on length of accounts array
  let token_a_vault_address;
  let token_b_vault_address;
  if instruction.accounts.len() == 17 {
    token_a_vault_address = instruction.accounts[4];
    token_b_vault_address = instruction.accounts[5];
  } else if instruction.accounts.len() == 18 {
    token_a_vault_address = instruction.accounts[5];
    token_b_vault_address = instruction.accounts[6];
  } else {
    panic!(
      "process_raydium_ammv4_swap_instruction: Invalid number of accounts in swap instruction, signature: {}",
      signature
    );
  }
  // Identify token addresses involved in tx, not included in swap instruction
  let token_a_address = *ta_mint.get(&token_a_vault_address).expect(format!("process_raydium_ammv4_swap_instruction: Token a vault address not found in ta_mint, signature: {}", signature).as_str());
  let token_b_address = *ta_mint.get(&token_b_vault_address).expect(format!("process_raydium_ammv4_swap_instruction: Token b vault address not found in ta_mint, signature: {}", signature).as_str());

  /*
  Amount in is how much you sent to the pool in the first transfer instruction. Amount out is how
  much you got out of the pool in the second transfer instruction. The amount value is stored in
  the instruction data following the instruction 1 byte discriminator and is in a u64 value.
  Direction doesn't influence where amount in and amount out are found and the order of the
  transfers is always the same. First in then out
  */
  let swapped_amount_in = u64::from_le_bytes(transfers[0].data[1..9].try_into().unwrap());
  let swapped_amount_received = u64::from_le_bytes(transfers[1].data[1..9].try_into().unwrap());

  /*
  Direction here is determined by if you are sending to the token a vault, then its A to B,
  otherwise its B to A the same way that historical ingestion does it
  */
  let direction: SwapDirection;
  if transfers[0].accounts[1] == token_a_vault_address {
    direction = SwapDirection::AToB;
    if let Some(running_token_balance) = running_token_balances.get_mut(&token_a_vault_address) {
      *running_token_balance += swapped_amount_in;
    }
    if let Some(running_token_balance) = running_token_balances.get_mut(&token_b_vault_address) {
      *running_token_balance -= swapped_amount_received;
    }
  } else {
    direction = SwapDirection::BToA;
    if let Some(running_token_balance) = running_token_balances.get_mut(&token_b_vault_address) {
      *running_token_balance += swapped_amount_in;
    }
    if let Some(running_token_balance) = running_token_balances.get_mut(&token_a_vault_address) {
      *running_token_balance -= swapped_amount_received;
    }
  };

  let pool_token_a_vault_amount: u64 = running_token_balances[&token_a_vault_address];
  let pool_token_b_vault_amount: u64 = running_token_balances[&token_b_vault_address];
  let price_b_a_lp =
    LAMPORTS_PER_SOL * pool_token_b_vault_amount as u128 / pool_token_a_vault_amount as u128;
  let price_a_b_lp =
    LAMPORTS_PER_SOL * pool_token_a_vault_amount as u128 / pool_token_b_vault_amount as u128;

  SwapTx {
    pool: Pools::RaydiumAmmV4,
    direction,
    block_time,
    slot,
    index,
    atomic_instruction_index,
    // Raydium Ammv4 fees are 0.25% fixed rate
    fee_fraction_lp: 2500000,
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
