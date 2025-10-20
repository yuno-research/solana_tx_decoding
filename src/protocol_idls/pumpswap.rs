use borsh::BorshDeserialize;
use solana_sdk::pubkey::Pubkey;

#[derive(BorshDeserialize)]
pub struct PumpAmmCreatePoolInstructionDataIdl {
  pub discriminator: [u8; 8],
  pub index: u16,
  pub base_amount_in: u64,
  pub quote_amount_in: u64,
  pub coin_creator: Pubkey,
}

/**
 * OLD Pumpswap BUY Event (401 bytes)
 * Used before min_base_amount_out and ix_name were added
 */
#[derive(BorshDeserialize)]
pub struct PumpswapBuyEventIdlOld {
  pub discriminator: [u8; 16],
  pub timestamp: i64,
  pub base_amount_out: u64,
  pub max_quote_amount_in: u64,
  pub user_base_token_reserves: u64,
  pub user_quote_token_reserves: u64,
  pub pool_base_token_reserves: u64,
  pub pool_quote_token_reserves: u64,
  pub quote_amount_in: u64,
  pub lp_fee_basis_points: u64,
  pub lp_fee: u64,
  pub protocol_fee_basis_points: u64,
  pub protocol_fee: u64,
  pub quote_amount_in_with_lp_fee: u64,
  pub user_quote_amount_in: u64,
  pub pool: Pubkey,
  pub user: Pubkey,
  pub user_base_token_account: Pubkey,
  pub user_quote_token_account: Pubkey,
  pub protocol_fee_recipient: Pubkey,
  pub protocol_fee_recipient_token_account: Pubkey,
  pub coin_creator: Pubkey,
  pub coin_creator_fee_basis_points: u64,
  pub coin_creator_fee: u64,
  pub track_volume: bool,
  pub total_unclaimed_tokens: u64,
  pub total_claimed_tokens: u64,
  pub current_sol_volume: u64,
  pub last_update_timestamp: u64,
}

/**
 * NEW Pumpswap BUY Event (416 bytes)
 * Added min_base_amount_out (u64) and ix_name (String)
 * We don't need these fields, so we skip them with padding
 */
#[derive(BorshDeserialize)]
pub struct PumpswapBuyEventIdlCurrent {
  pub discriminator: [u8; 16],
  pub timestamp: i64,
  pub base_amount_out: u64,
  pub max_quote_amount_in: u64,
  pub user_base_token_reserves: u64,
  pub user_quote_token_reserves: u64,
  pub pool_base_token_reserves: u64,
  pub pool_quote_token_reserves: u64,
  pub quote_amount_in: u64,
  pub lp_fee_basis_points: u64,
  pub lp_fee: u64,
  pub protocol_fee_basis_points: u64,
  pub protocol_fee: u64,
  pub quote_amount_in_with_lp_fee: u64,
  pub user_quote_amount_in: u64,
  pub pool: Pubkey,
  pub user: Pubkey,
  pub user_base_token_account: Pubkey,
  pub user_quote_token_account: Pubkey,
  pub protocol_fee_recipient: Pubkey,
  pub protocol_fee_recipient_token_account: Pubkey,
  pub coin_creator: Pubkey,
  pub coin_creator_fee_basis_points: u64,
  pub coin_creator_fee: u64,
  pub track_volume: bool,
  pub total_unclaimed_tokens: u64,
  pub total_claimed_tokens: u64,
  pub current_sol_volume: u64,
  pub last_update_timestamp: u64,
  // Skip the 2 new fields we don't need (15 bytes total)
  pub padding: [u8; 15],
}

/**
 * Pumpswap SELL Event (360 bytes)
 * This event structure has NOT changed - no old/new versions
 */
#[derive(BorshDeserialize)]
pub struct PumpswapSellEventIdl {
  pub discriminator: [u8; 16],
  pub timestamp: i64,
  pub base_amount_in: u64,
  pub min_quote_amount_out: u64,
  pub user_base_token_reserves: u64,
  pub user_quote_token_reserves: u64,
  pub pool_base_token_reserves: u64,
  pub pool_quote_token_reserves: u64,
  pub quote_amount_out: u64,
  pub lp_fee_basis_points: u64,
  pub lp_fee: u64,
  pub protocol_fee_basis_points: u64,
  pub protocol_fee: u64,
  pub quote_amount_out_without_lp_fee: u64,
  pub user_quote_amount_out: u64,
  pub pool: Pubkey,
  pub user: Pubkey,
  pub user_base_token_account: Pubkey,
  pub user_quote_token_account: Pubkey,
  pub protocol_fee_recipient: Pubkey,
  pub protocol_fee_recipient_token_account: Pubkey,
  pub coin_creator: Pubkey,
  pub coin_creator_fee_basis_points: u64,
  pub coin_creator_fee: u64,
}
