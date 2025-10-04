use yellowstone_grpc_proto::solana::storage::confirmed_block::Transaction;
use yellowstone_grpc_proto::solana::storage::confirmed_block::TransactionStatusMeta;

pub struct GrpcTx {
  pub tx: Transaction,
  pub meta: TransactionStatusMeta,
}