





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
//                  messages events
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

#[derive(Clone, Debug)] //-- bounding to Clone and the Debug trait
pub struct UpdateMode { //-- update mode message to tell the actor to update the validator mode with the new one
    pub id: Uuid,
    pub mode: Option<ValidatorMode>,
}

#[derive(Clone, Debug)] //-- bounding to Clone and the Debug trait
pub struct Communicate{
    pub id: Uuid,
    pub cmd: Cmd,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, Default)]
pub enum Cmd{
    #[default] //// enum data types can only have one field as the default value
    GetValidatorUuid, //// Mine field is the default value; utf8 encoded variant is 0
    GetAddr, //// utf8 encoded variant is 1
    GetRecentTx, //// utf8 encoded variant is 2
    GetMode, //// utf8 encoded variant is 3
}

#[derive(Clone, Debug)]
pub struct ValidatorJoined(pub Uuid); //// a message event to broadcast it by the channel to all validator subscriber actors about joining a new validator - first element of this struct is the validator uuid

#[derive(Clone, Debug)]
pub struct ValidatorUpdated(pub Uuid); //// a message event to broadcast it by the channel to all validator subscriber actors about udpating a validator - first element of this struct is the validator uuid

#[derive(Clone, Debug)]
pub struct UpdateValidatorAboutMempoolTx(pub Uuid); //// a message event to broadcast it by the channel to all validator subscriber actors about incoming a new transaction inside the mempool - first element of this struct is the transaction uuid

#[derive(Clone, Debug)]
pub struct UpdateValidatorAboutMiningProcess(pub Uuid); //// a message event to broadcast it by the channel to all validator subscriber actors about mining process - first element of this struct is the block uuid













// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
//                 Validator type actor
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈

#[actor(Communicate, Contract, UpdateTx, UpdateMode, UpdateValidatorAboutMempoolTx, UpdateValidatorAboutMiningProcess)] //-- Validator actor will receive a message from other actors or a channel to subscribe to of type Communicate, Contract, UpdateTx, ValidatorJoined, ValidatorUpdated, UpdateValidatorAboutMempoolTx and UpdateValidatorAboutMiningProcess
#[derive(Debug, Clone, Serialize, Deserialize)] //-- trait Clone is required to prevent the object of this struct from moving
pub struct Validator {
    pub id: Uuid,
    pub addr: SocketAddr,
    pub recent_transaction: Option<Transaction>, //-- signed the recent_transaction came from the peer
    pub mode: Mode,
    pub ttype_request: Option<u8>,
}


impl Validator{

    pub fn set_transaction(&mut self, transaction: Option<Transaction>){
        self.recent_transaction = transaction;
    }

    pub fn set_mode(&mut self, mode: ValidatorMode){
        self.mode = mode;
    }

    pub fn get_uuid(&self) -> Option<Uuid>{
        Some(self.id.clone())
    }

    pub fn get_mode(&self) -> Option<ValidatorMode>{
        Some(self.mode.clone())
    }

    pub fn get_recent_transaction(&self) -> Option<Transaction>{
        self.recent_transaction.clone()
    }

    pub fn get_addr(&self) -> Option<SocketAddr>{
        Some(self.addr.clone())
    }

}














// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
//    implementing the Actor for the Validator type
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈

impl Actor for Validator{

