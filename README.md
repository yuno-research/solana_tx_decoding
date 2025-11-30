# Solana Transaction Decoding

Library for decoding raw Solana transactions from various sources into standardized formats.

## Overview

This library processes raw transaction data from multiple sources (Archive blocks, gRPC streams, JSON RPC) and converts them into standardized types (`SwapTx`, `TokenCreation`) that can be easily used across systems. The decoding works in both live and historical settings, producing consistent output regardless of the transaction source.

## Key Features

- **Multi-format Support**: Handles transactions from Triton One Old Faithful Archive format, Yellowstone gRPC streams, and JSON RPC
- **Parallel Processing**: Writes decoded results to broadcast channels for concurrent downstream processing
- **Multiple Protocol Support**: Identifies and decodes swap transactions from multiple DEX protocols
- **Token Creation Detection**: Extracts new token/pool creation events (Pumpfun bonding curves)
- **Standardized Output**: All transaction formats are converted into unified `SwapTx` and `TokenCreation` types

## Supported Protocols

Swap decoding is supported for:

- Raydium Ammv4
- Raydium Cpmm/Ammv5
- Raydium Launchpad
- Pumpswap Amm
- Pumpfun Bonding Curve

Token creation detection:

- Pumpfun bonding curve creation

## Primary Usage

The main entry point is `analyze_tx`, which accepts a `TxFormat` enum containing any transaction format (Archive, gRPC, or JSON RPC) and writes decoded swap transactions and token creations to broadcast channels.

## Notes

- This library is a dependency and should not be run as a standalone application
- Failed transactions are automatically skipped during analysis. Output will be successful txs (error none on chain) only
