
# 🧧 BlueRangene Family Tree NFT Marketplace Smart Contract on NEAR

### ☢️ Environment Setup

1. Install wasm-opt using ```sudo npm i wasm-opt -g``` command
2. Install Rust from https://rustup.rs/
3. Install WSL and ```sudo apt update && sudo apt install build-essential```
4. Install near cli using ```npm install -g near-cli``` command
5. Create near [testnet](https://wallet.testnet.near.org/) or [mainnet](https://wallet.near.org/) account

> For contract method `calls` and `views` see the list of all available APIs using `ayoub` PaaS cli with `./ayoub.sh list --api --controller nft` command.

> The caller must have called the `new_*()` method in first call of the contract in order to initialize the state of the contract on chain and use other methods, calling this method will panic on second call.

> Market contract will use a cross contract call to the NFT contract which is deployed on the minter contract actor account for transferring and minting an NFT. 

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

# 📌 TODOs

* Market Royalty per Collection Minting 

* Minting a Collection Contains Many NFTs in a single Transaction Gas Fee inside `nft_mint()` Method 

* Multiple NFT AirDrop Feature in a single Transaction Gas Fee using `batch` functions

* Multiple Offer Feature for Biddings, Auctions and Buying NFTs like `gem.xyz` in a single Transaction Gas Fee using

* Should We Call `nft_mint` Method on Update an NFT of a Collection?

* Should We Pass the Royalty of the Minter When We're Calling `nft_mint` Method?

* Should We Deploy the NFT Contract on Every Account that Wants to Mint NFT?
