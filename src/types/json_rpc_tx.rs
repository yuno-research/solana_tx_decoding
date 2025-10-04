use solana_transaction_status_client_types::UiTransactionStatusMeta;
use solana_sdk::transaction::VersionedTransaction;
pub struct JsonRpcTx<'a> {  
  pub tx: &'a VersionedTransaction,
  pub meta: &'a UiTransactionStatusMeta,
}