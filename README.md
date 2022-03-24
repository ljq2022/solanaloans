# solanaloans

## Solana Installation

For Windows installation, follow these instructions: https://github.com/buildspace/buildspace-projects/tree/main/Solana_And_Web3/en/Section_2/Resources/windows_setup.md <br>
For Mac Installation, follow these instructions: https://github.com/buildspace/buildspace-projects/tree/main/Solana_And_Web3/en/Section_2/Resources/m1_setup.md

## Rust Installation

https://doc.rust-lang.org/book/ch01-01-installation.html

## Solana Local Configuration

Check if Solana is working by typing `solana --version` in your terminal.
Run these commands:

```
solana config set --url localhost
solana config get
```

This should output something like the following:

```
Config File: /Users/flynn/.config/solana/cli/config.yml
RPC URL: http://localhost:8899
WebSocket URL: ws://localhost:8900/ (computed)
Keypair Path: /Users/flynn/.config/solana/id.json
Commitment: confirmed
```

Get your Solana local node running by entering the following in your terminal:

```
solana-test-validator
```

## Anchor Installation

Assuming you already have Node and NPM, install all dependencies with `npm i`
From here, run:

```
cargo install --git https://github.com/project-serum/anchor anchor-cli --locked
anchor --version
npm install @project-serum/anchor @solana/web3.js
```

## Anchor Configuration

Create a key pair using this command:

```
solana-keygen new -k target/deploy/solanaloans-keypair.json
```

Take the result of this and paste it into the program ID for `programs/solanaloans/src/lib.rs` and `Anchor.toml` in the root.
Enter `anchor test` to run the unit tests.
