use solana_sdk::transaction::VersionedTransaction;
use solana_storage_proto::convert::generated::TransactionStatusMeta;

pub struct ArchiveTx<'a> {
  pub tx: &'a VersionedTransaction,
  pub meta: &'a TransactionStatusMeta,
}
