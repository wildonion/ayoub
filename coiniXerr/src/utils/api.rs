


use crate::*;





/////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
///////                fetching user data from the ayoub auth server 
/////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈

#[macro_export]
macro_rules! user_data {
    ($user_id:expr, $token:expr) => { //-- we have to use match on where this macro is called
        {

            use mongodb::bson::oid::ObjectId;
            use serde::{Deserialize, Serialize};
            use log::info;


            #[derive(Debug, Serialize, Deserialize)]
            pub struct UserData{
                pub _id: Option<ObjectId>, //-- this is the user id inside the users collection
                pub username: String,
                pub phone: String,
                pub access_level: u8, // NOTE - 0 means dev, 1 means admin, 2 means user
                pub status: u8, //-- last status in an event game that this user has participated in
                pub role_id: Option<ObjectId>, //-- last role in an event game that this user has participated in
                pub side_id: Option<ObjectId>, //-- last side in an event game that this user has participated in
                pub created_at: Option<i64>,
                pub updated_at: Option<i64>,
                pub last_login_time: Option<i64>,
                pub wallet_address: String,
                pub balance: Option<u128>,
            }

            


            let coiniXerr_http_port = env::var("AYOUB_PORT").expect("⚠️ please set ayoub port in .env");
            let host = env::var("HOST").expect("⚠️ please set host in .env");
            let url = format!("http://{}:{}/auth/check-token", host, coiniXerr_http_port, $user_id);
            match reqwest::Client::builder().build(){
                Ok(client) => {
                    match client
                        .get(&url)
                        .bearer_auth($token) // NOTE - it'll attach the Bearer token in request header
                        .send()
                        .await{
                            Ok(res) => {
                                match res.json::<UserData>().await{ //-- deserializing response utf8 bytes into the UserData struct
                                    Ok(resp) => {
                                        info!("[+] CURRENT SERVER TIME : {:?} | USER DATA FROM THE AYOUB SERVER : {:?}", chrono::Local::now().naive_local(), resp);
                                        Ok(resp)
                                    },
                                    Err(e) => {
                                        info!("[!] CURRENT SERVER TIME : {:?} | PARSING RESPONSE ERROR : {:?}", chrono::Local::now().naive_local(), e);
                                        Err(e)
                                    }
                                }
                            },
                            Err(e) => {
                                info!("[!] CURRENT SERVER TIME : {:?} | AYOUB SERVER STATUS : {:?}", chrono::Local::now().naive_local(), e);
                                Err(e)
                            }
                        }
                },
                Err(e) => {
                    info!("\t[!] CURRENT SERVER TIME : {:?} | FAILED TO BUILD THE HTTP CLIENT OBJECT : {:?}", chrono::Local::now().naive_local(), e);
                    Err(e)
                }
            }
        }
    };
}




/////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 
///////             sending fake transaction to the coiniXerr tcp server  
/////// ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ --------- ⚈ 

pub async fn tx_emulator(){
    
    let mut time: u64 = 0;
    let tcp_host = env::var("HOST").expect("⚠️ please set host in .env");
    let tcp_port = env::var("COINIXERR_TCP_PORT").expect("⚠️ please set coiniXerr tcp port in .env");
    let ip_port = format!("{}:{}", tcp_host, tcp_port);
    let pool = utils::scheduler::ThreadPool::new(10);
    let sleep = Duration::from_secs("3".to_string().parse::<u64>().unwrap());

    loop{ //-- simulating a transaction emulator by sending infinite tx to the coiniXerr tcp server
        
        time+=1;
        let ip_port = ip_port.clone();
        pool.execute(move || { //-- a closure with a async block in its return type
            tokio::spawn(async move{ //-- an async block or future object is the param of the tokio::spawn()
                match TcpStream::connect(ip_port.as_str()).await{
                    Ok(mut stream) => { //-- stream must be muatble in order to write on it

                        let random_tx = Transaction::default(); //-- creating a default transaction
                        let encoded_tx = random_tx.try_to_vec().unwrap(); //-- encoding using borsh; we can convert a Vec<u8> to &[u8] by taking a reference to it since &[u8] which will be on the stack is an slice of the Vec<u8> which is inside the heap 
                        stream.write_all(&encoded_tx).await.unwrap(); //-- writing the buffer into the stream to send back to the server

                    },
                    Err(e) => {
                        error!("😕 : {}", e);
                    }
                }
            });
        });  

        thread::sleep(sleep);
    }
    
}