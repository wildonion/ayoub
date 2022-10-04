







use crate::*; // loading all defined crates, structs and functions from the root crate which is lib.rs in our case
use super::peer; //-- super is the root of the current directory which is actors directory contains parathread.rs and peer.rs crates










// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
//                  Messages and enums
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈

#[derive(Clone, Debug)] //-- bounding to Clone and the Debug trait
pub struct Communicate{ //-- parathread sends this message to a parachain
    pub id: Uuid,
    pub cmd: Cmd,
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, Default)]
pub enum Cmd{
    #[default] //// enum data types can only have one field as the default value
    GetCurrentBlock, //// Mine field is the default value; utf8 encoded variant is 0
    GetSlot, //// utf8 encoded variant is 1
    GetBlockchain, //// utf8 encoded variant is 2
    GetNextParachain, //// utf8 encoded variant is 3
    GetGenesis, //// utf8 encoded variant is 4
    GetParachainUuid, //// utf8 encoded variant is 5

}







// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
//                 Parachain type actor
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈

#[actor(Communicate)] //-- Parachain actor will receive a message of type Contract
#[derive(Debug, Clone, Default)] //-- trait Clone is required to prevent the object of this struct from moving
pub struct Parachain {
    pub id: Uuid,
    pub slot: Option<Slot>,
    pub blockchain: Option<Chain>,
    pub next_parachain: Option<ActorRef<<Parachain as Actor>::Msg>>, //-- next parachain actor which is of type Parachain
    pub current_block: Option<Block>,
}

impl Parachain{ //// Parachain is the parallel chain of the coiniXerr network which is a shard actor
    
    pub fn heart_beat(){

        // TODO - check the parachain health
        // ...
    
    }

    pub fn get_uuid(&self) -> Option<Uuid>{
        Some(self.id.clone())
    }

    pub fn get_current_block(&self) -> Option<Block>{
        self.current_block.clone()
    }

    pub fn get_genesis(&self) -> Option<Block>{ //// the lifetime of the &Block is the lifetime of the &self
        let genesis_block = self.blockchain.as_ref().unwrap().get_genesis();
        Some(genesis_block) //// returning the genesis_block as an Option 
    }

    pub fn get_next_parachain(&self) -> Option<ActorRef<<Parachain as Actor>::Msg>>{
        self.next_parachain.clone()
    }

    pub fn get_slot(&self) -> Option<Slot>{
        self.slot.clone()
    }

    pub fn get_blockchain(&self) -> Option<Chain>{
        self.blockchain.clone()
    }

}









// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
//    implementing the Actors for the Parachain type
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈

impl Actor for Parachain{

    //// When using the #[actor()] attribute, the actor's Msg associated type should be set to '[DataType]Msg'. 
    //// E.g. if an actor is a struct named MyActor, then the Actor::Msg associated type will be MyActorMsg.
    type Msg = ParachainMsg; //// Msg associated type is the actor mailbox type and is of type ParachainMsg which is the Parachain type itself; actors can communicate with each other by sending message to each other

    fn recv(&mut self, 
            ctx: &Context<Self::Msg>, //// ctx is the actor system which we can build child actors with it also sender is another actor 
            msg: Self::Msg, 
            sender: Sender){
        
        self.receive(ctx, msg, sender);

    }
}


impl ActorFactoryArgs<(Uuid, Option<Slot>, Option<Chain>, Option<ActorRef<<Parachain as Actor>::Msg>>, Option<Block>)> for Parachain{

    fn create_args((id, slot, blockchain, next_parachain, current_block): (Uuid, Option<Slot>, Option<Chain>, Option<ActorRef<<Parachain as Actor>::Msg>>, Option<Block>)) -> Self{
        
        Self { id, slot, blockchain, next_parachain, current_block } //// initiate an instance of the Parachain with the passed in args
    
    }

}








// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
//    implementing the Receive types for our actor
// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈

