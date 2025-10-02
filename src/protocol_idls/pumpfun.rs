use borsh::BorshDeserialize;
use solana_sdk::pubkey::Pubkey;

/**
Current event IDL used in pumpfun bonding curve event, will always be 266 bytes, example tx:
Post creator fee addition event: 35Vbdm6aMboKYBJcrqKVLoaxnJgRJGe9iJ4nNovWacMeVxDRqeSkSfFDjVy16ywT6jX6CiGRQawKFKE3q8UFzAdo
Post length: 266
*/
#[derive(BorshDeserialize)]
pub struct PfTradeEventIdlCurrent {
  // 16 byte discriminator
  pub padding: [u8; 16],
  pub mint: Pubkey,
  /*
  Sol amount is how much SOL was actually sent to the protocol/bonding curve and this quantity
  EXCLUDES fees which the user additionally sends. So this is NOT the total amount of sol involved in the swap
  */
  pub sol_amount: u64,
  pub token_amount: u64,
  pub is_buy: bool,
  pub user: Pubkey,
  pub timestamp: i64,
  // These values are all after the swap takes place
  pub virtual_sol_reserves: u64,
  pub virtual_token_reserves: u64,
  pub real_sol_reserves: u64,
  pub real_token_reserves: u64,
  pub fee_recipient: Pubkey,
  pub fee_basis_points: u64,
  pub fee: u64,
  pub creator: Pubkey,
  pub creator_fee_basis_points: u64,
  pub creator_fee: u64,
  pub track_volume: bool,
  pub track_unclaimed_tokens: u64,
  pub total_claimed_tokens: u64,
  pub current_sol_volume: u64,
  pub last_update_timestamp: u64,
}

/**
Old event
Pre creator fee addition event: 4szssqMqm7tSkdQnBRDt8MijHu32GbLDnJxYijfinoo3sJnFAFya8oSE3sihmU3NA5U7DToDU5uE9jjmQCVn1yGs
Pre length: 137
*/
#[derive(BorshDeserialize)]
pub struct PfTradeEventIdlOld {
  pub discriminator: [u8; 16],
  pub mint: Pubkey,
  pub sol_amount: u64,
  pub token_amount: u64,
  pub is_buy: bool,
  pub user: Pubkey,
  pub timestamp: i64,
  pub virtual_sol_reserves: u64,
  pub virtual_token_reserves: u64,
}
