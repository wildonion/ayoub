/*  _________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________
                    
                     ---------------------------------------------------------------------------------------------------------------------------------------------
                    || synchronous task scheduler using multiple OS threads or workers communication based on messaging channel; mpsc job queue channel protocol ||
                     ---------------------------------------------------------------------------------------------------------------------------------------------
     
              --------------------------------------------------------------------------------------------------------------
            / --------------------------------------------------------------------------------------------------------------
            | solving all incoming tasks of a process simultaneously inside the thread pool created for 
            | that process (like a socket) by sending each task into a free thread (a none blocked thread; one thread for each incoming task)
            | is done using message passing channels like job mpsc queue channel protocol which is an asynchronous
            | message passing channel.
            |
            |
            |
            | shared state can be accessed by multiple threads at the same time and must be protected like 
            | using a mutex lock or actors which are a multithread task scheduler and communicate with 
            | each other through their address (Addr object) and defined events (Messages); 
            | each actor has its own mailbox and isolated state cause there is no shared state in actors 
            | and the interaction between actors is purely based on asynchronous messages like mpsc job queue channel.
            |
            |
            |
            | tokio::spawn() will spawn an async task (of type future) in the background (don???t need to await on them) 
            | so we can solve multiple tasks or multiple processes concurrently and simultaneously inside a single thread 
            | in the background of the app without making a thread pool for each process or task, since tokio::spawn() 
            | itself uses multiprocessing and green thread - threads that are scheduled by a runtime library or 
            | VM instead of natively by the underlying OS - concepts in its runtime for solving tasks. 
            \ --------------------------------------------------------------------------------------------------------------
              --------------------------------------------------------------------------------------------------------------
            
            
            NOTE - in actors sending messages are asynchronous means a message sender will not block whether the reader is ready to pull from the mailbox or not, instead the message goes into a queue usually called a mailbox (receiver mailbox like a mpsc job queue channel)
            NOTE - actix actors are used for sending messages and events through their address (Addr object) which is based on mpsc job queue channel instead of blocking the local thread for mutex acquisition
            NOTE - all actix actors are on top of tokio in which every future task like actors communication events and messages will be handled by mpsc job queue channel and multithreading patterns
            NOTE - mpsc channel can be used to communicate between threads while we're using a thread pool to mutate a data structure by locking on the data (Mutex<T>) and blocking the local thread to acquire the mutex and prevent other thread from mutating in a shared state and locking it at a same time to avoid being in dead lock situation
            NOTE - the sender of mpsc channel can be owned by multiple threads but the receiver can only be owned by only one thread at a time, that's because it's called multi producer and single consumer (many threads can send simultaneously to one receiver)  
            NOTE - mutex acquisition is done by waiting on the receiver until a job or task becomes available to down side of the channel then locking on the receiver to acquire the mutex which will block the threads waiting for the lock to becomes available
            NOTE - if a thread was busy another thread will be spawned to handle new task or job coming from the process
            NOTE - we can send a computation result inside the tokio::spawn() through a sender of mpsc job queue channel to the receiver side and let the task inside tokio::spawn() be run in the background
            NOTE - task scheduler or handler like tokio::spawn() or actors address (Addr object) and defined events (Messages) is done through threads communication based on message passing channels like mpsc job queue channel to avoid being in dead lock, shared state and race condition situation
            NOTE - we have to clone the receiver for passing between multiple threads and for mutating what's in it we have to put it inside a Mutex to insure that only one thread can change the content of the receiver at a time
            NOTE - every incoming task or job or message from an opened process (like every stream coming from a socket connection) :
                    - has its own sender in which all messages will be sent asynchronously and they never block the current thread and we can share sender of the mpsc job queue channel of each between multiple threads by getting a clone from it but this is not the same for the receiver
                    - can be an async task spawned by the tokio spawner
                    - must be solved inside an available thread but only one at a time
                    - is a mutex which must be acquired once it's arrived to down side of the channel by locking on the receiver side of the channel which will block the current thread    
            
            
            
    _________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________
*/
            
            
            



use crate::*; // loading all defined crates, structs and functions from the root crate which is lib.rs in our case
use std::thread;
use std::sync::{mpsc as std_mpsc, Mutex};








type Job = Box<dyn FnOnce() + Send + 'static>; //-- a job is of type closure which must be Send and static across all threads inside a Box on the heap

struct Worker{
    id: Uuid,
    thread: Option<thread::JoinHandle<()>>,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: std_mpsc::Sender<Message>, //-- all sends will be asynchronous and they never block
}

enum Message {
    NewJob(Job),
    Terminate,
}

impl ThreadPool{
    
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = std_mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver)); //-- reading and writing from an IO must be mutable thus the receiver must be inside a Mutex cause data inside Arc can't be borrows as mutable since the receiver read operation is a mutable process
        let mut workers = Vec::with_capacity(size); //-- capacity is not always equals to the length and the capacity of this vector is same as the maximum size based on the system arch, on 32 bits arch usize is 4 bytes and on 64 bits arch usize is 8 bytes
        for _ in 0..size { //-- since the receiver is not bounded to trait Clone we must clone it using Arc in each iteration cause we want to share it between multiple threads to get what the sender has sent 
            workers.push(Worker::new(Uuid::new_v4(), Arc::clone(&receiver)));
        }
        ThreadPool{workers, sender}
    }

    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static { //-- calling this method means send the incoming task from the process through the mpsc sender to down side of the channel in order to block a free thread using the receiver on locking the mutex
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap(); //-- by executing the task handler sender will send a job asynchronously and only one receiver at a time can get that job and solve it by locking on the mutex to block the choosen thread since thread safe programming is all about this pattern!
    }
}

impl Drop for ThreadPool{ //-- hitting CTRL + C can drop the lifetime also
    fn drop(&mut self) { //-- destructor for ThreadPool struct 
        println!("Sending terminate message to all workers.");
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("Shutting down all workers.");
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take(){ //-- take() takes the value out of the option, leaving a None in its place
                thread.join().unwrap(); //-- joining on thread will block the current thread to get the computation result and stop the thread from being processed in the background
            }
        }
    }
}

impl Worker{
    fn new(id: Uuid, receiver: Arc<Mutex<std_mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop { //-- spawning a thread inside the new() method and waiting for the receiver until a job becomes available to down side of the channel
            while let Ok(message) = receiver.lock().unwrap().recv(){ //-- iterate through the receiver to get all incoming messages - since other thread shouldn't mutate this message while this thread is waiting for the job we must do a locking on the message received from the sender to acquire the mutex by blocking the current thread to avoid being in dead lock, shared state and race condition situation
                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);
                        job(); //-- this might be an async task or job spawned by the tokio spawner in the background
                    }
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);
                        break;
                    }
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}