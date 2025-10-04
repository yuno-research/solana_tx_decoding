# Solana Transaction Decoding

This repository is responsible for all things related to decoding raw transaction data off of the Solana blockchain and process it into standardized types that we can easily work with and pass around within systems. The decoding is meant to work in both a live and historical setting and the standardized types like `SwapTx` yielded will ultimately all have the same format regardless of where they were sourced from. What is important to note is that the `SwapTx` type is stored in the `solana_central` repository as other systems will be using this type. However this system is specifically responsible for converting raw blockchain data into this standardized type that everyone else can understand easily.

This system is a dependency and should not be run in a standalone context

the parser function should take in tx 