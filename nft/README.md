
# 🧧 BlueRangene Family Tree NFT Smart Contract on NEAR

### ☢️ Environment Setup

1. Install wasm-opt using ```sudo npm i wasm-opt -g``` command
2. Install Rust from https://rustup.rs/
3. Install WSL and ```sudo apt update && sudo apt install build-essential```
4. Install near cli using ```npm install -g near-cli``` command
5. Create near [testnet](https://wallet.testnet.near.org/) or [mainnet](https://wallet.near.org/) account

> For contract method `calls` and `views` see the list of all available APIs using `ayoub` PaaS cli with `./ayoub.sh list --api --controller nft` command.

> The caller must have called the `new_*()` method in first call of the contract in order to initialize the state of the contract on chain and use other methods, calling this method will panic on second call.

> This contract has an `owner_id` field which is the who is the signer and the owner of deploying process of this contract, also is the owner of all the NFTs that will be minted on this contract actor account to sell them on the marketplace.

> Since this contract will be deployed on every contract actor account who wants to mint his/her all NFTs on his `account_id` to sell them on the marketplace thus the marketplace needs to be an approved `account_id` for the owner to transfer or list his/her all NFTs on behalf of him/her in there.

> The marketplace can make a cross contract call to all implemented methods in this contract (which is deployed on minter or creator contract actor account) like approval and transfer methods to sell the NFT on behalf of the owner.

> To update the state of the contract in production like migrating to a new data structure see https://www.near-sdk.io/upgrading/production-basics

### Compile Contract
```
$ sudo chmod +x build.sh && ./build.sh 
```

### Deplopy Contract
```
$ sudo chmod +x deploy.sh && ./deploy.sh
```

### Test Methods
```
$ sudo chmod +x test.sh && ./test.sh
```