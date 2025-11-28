use solana_central::Instruction;
use solana_central::Pools;
use solana_central::SwapDirection;
use solana_central::SwapTx;
use solana_central::constants::LAMPORTS_PER_SOL;
use solana_central::get_cpmm_fee_amount_from_config_account;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use std::collections::HashMap;
use std::collections::HashSet;

/**
This function is called in a scenario where the signer of a tx is a relevant signer has produced a
raydium cpmm swap instruction. No checks are done here, ingestion tx loop does the checks and then
calls this function when it detects a cpmm swap instruction on a relevant signer. This function is
not used for market updates. All it does is broadcast the swap tx for relevant signers.
*/
pub fn process_raydium_cpmm_swap_instruction(
  // The swap instruction itself
  instruction: &Instruction,
  // The 2 token transfers (into one vault, out of the other vault) that come after the swap
  transfers: &[Instruction],
  running_token_balances: &mut HashMap<u8, u64>,
  block_time: u64,
  slot: u64,
  index: u64,
  atomic_instruction_index: u8,
  signers: &HashSet<Pubkey>,
  signature: &Signature,
) -> SwapTx {
  let market_address = instruction.tx_account_keys[instruction.accounts[3] as usize];
  let input_token_mint = instruction.tx_account_keys[instruction.accounts[10] as usize];
  let output_token_mint = instruction.tx_account_keys[instruction.accounts[11] as usize];
  let input_token_vault = instruction.accounts[6];
  let output_token_vault = instruction.accounts[7];

  // Pool's input token balance will go up as the user sends funds to the pool
  let swapped_amount_in = u64::from_le_bytes(transfers[0].data[1..9].try_into().unwrap());
  if let Some(running_token_balance) = running_token_balances.get_mut(&input_token_vault) {
    *running_token_balance += swapped_amount_in;
  }

  // Pool's output token balance will go down as the pool sends funds to the user
  let swapped_amount_received = u64::from_le_bytes(transfers[1].data[1..9].try_into().unwrap());
  if let Some(running_token_balance) = running_token_balances.get_mut(&output_token_vault) {
    *running_token_balance -= swapped_amount_received;
  }

  let direction;
  let token_a_address;
  let token_b_address;
  let token_a_vault_address;
  let token_b_vault_address;

  // Lexiographic base58 encoded string sorting happens here
  // input mint is token a, output mint is token b, direction is token a to token b
  if input_token_mint < output_token_mint {
    token_a_address = input_token_mint;
    token_a_vault_address = input_token_vault;
    token_b_address = output_token_mint;
    token_b_vault_address = output_token_vault;
    direction = SwapDirection::AToB;
  }
  // input mint is token b, output mint is token a, direction is token b to token a
  else {
    token_a_address = output_token_mint;
    token_a_vault_address = output_token_vault;
    token_b_address = input_token_mint;
    token_b_vault_address = input_token_vault;
    direction = SwapDirection::BToA;
  }

  // The fee config is the third account

  let fee_fraction_lp = get_cpmm_fee_amount_from_config_account(
    instruction.tx_account_keys[instruction.accounts[2] as usize],
    &market_address,
  );

  let pool_token_a_vault_amount = running_token_balances[&token_a_vault_address];
  let pool_token_b_vault_amount = running_token_balances[&token_b_vault_address];
  let price_b_a_lp =
    LAMPORTS_PER_SOL * pool_token_b_vault_amount as u128 / pool_token_a_vault_amount as u128;
  let price_a_b_lp =
    LAMPORTS_PER_SOL * pool_token_a_vault_amount as u128 / pool_token_b_vault_amount as u128;

  SwapTx {
    pool: Pools::RaydiumCpmm,
    direction,
    block_time,
    slot,
    index,
    atomic_instruction_index,
    // Raydium CPMM fees are 0.25% fixed rate
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
