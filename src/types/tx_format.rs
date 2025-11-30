use crate::types::archive_tx::ArchiveTx;
use crate::types::grpc_tx::GrpcTx;
use crate::types::json_rpc_tx::JsonRpcTx;

/// Enum representing different transaction source formats. Allows the decoding system to handle
/// transactions from multiple sources (Archive blocks of Triton One Old Faithful CAR format used
/// in `solana_car`, gRPC streams, JSON RPC) in a unified way.
pub enum TxFormat<'a> {
  Archive(ArchiveTx<'a>),
  Grpc(GrpcTx<'a>),
  JsonRpc(JsonRpcTx<'a>),
}