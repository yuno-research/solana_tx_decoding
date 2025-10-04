use borsh::BorshDeserialize;
use solana_sdk::pubkey::Pubkey;

#[derive(BorshDeserialize)]
pub struct LaunchpadTradeEventIdl {
  discriminator: [u8; 16],
  pub pool_state: Pubkey,
  pub total_base_sell: u64,
  pub virtual_base: u64,
  pub virtual_quote: u64,
  pub real_base_before: u64,
  pub real_quote_before: u64,
  pub real_base_after: u64,
  pub real_quote_after: u64,
  pub amount_in: u64,
  pub amount_out: u64,
  pub protocol_fee: u64,
  pub platform_fee: u64,
  pub creator_fee: u64,
  pub share_fee: u64,
  // 0 means buy, 1 means sell in this enum
  pub trade_direction: u8,
  pub pool_status: u8,
  pub exact_in: bool,
}


