// pub mod instruction;
// pub mod protocol_idls;
// pub mod tx;
// pub mod types;

// use dotenv::dotenv;
// use solana_central::central_context::central_context::CentralContext;
// use solana_client::rpc_config::RpcTransactionConfig;
// use solana_sdk::commitment_config::CommitmentConfig;
// use solana_sdk::signature::Signature;
// use solana_transaction_status_client_types::UiTransactionEncoding;
// use solana_tx_decoding::tx::analyze_tx::analyze_tx;
// use solana_tx_decoding::types::json_rpc_tx::JsonRpcTx;
// use solana_tx_decoding::types::tx_format::TxFormat;
// use solana_tx_decoding::utilities::fetch_token_metadata_from_uri::fetch_token_metadata_from_uri;
// use std::str::FromStr;
// use tokio::sync::broadcast;

// #[tokio::main]
// async fn main() {
//   dotenv().ok();

//   let central_context = CentralContext::new();

//   // Create a reusable HTTP client for fetching token metadata
//   let http_client = reqwest::Client::new();

//   // Parse the signature
//   let sig_str =
//     "3pL6T9pQ1XsxjQe24vUtjsUH42vzntzHJq8LQMWbfFXhjTTpLuaFgW7yB63D9r2jQQzV4Qqp1zhDjJ68zaaTj7W7";
//   let signature = Signature::from_str(sig_str).expect("Failed to parse signature");

//   // Configure the RPC request to get the transaction in base64 encoding
//   let config = RpcTransactionConfig {
//     encoding: Some(UiTransactionEncoding::Base64),
//     commitment: Some(CommitmentConfig::confirmed()),
//     max_supported_transaction_version: Some(0),
//   };

//   // Fetch the transaction
//   let encoded_tx = central_context
//     .json_rpc_client
//     .get_transaction_with_config(&signature, config)
//     .expect("Failed to get transaction");

//   // Decode the transaction and extract versioned transaction and metadata
//   let versioned_tx = encoded_tx
//     .transaction
//     .transaction
//     .decode()
//     .expect("Failed to decode transaction");
//   let meta = encoded_tx
//     .transaction
//     .meta
//     .as_ref()
//     .expect("Transaction metadata not found");

//   // Create broadcast channels for output
//   let (swap_tx_sender, mut swap_tx_receiver) = broadcast::channel(10);
//   let (token_create_sender, mut token_create_receiver) = broadcast::channel(10);

//   // Wrap in JsonRpcTx and TxFormat
//   let json_rpc_tx = JsonRpcTx {
//     tx: &versioned_tx,
//     meta,
//   };
//   let tx_format = TxFormat::JsonRpc(json_rpc_tx);

//   // Analyze the transaction
//   analyze_tx(
//     &tx_format,
//     &swap_tx_sender,
//     &token_create_sender,
//     encoded_tx.block_time.unwrap_or(0) as u64,
//     encoded_tx.slot,
//     0, // index (position in block)
//   );

//   // Receive and print any token create transactions
//   tokio::spawn(async move {
//     while let Ok(token_create_tx) = token_create_receiver.recv().await {
//       println!("Token create tx: {:?}", token_create_tx);
//       let (description, twitter, website) =
//         fetch_token_metadata_from_uri(&http_client, &token_create_tx.uri).await;
//       println!("Description: {}", description);
//       println!("Twitter: {}", twitter);
//       println!("Website: {}", website);
//     }
//   });

//   // Receive and print any swap transactions
//   tokio::spawn(async move {
//     while let Ok(swap_tx) = swap_tx_receiver.recv().await {
//       println!("Swap tx: {:?}", swap_tx);
//     }
//   })
//   .await
//   .expect("Task failed");
// }
