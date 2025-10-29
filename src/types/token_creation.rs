use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;

#[derive(Debug, Clone)]
pub struct TokenCreation {
  pub address: Pubkey,
  pub creator: Pubkey,
  pub market_address: Pubkey,
  pub name: String,
  pub symbol: String,
  pub uri: String,
  pub block_time: u64,
  pub slot: u64,
  pub index: u64,
  pub atomic_instruction_index: u8,
  pub signature: Signature,
}