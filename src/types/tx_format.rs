use crate::types::archive_tx::ArchiveTx;
use crate::types::grpc_tx::GrpcTx;
use crate::types::json_rpc_tx::JsonRpcTx;

pub enum TxFormat<'a> {
  Archive(ArchiveTx<'a>),
  Grpc(GrpcTx<'a>),
  JsonRpc(JsonRpcTx<'a>),
}