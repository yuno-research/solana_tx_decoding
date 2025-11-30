use solana_sdk::transaction::VersionedTransaction;
use solana_transaction_status_client_types::UiTransactionStatusMeta;

/// Transaction wrapper for JSON RPC transactions. Contains a reference to the transaction and its
/// metadata from JSON RPC responses.
pub struct JsonRpcTx<'a> {
  pub tx: &'a VersionedTransaction,
  pub meta: &'a UiTransactionStatusMeta,
}
