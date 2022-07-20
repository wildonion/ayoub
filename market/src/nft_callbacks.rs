




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







// NOTE - we'll handle all cross contract call processes coming from each NFT contract in here like calling nft_on_approve() method from the NFT contract
// NOTE - we have to define a trait with a name NonFungibleToken*Receiver in which the * is the name of the process that is triggered from the NFT contract like NonFungibleTokenApprovalsReceiver which is a trait contains all approval methods

















// -----------------------------------------------------------------------------------
//      METHODS THAT WILL BE CALLED FROM NFT CONTRACT USING CROSS CONTRACT CALLS 
// -----------------------------------------------------------------------------------
trait NonFungibleTokenApprovalsReceiver{ //-- this trait will be used to define the approval methods which might be called from the NFT contracts like nft_on_approve() method - when nft_approve() method is called on the NFT contract if the msg param of the nft_approve() method wan't None it'll fire a cross contract call to our marketplace and the following is the method that is invoked     
    fn nft_on_approve(&mut self, token_id: TokenId, owner_id: AccountId, approval_id: u64, msg: String); //-- since we want to create a sale object in this method and mutate the state of the contract we must define the first param as &mut self
}















// -------------------------------------------------------------
//     TRAITS IMPLEMENTATION OF CROSS CONTRACT CALL METHODS 
// -------------------------------------------------------------
#[near_bindgen] //-- implementing the #[near_bindgen] proc macro attribute on the trait implementation for the extended interface (NonFungibleTokenApprovalsReceiver trait) of `MarketContract` struct interface in order to have a compiled wasm trait methods for this contract struct so we can call it from the near cli 
impl NonFungibleTokenApprovalsReceiver for MarketContract{ //-- implementing the NonFungibleTokenApprovalsReceiver trait for our main `MarketContract` struct to extend its interface; bounding the mentioned trait to the `NFTContract` struct to query NFT approval infos
    
    fn nft_on_approve(&mut self, token_id: TokenId, owner_id: AccountId, approval_id: u64, msg: String){ //-- overriding the nft_on_approve() method of the NonFungibleTokenApprovalsReceiver trait - if the seller or the NFT owner wants to sell his/her NFT he/she must call nft_approve() method on his/her NFT contract and pass a none empty message to that method since this method (which is in our marketplace obviously) will fire only if the msg param inside the nft_approve() method wasn't empty (Some(msg)); and if so the NFT owner which is the seller can list his/her NFT on the market by scheduling this call on his/her NFT contract and give the access to the market to transfer and list the NFT on behalf of his/her   
        
        let nft_contract_id = env::predecessor_account_id(); //-- getting the caller id (last caller) of this method since this method will be called from the NFT contract thus the caller is the owner of the NFT contract
        let signer_id = env::signer_account_id();





        //-- the msg contains the price of the NFT in u128 of yocto$NEAR which we must deserialize it to u128 type
        // ...






    
    }

}