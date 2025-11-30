use solana_sdk::transaction::VersionedTransaction;
use solana_storage_proto::convert::generated::TransactionStatusMeta;

/// Transaction wrapper for Archive format transactions. Contains a reference to the transaction
/// and its metadata from Old Faithful CAR format archive block storage.
pub struct ArchiveTx<'a> {
  pub tx: &'a VersionedTransaction,
  pub meta: &'a TransactionStatusMeta,
}
