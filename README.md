# ☣️ Build Ayoub Servers

* ☢️ To run the `event` server just change the `CURRENT_SERVICE` variable to `event` value.

* ☢️ To run the `auth` server just change the `CURRENT_SERVICE` variable to `auth` value.

* ☢️ To run the `game` server just change the `CURRENT_SERVICE` variable to `game` value.

```console
$ cargo build --bin ayoub --release
```

# 📑 Solana Smart Contracts

### Environment Setup
1. Install Rust from https://rustup.rs/
2. Install Solana from https://project-serum.github.io/anchor/getting-started/installation.html#install-rust
```
$ solana config set --url https://api.devnet.solana.com
$ mkdir solana-wallet
$ solana-keygen new --outfile solana-wallet/keypair.json
$ solana airdrop 1 $(solana-keygen pubkey solana-wallet/keypair.json)
$ solana config get
$ solana account $(solana-keygen pubkey solana-wallet/keypair.json)
```

### Build and test for program compiled natively
```
$ cargo build
$ cargo test
```

### Build and test the program compiled for BPF
```
$ cargo build-bpf
$ cargo test-bpf
```

### Deplopy the program
```
$ solana deploy -v --keypair solana-wallet/keypair.json target/deploy/smarties.so
```

### Show program info
```
$ solana program show --keypair solana-wallet/keypair.json <PROGRAM_ID>
```

# 📌 TODOs

* postman collection

* server documentaion
