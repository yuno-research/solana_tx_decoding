use crate::types::token_creation::TokenCreation;
use solana_central::types::instruction::Instruction;
use solana_sdk::signature::Signature;

pub fn process_pf_bonding_curve_create_instruction(
  instruction: &Instruction,
  block_time: u64,
  slot: u64,
  index: u64,
  atomic_instruction_index: u8,
  signature: &Signature,
) -> TokenCreation {
  let data = instruction.data;

  // Skip 8-byte discriminator
  let mut offset = 8;

  // Parse name string (4 bytes length + UTF-8 bytes)
  let name_len = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap()) as usize;
  offset += 4;
  let name = String::from_utf8_lossy(&data[offset..offset + name_len]).to_string();
  offset += name_len;

  // Parse symbol string (4 bytes length + UTF-8 bytes)
  let symbol_len = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap()) as usize;
  offset += 4;
  let symbol = String::from_utf8_lossy(&data[offset..offset + symbol_len]).to_string();
  offset += symbol_len;

  // Parse uri string (4 bytes length + UTF-8 bytes)
  let uri_len = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap()) as usize;
  offset += 4;
  let uri = String::from_utf8_lossy(&data[offset..offset + uri_len]).to_string();

  TokenCreation {
    address: instruction.tx_account_keys[instruction.accounts[0] as usize],
    creator: instruction.tx_account_keys[instruction.accounts[7] as usize],
    market_address: instruction.tx_account_keys[instruction.accounts[2] as usize],
    name,
    symbol,
    uri,
    description: String::new(),
    twitter: String::new(),
    website: String::new(),
    block_time,
    slot,
    index,
    atomic_instruction_index,
    signature: signature.clone(),
  }
}
