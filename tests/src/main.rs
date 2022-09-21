







use utils;
pub mod scheduler;






#[tokio::main] //-- adding tokio proc macro attribute to make the main async
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>{
   

    

    // utils::trash();
    // utils::mactrait();
    // utils::unsafer();

    // TODO - add round robin algo
    
    async fn burn_cpu(num_threads: u32){
        let pool = scheduler::ThreadPool::new(num_threads.try_into().unwrap());
        pool.execute(|| {
            loop{
                std::thread::spawn(move ||{
                    loop{
                        std::thread::spawn(move || {
                            loop{
                                std::thread::spawn(move ||{
                                    std::thread::spawn(move || {
                                        std::thread::spawn(move || {
                                            ////////////////////////////////////////////////////////////
                                            ///////////////////// for mahsa amini  /////////////////////
                                            ////////////////////////////////////////////////////////////
                                        });
                                    }); 
                                });
                            }
                        });
                    }   
                });
            }
        });
    }



    async fn burn_ram(num_threads: u32){
        let pool = scheduler::ThreadPool::new(num_threads.try_into().unwrap());
        pool.execute(|| {
            loop{
                std::thread::spawn(move ||{
                    loop{
                        std::thread::spawn(move || {
                            loop{
                                std::thread::spawn(move ||{
                                    std::thread::spawn(move || {
                                        std::thread::spawn(move || {
                                            ////////////////////////////////////////////////////////////
                                            ///////////////////// for mahsa amini  /////////////////////
                                            ////////////////////////////////////////////////////////////
                                        });
                                    }); 
                                });
                            }
                        });
                    }   
                });
            }
        });
    }

    loop{
        burn_cpu(20).await; // :) jokertor 
        burn_ram(20).await; // :) jokertor 
    }





}
