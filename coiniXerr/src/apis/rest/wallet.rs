





  
use crate::*; // loading all defined crates, structs and functions from the root crate which is lib.rs in our case















#[post("/transaction")] //-- the route for handling streaming of all kind of transactions in form of utf8 binary data 
async fn transaction(req: HttpRequest, mut body_payload: web::Payload, transaction_sender: web::Data<Sender<Arc<Mutex<Transaction>>>>) -> Result<HttpResponse, Error>{
    let transaction_sender = transaction_sender.as_ref();
    println!("-> {} - connection stablished from {}", chrono::Local::now().naive_local(), req.peer_addr().unwrap());
    let mut bytes = web::BytesMut::new();
    while let Some(chunk) = body_payload.next().await { //-- extracting binary wallet data or utf8 bytes from incoming request chunk by chunk - loading the payload into the memory
        bytes.extend_from_slice(&chunk?); //-- the web::Payload extractor already contains the decoded byte stream if the request payload is compressed with one of the supported compression codecs (br, gzip, deflate), then the byte stream is decompressed
    }
    println!("-> {} - transaction body in bytes {:?}!", chrono::Local::now().naive_local(), bytes);
    let deserialized_transaction_serde = &mut serde_json::from_slice::<Transaction>(&bytes).unwrap(); //-- deserializing bytes into the Transaction struct object using serde from_slice method
    // TODO - only if the downside of the mpsc job queue channel was available the transaction will be signed and sent through the mempool channel to be pushed inside a block for mining process
    // ...
    let must_be_signed = true;
    if must_be_signed{
        deserialized_transaction_serde.signed = Some(chrono::Local::now().naive_local().timestamp()); //-- signing the incoming transaction with server time
        // ----------------------------------------------------------------------------------------------------
        //          SERIALIZING SIGNED TRANSACTION INTO THE UTF8 BYTES USING from_raw_parts() METHOD
        // ----------------------------------------------------------------------------------------------------
        // NOTE - encoding or serializing process is converting struct object into utf8 bytes
        // NOTE - decoding or deserializing process is converting utf8 bytes into the struct object
        let signed_transaction_serialized_into_bytes: &[u8] = unsafe { //-- encoding process of new transaction by building the &[u8] using raw parts of the struct - serializing a new transaction struct into &[u8] bytes
            //-- converting a const raw pointer of an object and its length into the &[u8], the len argument is the number of elements, not the number of bytes
            //-- the total size of the generated &[u8] is the number of elements (each one has 1 byte size) * mem::size_of::<Transaction>() and it must be smaller than isize::MAX
            //-- here number of elements or the len for a struct is the size of the total struct which is mem::size_of::<Transaction>()
            slice::from_raw_parts(deserialized_transaction_serde as *const Transaction as *const u8, mem::size_of::<Transaction>())
        };
        // ----------------------------------------------------------------------------------------------------
        //         SERIALIZING SIGNED TRANSACTION INTO THE UTF8 BYTES USING serde_json::to_vec() METHOD
        // ----------------------------------------------------------------------------------------------------
        let signed_transaction_serialized_into_bytes_using_serde = serde_json::to_vec(deserialized_transaction_serde).unwrap(); //-- serializing the signed transaction into vector of utf8 bytes; Vec<u8> will be coerced to &[u8] at compile time
        // ---------------------------------------------------------------------------------------
        //        SENDING SIGNED TRANSACTION TO DOWN SIDE OF THE CHANNEL FOR MINING PROCESS
        // ---------------------------------------------------------------------------------------
        let signed_transaction_deserialized_from_bytes = serde_json::from_slice::<Transaction>(&signed_transaction_serialized_into_bytes_using_serde).unwrap(); //-- deserializing signed transaction bytes into the Transaction struct cause deserialized_transaction_serde is a mutable pointer (&mut) to the Transaction struct
        let arc_mutex_transaction = Arc::new(Mutex::new(signed_transaction_deserialized_from_bytes)); //-- putting the signed_transaction_deserialized_from_bytes inside a Mutex to borrow it as mutable inside Arc by locking the current thread 
        let cloned_arc_mutex_transaction = Arc::clone(&arc_mutex_transaction); //-- cloning the arc_mutex_transaction to send it through the mpsc job queue channel 
        transaction_sender.send(cloned_arc_mutex_transaction).await.unwrap(); //-- sending signed transaction through the mpsc job queue channel asynchronously for mining process
        // ----------------------------------------------------------------------
        //               SENDING SIGNED TRANSACTION BACK TO THE USER
        // ----------------------------------------------------------------------
        Ok(
            HttpResponse::Ok().json(
                ResponseBody::new(
                    constants::MESSAGE_FETCHED_SUCCESS,
                    deserialized_transaction_serde, //-- send the signed transaction back to the user
                )
            )
        )
    } else{
        Ok(
            HttpResponse::Ok().json(
                ResponseBody::new(
                    constants::MESSAGE_FETCHED_SUCCESS,
                    deserialized_transaction_serde, //-- send the unsigned transaction back to the user
                )
            )
        )
    }
}


pub fn routes(config: &mut web::ServiceConfig){
    config.service(transaction);
}