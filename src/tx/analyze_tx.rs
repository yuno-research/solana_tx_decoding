use crate::tx::top_level_instructions_loop::top_level_instructions_loop;
use crate::types::tx_format::TxFormat;
use solana_central::types::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use std::collections::HashMap;
use std::collections::HashSet;

/**
This is going to be the main function that analyzes raw solana transactions and extracts what we
need from them. For now its just swap txs and bubblemapping but we will extract more later on like
adding/removing liquidity and etc.

This function does not return anything, it writes to a broadcast channel
*/
pub fn analyze_tx(tx: &TxFormat, block_time: u64, slot: u64, index: u64) {
  let mut account_keys;
  let mut top_level_instructions: Vec<Instruction> = Vec::new();
  // Key is the top level instruction index, value is the list of inner instructions yielded by that top level instruction
  let mut inner_instructions: HashMap<u8, Vec<Instruction>> = HashMap::new();
  // Maps a token account address index to the token address (mint) that that token account is for
  let mut ta_mint: HashMap<u8, Pubkey> = HashMap::new();
  // Maps a token account address index to the token balance of that token account
  let mut running_token_balances: HashMap<u8, u64> = HashMap::new();
  let num_required_signatures;
  let signature;

  match tx {
    // Standardize data from rpc txs
    TxFormat::Rpc(tx) => {
      let account_keys_length = tx.tx.message.static_account_keys().len()
        + tx.meta.loaded_writable_addresses.len()
        + tx.meta.loaded_readonly_addresses.len();
      account_keys = Vec::with_capacity(account_keys_length);
      account_keys.extend_from_slice(tx.tx.message.static_account_keys());

      // Add writable and readable addresses loaded from lookup tables for v0 txs if they exist
      for bytes in &tx.meta.loaded_writable_addresses {
        account_keys.push(
          bytes
            .as_slice()
            .try_into()
            .map(Pubkey::new_from_array)
            .unwrap(),
        );
      }
      // Add loaded readonly addresses
      for bytes in &tx.meta.loaded_readonly_addresses {
        account_keys.push(
          bytes
            .as_slice()
            .try_into()
            .map(Pubkey::new_from_array)
            .unwrap(),
        );
      }
      let mut atomic_instruction_index = 0;
      for raw_inst in tx.tx.message.instructions() {
        let inst = Instruction {
          tx_account_keys: &account_keys,
          accounts: &raw_inst.accounts,
          data: &raw_inst.data,
          program_id_index: raw_inst.program_id_index,
          atomic_instruction_index,
        };
        top_level_instructions.push(inst);
        atomic_instruction_index += 1;
      }

      for inner_inst_set in &tx.meta.inner_instructions {
        for inner_inst_raw in &inner_inst_set.instructions {
          let inner_inst = Instruction {
            tx_account_keys: &account_keys,
            accounts: &inner_inst_raw.accounts,
            data: &inner_inst_raw.data,
            program_id_index: inner_inst_raw.program_id_index as u8,
            atomic_instruction_index,
          };
          inner_instructions
            .entry(inner_inst_set.index as u8)
            .or_insert(Vec::new())
            .push(inner_inst);
          atomic_instruction_index += 1;
        }
      }

      for pre_token_balance in &tx.meta.pre_token_balances {
        ta_mint.insert(
          pre_token_balance.account_index as u8,
          Pubkey::from_str_const(&pre_token_balance.mint),
        );
        running_token_balances.insert(
          pre_token_balance.account_index as u8,
          pre_token_balance
            .ui_token_amount
            .as_ref()
            .unwrap()
            .amount
            .parse::<u64>()
            .unwrap(),
        );
      }
      for post_token_balance in &tx.meta.post_token_balances {
        ta_mint.insert(
          post_token_balance.account_index as u8,
          Pubkey::from_str_const(&post_token_balance.mint),
        );
      }
      num_required_signatures = tx.tx.message.header().num_required_signatures;
      signature = Signature::from(tx.tx.signatures[0]);
    }

    // Standardize data from grpc txs
    TxFormat::Grpc(tx) => {
      let message = tx.tx.message.as_ref().unwrap();
      let account_keys_length = message.account_keys.len()
        + tx.meta.loaded_writable_addresses.len()
        + tx.meta.loaded_readonly_addresses.len();
      account_keys = Vec::with_capacity(account_keys_length);
      for bytes in &message.account_keys {
        account_keys.push(
          bytes
            .as_slice()
            .try_into()
            .map(Pubkey::new_from_array)
            .unwrap(),
        );
      }

      // Add writable and readable addresses loaded from lookup tables for v0 txs if they exist
      for bytes in &tx.meta.loaded_writable_addresses {
        account_keys.push(
          bytes
            .as_slice()
            .try_into()
            .map(Pubkey::new_from_array)
            .unwrap(),
        );
      }
      // Add loaded readonly addresses
      for bytes in &tx.meta.loaded_readonly_addresses {
        account_keys.push(
          bytes
            .as_slice()
            .try_into()
            .map(Pubkey::new_from_array)
            .unwrap(),
        );
      }
      let mut atomic_instruction_index = 0;
      for raw_inst in &message.instructions {
        let inst = Instruction {
          tx_account_keys: &account_keys,
          accounts: &raw_inst.accounts,
          data: &raw_inst.data,
          program_id_index: raw_inst.program_id_index as u8,
          atomic_instruction_index,
        };
        top_level_instructions.push(inst);
        atomic_instruction_index += 1;
      }

      for inner_inst_set in &tx.meta.inner_instructions {
        for inner_inst_raw in &inner_inst_set.instructions {
          let inner_inst = Instruction {
            tx_account_keys: &account_keys,
            accounts: &inner_inst_raw.accounts,
            data: &inner_inst_raw.data,
            program_id_index: inner_inst_raw.program_id_index as u8,
            atomic_instruction_index,
          };
          inner_instructions
            .entry(inner_inst_set.index as u8)
            .or_insert(Vec::new())
            .push(inner_inst);
          atomic_instruction_index += 1;
        }
      }

      for pre_token_balance in &tx.meta.pre_token_balances {
        ta_mint.insert(
          pre_token_balance.account_index as u8,
          Pubkey::from_str_const(&pre_token_balance.mint),
        );
        running_token_balances.insert(
          pre_token_balance.account_index as u8,
          pre_token_balance
            .ui_token_amount
            .as_ref()
            .unwrap()
            .amount
            .parse::<u64>()
            .unwrap(),
        );
      }
      for post_token_balance in &tx.meta.post_token_balances {
        ta_mint.insert(
          post_token_balance.account_index as u8,
          Pubkey::from_str_const(&post_token_balance.mint),
        );
      }
      num_required_signatures = message.header.unwrap().num_required_signatures as u8;
      signature = Signature::from(
        <[u8; 64]>::try_from(tx.tx.signatures[0].as_slice())
          .expect("analyze_tx: Signature should be 64 bytes"),
      );
    }
  }

  let mut signers = HashSet::new();
  for i in 0..num_required_signatures {
    signers.insert(account_keys[i as usize]);
  }

  top_level_instructions_loop(
    &top_level_instructions,
    &inner_instructions,
    &account_keys,
    &ta_mint,
    &mut running_token_balances,
    block_time,
    slot,
    index,
    &signers,
    &signature,
  );
}
