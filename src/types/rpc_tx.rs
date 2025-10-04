use solana_sdk::transaction::VersionedTransaction;
use solana_storage_proto::convert::generated::TransactionStatusMeta;

pub struct RpcTx {
  pub tx: VersionedTransaction,
  pub meta: TransactionStatusMeta,
}