    //// Validator actor must support the message type of the channels related to the validator message events (ValidatorJoined, ValidatorUpdated, UpdateValidatorAboutMempoolTx, UpdateValidatorAboutMiningProcess) that they want to subscribe to
    //// When using the #[actor()] attribute, the actor's Msg associated type should be set to '[DataType]Msg'. 
    //// E.g. if an actor is a struct named MyActor, then the Actor::Msg associated type will be MyActorMsg.
    type Msg = ValidatorMsg; //// we can access all the message event actors which has defined for Validator using ValidatorMsg::Communicate, ValidatorMsg::Contract, ValidatorMsg::UpdateTx, ValidatorMsg::UpdateMode, ValidatorMsg::UpdateValidatorAboutMempoolTx, ValidatorMsg::UpdateValidatorAboutMiningProcess //// Msg associated type is the actor mailbox type and is of type ValidatorMsg which is the Validator type itself; actors can communicate with each other by sending message to each other

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










//// we must first define the event then impl its handler for our actor
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈  
//      message event receive handlers for the Validator actor
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈

impl Receive<Contract> for Validator{ //// implementing the Receive trait for the Validator actor to handle the incoming message of type Contract
    type Msg = ValidatorMsg; //// we can access all the message event actors which has defined for Validator using ValidatorMsg::Communicate, ValidatorMsg::Contract, ValidatorMsg::UpdateTx, ValidatorMsg::UpdateMode, ValidatorMsg::UpdateValidatorAboutMempoolTx, ValidatorMsg::UpdateValidatorAboutMiningProcess

    fn receive(&mut self,
                _ctx: &Context<Self::Msg>, //// ctx is the actor system which we can build child actors with it also sender is another actor 
                _msg: Contract, //-- _msg is of type Contract since we're implementing the Receive trait for the Contract type
                _sender: Sender){
    
        info!("-> {} - message info received with id [{}] and ttype [{}]", chrono::Local::now().naive_local(), _msg.id, _msg.ttype);
        self.ttype_request = Some(_msg.ttype); //// updating the transaction type request using the incoming message of type Contract 
                    
    }

}


impl Receive<UpdateTx> for Validator{ //// implementing the Receive trait for the Validator actor to handle the incoming message of type UpdateTx
    type Msg = ValidatorMsg; //// we can access all the message event actors which has defined for Validator using ValidatorMsg::Communicate, ValidatorMsg::Contract, ValidatorMsg::UpdateTx, ValidatorMsg::UpdateMode, ValidatorMsg::UpdateValidatorAboutMempoolTx, ValidatorMsg::UpdateValidatorAboutMiningProcess

    fn receive(&mut self,
                _ctx: &Context<Self::Msg>, //// ctx is the actor system which we can build child actors with it also sender is another actor 
                _msg: UpdateTx, //-- _msg is of type UpdateTx since we're implementing the Receive trait for the UpdateTx type
                _sender: Sender){
    
        info!("-> {} - message info received with id [{}] and new transaction [{:?}]", chrono::Local::now().naive_local(), _msg.id, _msg.tx.as_ref().unwrap()); //// calling as_ref() method on the _msg.tx to prevent ownership moving
        self.set_transaction(_msg.tx); //// updating the last transaction of a specific validator using the incoming message of type UpdateTx 
                    
    }

}


impl Receive<UpdateMode> for Validator{ //// implementing the Receive trait for the Validator actor to handle the incoming message of type UpdateMode
    type Msg = ValidatorMsg; //// we can access all the message event actors which has defined for Validator using ValidatorMsg::Communicate, ValidatorMsg::Contract, ValidatorMsg::UpdateTx, ValidatorMsg::UpdateMode, ValidatorMsg::UpdateValidatorAboutMempoolTx, ValidatorMsg::UpdateValidatorAboutMiningProcess

    fn receive(&mut self,
                _ctx: &Context<Self::Msg>, //// ctx is the actor system which we can build child actors with it also sender is another actor 
                _msg: UpdateMode, //-- _msg is of type UpdateMode since we're implementing the Receive trait for the UpdateMode type
                _sender: Sender){
    
        info!("-> {} - message info received with id [{}] and new transaction [{:?}]", chrono::Local::now().naive_local(), _msg.id, _msg.mode.as_ref().unwrap()); //// calling as_ref() method on the _msg.tx to prevent ownership moving
        self.set_mode(_msg.mode.unwrap()); //// updating the last transaction of a specific validator using the incoming message of type UpdateMode 
                    
    }
    
}


impl Receive<Communicate> for Validator{ //// implementing the Receive trait for the Validator actor to handle the incoming message of type Communicate
    type Msg = ValidatorMsg; //// we can access all the message event actors which has defined for Validator using ValidatorMsg::Communicate, ValidatorMsg::Contract, ValidatorMsg::UpdateTx, ValidatorMsg::UpdateMode, ValidatorMsg::UpdateValidatorAboutMempoolTx, ValidatorMsg::UpdateValidatorAboutMiningProcess

