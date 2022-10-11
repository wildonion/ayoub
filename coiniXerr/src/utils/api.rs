


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

// use tokio::spawn(async move{}); and pool.execute(|| async move{}); to send encoded tx to the tcp server


/*

    let mut time:u64 = 0;
    let sleep = Duration::from_millis(env::args().nth(2).unwrap_or("0".to_string()).parse::<u64>().unwrap());
    loop {
        time+=1;
        thread::spawn(move|| {
            match TcpStream::connect(env::args().nth(1).unwrap()) {
                Ok(mut tcp) => {
                    tcp.write(&[0]).unwrap();
                    print!("\r{}", time);
                    io::stdout().flush().ok().expect("Could not flush stdout");
                },
                _ => {}
            };
        });
        thread::sleep(sleep);
    }
    
*/