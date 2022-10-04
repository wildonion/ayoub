





use crate::*; // loading all defined crates, structs and functions from the root crate which is lib.rs in our case







impl CRC20 for Validator{ //-- issuing a FT (fungible token) contract for a validator

    type TokenID = u8;
    type TokenName = String;
    type TotalSupply = u128;
    type Decimal = u8;
    type TokenAddress = String;
    type ExpTime = u64;

    fn mint(&mut self){ //-- self is a mutable pointer to the Validator fields
        //-- minting FT is a transaction and means assigning a token or an asset value with a limited to a wallet address which can be issued by this contract
        let mint_address: Self::TokenAddress = self.recent_transaction.as_ref().unwrap().from_address.clone(); //-- cloning the from_address field of the Transaction struct cause is of type String - for unwrapping the transaction we must first clone it cause it's behind a shared reference which is &mut behind the self parameter which is &mut behind the Option cause recent_transaction is of type Option<Transaction> - we can also use as_ref() method instead of cloning cause the as_ref() will conver the &Option<T> to Option<&T>
    }

    fn transfer_from(&mut self){
        //-- transfer token from a sender to a recipient

    } 

    fn balance_of(&mut self){
        //-- provides the number of tokens held by a given wallet address

    } 
    
    fn approve(&mut self){
        //-- the code that's executed by the contract's method can include calls to other contracts, these trigger more transactions that have the from field set to the contract's address - a token holder gives another address (usually of a smart contract) approval to transfer up to a certain number of tokens, known as an allowance. The token holder uses approve() to provide this information

    }

    fn allowance(&mut self){
        //-- provides the number of tokens allowed to be transferred from a given wallet address by another given wallet address
        
    } 

    fn owner_of(&mut self){
        //-- this function returns the address of the owner of a token. As each ERC-721 token is unique and non-fungible, they are represented on the blockchain by an ID,  other users, contracts, apps can use this ID to determine the owner of the token
    }

    fn burn(&mut self){
        //-- burn some the tokens
    }
}













// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
//                  Messages and enums
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize ,Default, Clone, Debug)]
pub enum Mode{
    #[default] //// enum data types can only have one field as the default value
    Mine, //// Mine field is the default value; utf8 encoded variant is 1
    Stake, //// utf8 encoded variant is 2
    Deposit, //// utf8 encoded variant is 3
    Withdraw, //// utf8 encoded variant is 4
}

#[derive(Clone, Debug)] //-- bounding to Clone and the Debug trait
pub struct Contract { //-- Contract event between two validators on the coiniXerr network; this the message that we'll use between validator actors
    pub id: Uuid,
    pub ttype: u8,
}


#[derive(Clone, Debug)] //-- bounding to Clone and the Debug trait
pub struct UpdateTx { //-- update transaction message to tell the actor to update the last transaction with the new one
    pub id: Uuid,
    pub tx: Option<Transaction>,
}







// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
//                 Validator type actor
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈

#[actor(Contract, UpdateTx)] //-- Validator actor will receive a message of type Contract and UpdateTx
#[derive(Debug, Clone, Serialize, Deserialize)] //-- trait Clone is required to prevent the object of this struct from moving
pub struct Validator {
    pub id: Uuid,
    pub addr: SocketAddr,
    pub recent_transaction: Option<Transaction>, //-- signed the recent_transaction came from the peer
    pub mode: Mode,
    pub ttype_request: Option<u8>,
}


impl Validator{

    pub fn update_transaction(&mut self, transaction: Option<Transaction>){
        self.recent_transaction = transaction;
    }

}







// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
//    implementing the Actors for the Validator type
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈

impl Actor for Validator{

    //// When using the #[actor()] attribute, the actor's Msg associated type should be set to '[DataType]Msg'. 
    //// E.g. if an actor is a struct named MyActor, then the Actor::Msg associated type will be MyActorMsg.
    type Msg = ValidatorMsg; //// Msg associated type is the actor mailbox type and is of type ValidatorMsg which is the Validator type itself; actors can communicate with each other by sending message to each other

    fn recv(&mut self, 
            ctx: &Context<Self::Msg>, //// ctx is the actor system which we can build child actors with it also sender is another actor 
            msg: Self::Msg, 
            sender: Sender){
                
        self.receive(ctx, msg, sender);

    }
}


impl ActorFactoryArgs<(Uuid, SocketAddr, Option<Transaction>, Mode, Option<u8>)> for Validator{

    fn create_args((id, addr, recent_transaction, mode, ttype_request): (Uuid, SocketAddr, Option<Transaction>, Mode, Option<u8>)) -> Self{

        Self { id, addr, recent_transaction, mode, ttype_request }
        
    }

}






// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
//    implementing the Receive types for our actor
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈

impl Receive<Contract> for Validator{ //// implementing the Receive trait for the Validator actor to handle the incoming message of type Contract
    type Msg = ValidatorMsg;

    fn receive(&mut self,
                _ctx: &Context<Self::Msg>, //// ctx is the actor system which we can build child actors with it also sender is another actor 
                _msg: Contract, //-- _msg is of type Contract since we're implementing the Receive trait for the Contract type
                _sender: Sender){
    
        info!("-> {} - message info received with id [{}] and ttype [{}]", chrono::Local::now().naive_local(), _msg.id, _msg.ttype);
        self.ttype_request = Some(_msg.ttype); //// updating the transaction type request using the incoming message of type Contract 
                    
    }

}


impl Receive<UpdateTx> for Validator{ //// implementing the Receive trait for the Validator actor to handle the incoming message of type UpdateTx
    type Msg = ValidatorMsg;

    fn receive(&mut self,
                _ctx: &Context<Self::Msg>, //// ctx is the actor system which we can build child actors with it also sender is another actor 
                _msg: UpdateTx, //-- _msg is of type UpdateTx since we're implementing the Receive trait for the UpdateTx type
                _sender: Sender){
    
        info!("-> {} - message info received with id [{}] and new transaction [{:?}]", chrono::Local::now().naive_local(), _msg.id, _msg.tx.as_ref().unwrap()); //// calling as_ref() method on the _msg.tx to prevent ownership moving
        self.update_transaction(_msg.tx); //// updating the last transaction of a specific validator using the incoming message of type UpdateTx 
                    
    }


}