    fn receive(&mut self,
                _ctx: &Context<Self::Msg>, //// ctx is the actor system which we can build child actors with it also sender is another actor 
                _msg: Communicate, //-- _msg is of type Communicate since we're implementing the Receive trait for the Communicate type
                _sender: Sender){
    
        info!("-> {} - message info received with id [{}] and command [{:?}]", chrono::Local::now().naive_local(), _msg.id, _msg.cmd);
        match _msg.cmd{
            Cmd::GetAddr => {
                info!("-> {} - getting validator address with id [{}]", chrono::Local::now().naive_local(), self.id);
                let validator_address = self.get_addr();
                _sender
                    .as_ref() //// convert to Option<&T> - we can also use clone() method instead of as_ref() method in order to borrow the content inside the Option to prevent the content from moving and loosing ownership
                    .unwrap()
                    .try_tell(
                        validator_address, //// sending the validator_address as the response message from this actor (not tha main() function)
                        Some(_ctx.myself().into()) //// to the actor or the caller itself
                    );
            },
            Cmd::GetMode => {
                info!("-> {} - getting validator mode with id [{}]", chrono::Local::now().naive_local(), self.id);
                let validator_mode = self.get_mode();
                _sender
                    .as_ref() //// convert to Option<&T> - we can also use clone() method instead of as_ref() method in order to borrow the content inside the Option to prevent the content from moving and loosing ownership
                    .unwrap()
                    .try_tell(
                        validator_mode, //// sending the validator_mode as the response message from this actor (not tha main() function)
                        Some(_ctx.myself().into()) //// to the actor or the caller itself
                    );
            },
            Cmd::GetRecentTx => {
                info!("-> {} - getting the recent transaction of the validator with id [{}]", chrono::Local::now().naive_local(), self.id);
                let validator_recent_transaction = self.get_recent_transaction();
                _sender
                    .as_ref() //// convert to Option<&T> - we can also use clone() method instead of as_ref() method in order to borrow the content inside the Option to prevent the content from moving and loosing ownership
                    .unwrap()
                    .try_tell(
                        validator_recent_transaction, //// sending the validator_recent_transaction as the response message from this actor (not tha main() function)
                        Some(_ctx.myself().into()) //// to the actor or the caller itself
                    );
            },
            _ => { //// Get Uuid
                info!("-> {} - getting the slot of the parachain with id [{}]", chrono::Local::now().naive_local(), self.id);
                let validator_uuid = self.get_uuid();
                _sender
                    .as_ref() //// convert to Option<&T> - we can also use clone() method instead of as_ref() method in order to borrow the content inside the Option to prevent the content from moving and loosing ownership
                    .unwrap()
                    .try_tell(
                        validator_uuid, //// sending the validator_uuid as the response message from this actor (not tha main() function)
                        Some(_ctx.myself().into()) //// to the actor or the caller itself
                    );
            }
        }
                    
    }
    
}


impl Receive<ValidatorJoined> for Validator{ //// implementing the Receive trait for the Validator actor to handle the incoming message of type ValidatorJoined
    type Msg = ValidatorMsg; //// we can access all the message event actors which has defined for Validator using ValidatorMsg::Communicate, ValidatorMsg::Contract, ValidatorMsg::UpdateTx, ValidatorMsg::UpdateMode, ValidatorMsg::UpdateValidatorAboutMempoolTx, ValidatorMsg::UpdateValidatorAboutMiningProcess

