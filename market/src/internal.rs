



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











impl MarketContract{ //-- we've defined the following methods of the `MarketContract` struct in this crate cause this crate is related to all internal calculation functions and methods - we don't need to add #[near_bindgen] proc macro attribute on this impl cause these are none exporting methods and won't compile to wasm to call them from cli 

    pub fn internal_remove_sale(&mut self, nft_contract_id: AccountId, token_id: TokenId) -> Sale{
        let contract_and_token_id = format!("{}{}{}", &nft_contract_id, DELIMETER, token_id); //-- creating the unique id for the sale object from nft_contract_id and the token_id
        let sale = match self.sales.remove(&contract_and_token_id){ //-- removing the sale object related to a unique sale id
            Some(sale) => {
                sale //-- this sale object contains the owner_id which we can use it to find the set of all sale ids for its owner 
            },
            None => { //-- means there is no set related to this unique sale id
                env::panic_str("Found No Sale"); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unknown size at compile time we must borrow it by taking a pointer to its location
            }
        };
        
        
        let mut sale_ids = match self.by_owner_id.get(&sale.owner_id){ //-- getting the set of all sale ids related to an owner_id 
            Some(sale_ids) => {
                sale_ids //-- returning the set of all sale ids related to a specific owner
            },
            None => {
                env::panic_str("Found No Sale Ids"); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unknown size at compile time we must borrow it by taking a pointer to its location
            },
        };
        let mut nft_ids = match self.by_nft_contract_id.get(&nft_contract_id){ //-- getting the set of all nft ids related to a specific owner
            Some(nft_ids) => {
                nft_ids //-- returning the set of all nft ids related to a specific owner
            },
            None => {
                env::panic_str("Found No Nft Ids"); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unknown size at compile time we must borrow it by taking a pointer to its location
            }
        };
        sale_ids.remove(&contract_and_token_id); //-- removing the created unique sale id from the set of all sale ids related to its owner
        nft_ids.remove(&token_id); //-- removing the NFT from the found nft_ids set for a specific owner related to the passed in token_id  


        
        if sale_ids.is_empty(){ //-- if the found set of all sale ids was empty we have to remove the sale.owner_id entry from the self.by_owner_id collection 
            self.by_owner_id.remove(&sale.owner_id);
        } else{ //-- if the found set of all sale ids was'nt empty we have to insert the updated sale ids set (since we've removed the a sale id from it) back to the self.by_owner_id collection 
            self.by_owner_id.insert(&sale.owner_id, &sale_ids); //-- inserting the updated sale_ids set back into the self.by_owner_id collection
        }
        if nft_ids.is_empty(){ //-- if the found set of all nft ids was empty we have to remove the passed in nft_contract_id entry from the self.by_nft_contract_id collection
            self.by_nft_contract_id.remove(&nft_contract_id);
        } else{ //-- if the found set of all nft ids was'nt empty we have to insert the updated nft ids set (since we've removed the a nft id from it) back to the self.by_nft_contract_id collection 
            self.by_nft_contract_id.insert(&nft_contract_id, &nft_ids); //-- inserting the updated nft_ids set back into the self.by_nft_contract_id collection
        }

        sale //-- returning the removed sale object

    }

}