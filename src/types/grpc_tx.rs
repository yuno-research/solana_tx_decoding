use yellowstone_grpc_proto::solana::storage::confirmed_block::Transaction;
use yellowstone_grpc_proto::solana::storage::confirmed_block::TransactionStatusMeta;

pub struct GrpcTx<'a> {
  pub tx: &'a Transaction,
  pub meta: &'a TransactionStatusMeta,
}