    fn receive(&mut self,
                _ctx: &Context<Self::Msg>, //// ctx is the actor system which we can build child actors with it also sender is another actor 
                _msg: ValidatorJoined, //-- _msg is of type ValidatorJoined since we're implementing the Receive trait for the ValidatorJoined type
                _sender: Sender){
    
        info!("-> {} - new validator joined with id [{}]", chrono::Local::now().naive_local(), _msg.0); //// ValidatorJoined is a tuple like struct so we have to get the first elem of it using .0
        
        
        // other logics goes here
        // ...
                    
    }
    
}


impl Receive<ValidatorUpdated> for Validator{ //// implementing the Receive trait for the Validator actor to handle the incoming message of type ValidatorUpdated
    type Msg = ValidatorMsg; //// we can access all the message event actors which has defined for Validator using ValidatorMsg::Communicate, ValidatorMsg::Contract, ValidatorMsg::UpdateTx, ValidatorMsg::UpdateMode, ValidatorMsg::UpdateValidatorAboutMempoolTx, ValidatorMsg::UpdateValidatorAboutMiningProcess

    fn receive(&mut self,
                _ctx: &Context<Self::Msg>, //// ctx is the actor system which we can build child actors with it also sender is another actor 
                _msg: ValidatorUpdated, //-- _msg is of type ValidatorUpdated since we're implementing the Receive trait for the ValidatorUpdated type
                _sender: Sender){
    
        info!("-> {} - validator with id [{}] updated", chrono::Local::now().naive_local(), _msg.0); //// ValidatorJoined is a tuple like struct so we have to get the first elem of it using .0
        
        
        // other logics goes here
        // ...
                    
    }
}


impl Receive<UpdateValidatorAboutMempoolTx> for Validator{ //// implementing the Receive trait for the Validator actor to handle the incoming message of type UpdateValidatorAboutMempoolTx
    type Msg = ValidatorMsg; //// we can access all the message event actors which has defined for Validator using ValidatorMsg::Communicate, ValidatorMsg::Contract, ValidatorMsg::UpdateTx, ValidatorMsg::UpdateMode, ValidatorMsg::UpdateValidatorAboutMempoolTx, ValidatorMsg::UpdateValidatorAboutMiningProcess

    fn receive(&mut self,
                _ctx: &Context<Self::Msg>, //// ctx is the actor system which we can build child actors with it also sender is another actor 
                _msg: UpdateValidatorAboutMempoolTx, //-- _msg is of type UpdateValidatorAboutMempoolTx since we're implementing the Receive trait for the UpdateValidatorAboutMempoolTx type
                _sender: Sender){
    
        info!("-> {} - new transaction with id [{}] slided into the mempool", chrono::Local::now().naive_local(), _msg.0); //// UpdateValidatorAboutMempoolTx is a tuple like struct so we have to get the first elem of it using .0
        
        
        // other logics goes here
        // ...
                    
    }
}


impl Receive<UpdateValidatorAboutMiningProcess> for Validator{ //// implementing the Receive trait for the Validator actor to handle the incoming message of type UpdateValidatorAboutMiningProcess
    type Msg = ValidatorMsg; //// we can access all the message event actors which has defined for Validator using ValidatorMsg::Communicate, ValidatorMsg::Contract, ValidatorMsg::UpdateTx, ValidatorMsg::UpdateMode, ValidatorMsg::UpdateValidatorAboutMempoolTx, ValidatorMsg::UpdateValidatorAboutMiningProcess

    fn receive(&mut self,
                _ctx: &Context<Self::Msg>, //// ctx is the actor system which we can build child actors with it also sender is another actor 
                _msg: UpdateValidatorAboutMiningProcess, //-- _msg is of type UpdateValidatorAboutMiningProcess since we're implementing the Receive trait for the UpdateValidatorAboutMiningProcess type
                _sender: Sender){
    
        info!("-> {} - start mining process of block with id [{}]", chrono::Local::now().naive_local(), _msg.0); //// UpdateValidatorAboutMiningProcess is a tuple like struct so we have to get the first elem of it using .0
        
        
        // other logics goes here
        // ...
                    
    }
}