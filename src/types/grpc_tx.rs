use yellowstone_grpc_proto::solana::storage::confirmed_block::Transaction;
use yellowstone_grpc_proto::solana::storage::confirmed_block::TransactionStatusMeta;

/// Transaction wrapper for Yellowstone gRPC stream transactions. Contains a reference to the
/// transaction and its metadata from Yellowstone gRPC streams.
pub struct GrpcTx<'a> {
  pub tx: &'a Transaction,
  pub meta: &'a TransactionStatusMeta,
}