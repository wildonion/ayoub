
# 📑 NEAR Smart Contracts

### Environment Setup
1. Install Rust from https://rustup.rs/
2. Install WSL and ```sudo apt update && sudo apt install build-essential```
3. Install near cli using ```npm install -g near-cli``` command
4. Run ```rustup target add wasm32-unknown-unknown```
5. Create near [testnet](https://wallet.testnet.near.org/) or [mainnet](https://wallet.near.org/) account

> To deploy on _mainnet_ you should have an account on mainnet like _wildonion.near_.

> For calling contract methods, the first account name must be the owner of the contract and the second one would be either the contract owner account or another account name. 

> For calling private method inside the contract, `current_account_id` must be equal to `predecessor_account_id` (account of the contract).

> Only one contract per account is possible; in order to have multiple contracts we must use sub accounts.

> For contract method calls and views see the list of all available APIs in using _ayoub_ PaaS cli with `./ayoub.sh list --api --controller nft` command.

### Compile
```
$ sudo chmod +x build.sh && ./build.sh 
```

### Deplopy
```
$ sudo chmod +x deploy.sh && ./deploy.sh
```