


use crate::*; // load all defined crates, structs and functions from the root crate which is lib.rs in our case



const GAS_FOR_RESOLVE_TRANSFER: Gas = 10_000_000_000_000; 
const GAS_FOR_NFT_TRANSFER_CALL: Gas = 25_000_000_000_000 + GAS_FOR_RESOLVE_TRANSFER as Gas;
const NO_DEPOSIT: Balance = 0;