impl Receive<Communicate> for Parachain{ //// implementing the Receive trait for the Parachain actor to handle the incoming message of type Communicate
    type Msg = ParachainMsg;

    fn receive(&mut self,
                _ctx: &Context<Self::Msg>, //// ctx is the actor system which we can build child actors with it also sender is another actor 
                _msg: Communicate, //-- _msg is of type Communicate since we're implementing the Receive trait for the Communicate type
                _sender: Sender){
    
        info!("-> {} - message info received with id [{}] and ttype [{:?}]", chrono::Local::now().naive_local(), _msg.id, _msg.cmd);
        match _msg.cmd{
            Cmd::GetCurrentBlock => {
                info!("-> {} - getting current block", chrono::Local::now().naive_local());
                let current_block = self.get_current_block();
                _sender
                    .as_ref() //// convert to Option<&T> - we can also use clone() method instead of as_ref() method in order to borrow the content inside the Option to prevent the content from moving and loosing ownership
                    .unwrap()
                    .try_tell(
                        current_block, //// sending the current_block as the response message 
                        Some(_ctx.myself().into()) //// to the actor or the caller itself
                    );
            },
            Cmd::GetNextParachain => {
                info!("-> {} - getting the next parachain of the parachain with id [{}]", chrono::Local::now().naive_local(), self.id);
                let next_parachain = self.get_next_parachain();
                _sender
                    .as_ref() //// convert to Option<&T> - we can also use clone() method instead of as_ref() method in order to borrow the content inside the Option to prevent the content from moving and loosing ownership
                    .unwrap()
                    .try_tell(
                        next_parachain, //// sending the next_parachain as the response message 
                        Some(_ctx.myself().into()) //// to the actor or the caller itself
                    );
            },
            Cmd::GetBlockchain => {
                info!("-> {} - getting the blockchain of the parachain with id [{}]", chrono::Local::now().naive_local(), self.id);
                let blockchain = self.get_blockchain();
                _sender
                    .as_ref() //// convert to Option<&T> - we can also use clone() method instead of as_ref() method in order to borrow the content inside the Option to prevent the content from moving and loosing ownership
                    .unwrap()
                    .try_tell(
                        blockchain, //// sending the blockchain as the response message 
                        Some(_ctx.myself().into()) //// to the actor or the caller itself
                    );
            },
            Cmd::GetGenesis => {
                info!("-> {} - getting the genesis block of the parachain with id [{}]", chrono::Local::now().naive_local(), self.id);
                let genesis_block = self.get_genesis();
                _sender
                    .as_ref() //// convert to Option<&T> - we can also use clone() method instead of as_ref() method in order to borrow the content inside the Option to prevent the content from moving and loosing ownership
                    .unwrap()
                    .try_tell(
                        genesis_block, //// sending the genesis_block as the response message 
                        Some(_ctx.myself().into()) //// to the actor or the caller itself
                    );
            },
            Cmd::GetParachainUuid => {
                info!("-> {} - getting the parachain uuid", chrono::Local::now().naive_local());
                let genesis_block = self.get_uuid();
                _sender
                    .as_ref() //// convert to Option<&T> - we can also use clone() method instead of as_ref() method in order to borrow the content inside the Option to prevent the content from moving and loosing ownership
                    .unwrap()
                    .try_tell(
                        genesis_block, //// sending the genesis_block as the response message 
                        Some(_ctx.myself().into()) //// to the actor or the caller itself
                    );
            },
            _ => { //// GetSlot
                info!("-> {} - getting the slot of the parachain with id [{}]", chrono::Local::now().naive_local(), self.id);
                let current_slot = self.get_slot();
                _sender
                    .as_ref() //// convert to Option<&T> - we can also use clone() method instead of as_ref() method in order to borrow the content inside the Option to prevent the content from moving and loosing ownership
                    .unwrap()
                    .try_tell(
                        current_slot, //// sending the current_slot as the response message 
                        Some(_ctx.myself().into()) //// to the actor or the caller itself
                    );
            }
        }            


    }

}