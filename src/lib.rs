//! # Solana Transaction Decoding
//!
//! Library for decoding raw Solana transactions from various sources into standardized formats.
//!
//! This library provides:
//! - Multi-format transaction parsing (Archive, gRPC, JSON RPC)
//! - Instruction classification and decoding
//! - Parallel processing ability with Tokio broadcast channels
//! - Standardized output types (`SwapTx`, `TokenCreation`)
//! - Protocol-specific instruction processors (Raydium, Pumpswap, Pumpfun)
//!
//! ## Usage
//!
//! The main entry point is `analyze_tx`, which takes a `TxFormat` enum (containing any transaction
//! format) and writes decoded swap transactions and token creations to broadcast channels.

mod instruction;
mod tx;
mod types;
mod utilities;

// Re-exports
pub use tx::analyze_tx::analyze_tx;
pub use tx::top_level_instructions_loop::top_level_instructions_loop;
pub use tx::inner_instructions_loop::inner_instructions_loop;
pub use instruction::classify_instruction::classify_instruction;
pub use instruction::raydium::process_raydium_ammv4_swap_instruction::process_raydium_ammv4_swap_instruction;
pub use instruction::raydium::process_raydium_cpmm_swap_instruction::process_raydium_cpmm_swap_instruction;
pub use instruction::raydium::process_raydium_launchpad_swap_instruction::process_raydium_launchpad_swap_instruction;
pub use instruction::pumpswap::process_pumpswap_swap_instruction::process_pumpswap_swap_instruction;
pub use instruction::pumpfun::process_pumpfun_event_instruction::process_pumpfun_event_instruction;
pub use instruction::pumpfun::process_pf_bonding_curve_create_instruction::process_pf_bonding_curve_create_instruction;
pub use instruction::raydium::is_raydium_ammv4_swap_instruction::is_raydium_ammv4_swap_instruction;
pub use instruction::raydium::is_raydium_cpmm_swap_instruction::is_raydium_cpmm_swap_instruction;
pub use instruction::raydium::is_raydium_launchpad_swap_instruction::is_raydium_launchpad_swap_instruction;
pub use instruction::pumpswap::is_pumpswap_swap_instruction::is_pumpswap_swap_instruction;
pub use instruction::pumpfun::is_pumpfun_event_instruction::is_pumpfun_event_instruction;
pub use instruction::pumpfun::is_pf_bonding_curve_create_instruction::is_pf_bonding_curve_create_instruction;
pub use types::tx_format::TxFormat;
pub use types::instruction_type::InstructionType;
pub use types::archive_tx::ArchiveTx;
pub use types::grpc_tx::GrpcTx;
pub use types::json_rpc_tx::JsonRpcTx;
pub use utilities::fetch_token_metadata_from_uri::fetch_token_metadata_from_uri;