



/*



Coded by



 █     █░ ██▓ ██▓    ▓█████▄  ▒█████   ███▄    █  ██▓ ▒█████   ███▄    █ 
▓█░ █ ░█░▓██▒▓██▒    ▒██▀ ██▌▒██▒  ██▒ ██ ▀█   █ ▓██▒▒██▒  ██▒ ██ ▀█   █ 
▒█░ █ ░█ ▒██▒▒██░    ░██   █▌▒██░  ██▒▓██  ▀█ ██▒▒██▒▒██░  ██▒▓██  ▀█ ██▒
░█░ █ ░█ ░██░▒██░    ░▓█▄   ▌▒██   ██░▓██▒  ▐▌██▒░██░▒██   ██░▓██▒  ▐▌██▒
░░██▒██▓ ░██░░██████▒░▒████▓ ░ ████▓▒░▒██░   ▓██░░██░░ ████▓▒░▒██░   ▓██░
░ ▓░▒ ▒  ░▓  ░ ▒░▓  ░ ▒▒▓  ▒ ░ ▒░▒░▒░ ░ ▒░   ▒ ▒ ░▓  ░ ▒░▒░▒░ ░ ▒░   ▒ ▒ 
  ▒ ░ ░   ▒ ░░ ░ ▒  ░ ░ ▒  ▒   ░ ▒ ▒░ ░ ░░   ░ ▒░ ▒ ░  ░ ▒ ▒░ ░ ░░   ░ ▒░
  ░   ░   ▒ ░  ░ ░    ░ ░  ░ ░ ░ ░ ▒     ░   ░ ░  ▒ ░░ ░ ░ ▒     ░   ░ ░ 
    ░     ░      ░  ░   ░        ░ ░           ░  ░      ░ ░           ░ 
                      ░                                                  



*/



use crate::*; // loading all defined crates, structs and functions from the root crate which is lib.rs in our case


// NOTE - all gas fees are in gas unit amount which will be attached to a specific call

pub const GAS_FOR_NFT_APPROVE: Gas = Gas(10_000_000_000_000); //-- the required gas fee for cross contract call nft_on_approve() method on the market contract actor account
pub const GAS_FOR_RESOLVE_TRANSFER: Gas = Gas(10_000_000_000_000);  //-- the required gas to resolve the result of cross contract call nft_on_approve() 
pub const GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(25_000_000_000_000 + GAS_FOR_RESOLVE_TRANSFER.0); //-- GAS_FOR_RESOLVE_TRANSFER.0 return the value in u64 which is unsigned integer cause 25_000_000_000_000 is of type u64
pub const NO_DEPOSIT: Balance = 0;
pub const IO_BUFFER_SIZE: u16 = 1024;
pub const NFT_METADATA_SPEC: &str = "1.0.0"; //-- the standard version
pub const NFT_STANDARD_NAME: &str = "nep171"; //-- the NFT standard name 
pub type TokenId = String;