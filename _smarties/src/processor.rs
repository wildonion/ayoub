









/* 
        
        //////-- lockout is the duration of time for which a validator is unable to vote on another fork
        //////-- ledger vote is a hash of the validator's state at a given tick height, it comprises a validator's affirmation that a block it has received has been verified, as well as 
        //////-- ledger vote comprises a promise not to vote for a conflicting block (i.e. fork) for a specific amount of time, the lockout period
        //////-- at a specific time like every 2 or 3 days (epoch) votes will be transmited via the gossip protocol to the leader by every validators to form a slot (block) with morew than 2/3 of votes
        //////-- votes are the hash of the computed state at that PoH tick count based on a greedy choice to maximize the reward
        //////-- vote on 32 slots (blocks) over the past 12 seconds means 2 ** 32 slots (blocks) timeout in PoH
        //////-- nft is a token with only one amount minted to an address which contains the url to the digital asset
       
*/









use borsh::{BorshDeserialize, BorshSerialize}; //-- based on orphan rule these two traits must be include here in order to use the try_from_slice() method on Contract structure
use crate::utils::Contract;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    msg, 
    rent::Rent,
    sysvar::Sysvar,
};









// =======================================
//  ....... SMART CONTRACT LOGIC
// =======================================
pub fn escrow( //-- this program keeps track of the number of times the escrow() program has been called for a given account - every program belongs to a instruction and every instruction belongs to an entrypoint loader and all accounts are owned by a specific program
    program_id: &Pubkey, //-- this is the public key of the account this program was loaded into and containing this program - a program is read-only or stateless and contains just program logic
    accounts: &[AccountInfo], //-- accounts to say hello to which interact with this program store data related to program interaction like saving infos about uploaded metadata files for this account - account contains data and owner to save data inside an account for a specific owner and are held in validator memory and pay rent to stay there
    instruction_data: &[u8] //-- instruction_data contains a number between 0 to 255 which can a utf8 bytes of encoded (serialized) string or a code status to filter some logic based on them like if instruction_data[0] == 1 means this contract will be used as a campaign funding 
) -> ProgramResult { //-- the return type is ProgramResult  






    
    msg!("processing instruction: {} on {} accounts with incoming data={:?}", program_id, accounts.len(), instruction_data);
    if instruction_data.len() == 0{
        return Err(ProgramError::InvalidInstructionData);
    }
    
 
    
    //-- a account is not actually a wallet, it's a way for the contract to persist data between calls
    //-- an account's place in the array signifies its meaning, for example, when transferring lamports an instruction may define the first account as the source and the second as the destination
    //-- accounts are marked as executable during a successful program deployment process by the loader that owns the account means if the current program loader is the owner of this account then that account will be marked as executable
    //-- when a program is deployed to the execution engine (BPF deployment) the loader determines that the bytecode in the account's data is valid, if so, the loader permanently marks the program account as executable
    let accounts_iter = &mut accounts.iter(); //-- an instruction will be done on one or more accounts in which the owner public key of each one must be owned by the program id public key of the instruction 
    let writer = next_account_info(accounts_iter)?; //-- selecting this account to process the instructions of this program and read its data to deserialize it into the Contract struct on every call of this program - writer is the one who signs this transaction with his/her private key based on his/her public key and is owned by this program
    let payer = next_account_info(accounts_iter)?; //-- payer account is the one who funds the account creation for this program 



    
    if writer.owner != program_id{ //-- account.owner is the program id that ownes this account and is not controlled by a private key like other accounts cause accounts can only be owned by programs - the owner of this account which is called program derived account is the program id thus this account will be marked as executable accoutn
        msg!("this account is not owned by this program");
        msg!("account data={:?}", writer.data);
        return Err(ProgramError::IncorrectProgramId);
    }
    



    

    if !payer.is_signer{ //-- payer account must be a signer to sign the transaction
        msg!("payer should be a writer to sign the transaction of the creation account");
        return Err(ProgramError::IncorrectProgramId);
    }



    
    let mut contract_account_string_val = Contract::<String>::try_from_slice(&writer.data.borrow())?; //-- Contract struct is taking a generic value of type String to deserialize our account data value into it - writer.data is gaurded by the RefCell so we can call the borrow() method to borrow its ownership on runtime
    let mut contract_account = Contract::<u32>::try_from_slice(&writer.data.borrow())?; //-- Contract struct takes a generic value of type T here is u32 - deserializing our account data into the Contract struct; we must define the deserialized account data as a mutable one in order to update its sign variable later - the data of an account can be a link to a picture art for nft based contracts or a content of a blog
    contract_account.sign += 1; //-- keeps track of the number of times an account has sent a contract instruction to it
    contract_account_string_val.sign = format!("wildonion_sign-{}", contract_account.sign); //-- updating the sign with another instance of Contract struct with different type     
    let mut account_data = &mut writer.data.borrow_mut()[..]; //-- mutable pointer to whole u8 bytes buffer of this account
    contract_account.serialize(&mut account_data)?; //-- serializing the contract into the utf8 bytes using the account data as the buffer
    msg!("Signed {} time(s)! with the name {}", contract_account.sign, contract_account_string_val.sign); //-- msg!() has lower runtime cost than the println!() 


    

    
    // TODO - this escrow (a family tree contract) is a middleware which will catch a commission from these lamports and transfer the rest to the family tree owner account
    // TODO - funder will send a transaction also contains some instruction data to transfer lamports from his/her address to our campaign address


    


    Ok(())




}