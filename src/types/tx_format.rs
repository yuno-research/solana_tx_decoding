use crate::types::rpc_tx::RpcTx;
use crate::types::grpc_tx::GrpcTx;

pub enum TxFormat {
  Rpc(RpcTx),
  Grpc(GrpcTx),
}