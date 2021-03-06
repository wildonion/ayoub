




/*



Coded by



 █     █░ ██▓ ██▓    ▓█████▄  ▒█████   ███▄    █  ██▓ ▒█████   ███▄    █ 
▓█░ █ ░█░▓██▒▓██▒    ▒██▀ ██▌▒██▒  ██▒ ██ ▀█   █ ▓██▒▒██▒  ██▒ ██ ▀█   █ 
▒█░ █ ░█ ▒██▒▒██░    ░██   █▌▒██░  ██▒▓██  ▀█ ██▒▒██▒▒██░  ██▒▓██  ▀█ ██▒
░█░ █ ░█ ░██░▒██░    ░▓█▄   ▌▒██   ██░▓██▒  ▐▌██▒░██░▒██   ██░▓██▒  ▐▌██▒
░░██▒██▓ ░██░░██████▒░▒████▓ ░ ████▓▒░▒██░   ▓██░░██░░ ████▓▒░▒██░   ▓██░
░ ▓░▒ ▒  ░▓  ░ ▒░▓  ░ ▒▒▓  ▒ ░ ▒░▒░▒░ ░ ▒░   ▒ ▒ ░▓  ░ ▒░▒░▒░ ░ ▒░   ▒ ▒ 
  ▒ ░ ░   ▒ ░░ ░ ▒  ░ ░ ▒  ▒   ░ ▒ ▒░ ░ ░░   ░ ▒░ ▒ ░  ░ ▒ ▒░ ░ ░░   ░ ▒░
  ░   ░   ▒ ░  ░ ░    ░ ░  ░ ░ ░ ░ ▒     ░   ░ ░  ▒ ░░ ░ ░ ▒     ░   ░ ░ 
    ░     ░      ░  ░   ░        ░ ░           ░  ░      ░ ░           ░ 
                      ░                                                  



    Server Design Pattern Idea: https://github.com/hyperium/hyper/tree/master/examples
    Return Pointer from Functions Explanation: https://stackoverflow.com/a/57894943/12132470
    Return Pointer to a Structure Explanation: https://stackoverflow.com/questions/37789925/how-to-return-a-newly-created-struct-as-a-reference
    Rust Rules Gathered by wildonion: https://github.com/wildonion/extrust/blob/master/_docs/rust.rules




    [?] if the type was dropped from the memory then any reference to it is invalid due to the fact that the pointer might be a dangling pointer
    [?] due to avoiding use of dangling pointer in rust in order to return pointer in function body we could:
        0) if we do not allocate memory on the stack inside the function body we can return references to the result which already lives in memory
        1) return by value like returning T instead of &T
        2) return by reference with a defined lifetime for the return type to extend its lifetime outside of the function cause we can not return a reference pointing to a local variable inside the function
        3) return by reference using the same lifetime of one of the passed in arguments which is a (mutable) reference (&T or &mut T and T can be any type) to the argument to copy the value that we want to create into the caller's memory space
        4) return a Box which contains the T inside of it which has the address to the location of the T inside the heap



    [?] in order to return a value (indeed, any value) from a function, it has to be either copied to a memory location that the function's caller has access to, 
        or placed in a special memory location called "the heap," which is one particular place that each of your functions has access to and isn't destroyed 
        the same way that a function stack is



    [?] the reason you can't do this is because bindings (variables) that you define in a function only live for as long as the function is executing,
        this isn't specifically a Rust semantic but really a more general semantic of how (most or all?) modern computers work. When a function is called, 
        a block of memory is created for that function to store temporary data in, called "a function stack." When that function finishes, this function stack is, 
        for all intents and purposes, "destroyed" by the operating system, invalidating whatever was contained within it so that 
        the memory can be reused by other functions in the future.




    NOTE - when we use & in struct methods we can use the lifetime of the self to return pointer to a type from the method body 
    NOTE - wehn we use & in struct methods means we can call other methods inside the scope where the instance is, right after calling a method on the instance
    NOTE - it's ok to setup app storage and api object inside each controller instead of setting them up in main.rs to have single instance of them in the whole lifetime of the app since rust doesn't have garbage collector thus based on borrowing and ownership rules each app storage and api object inside each controller function lifetime will be valid till the end of function scope or body   
    NOTE - 'static trait bound means the type does not contain any non-static references, the receiver (function or the struct field) can hold on to the type for as long as they want and it will never become invalid until they drop it also any owned data always passes a 'static lifetime bound, but a reference to that owned data generally does not
    NOTE - based on orphan rule future traits must be imported to call their methods on hyper instances of the request and response body struct
    NOTE - it's ok to bound the generic type inside the function to traits and a valid lifetime (trait bound lifetime like 'static) without the Box but in order to define a generic type from traits or closures (closures are marked as traits) they must be inside the Box with dyn keyword behind them plus a valid lifetime ('static or 'other) cause object safe traits are no bounded to Sized traits and each closure generates a unique anonymous type for the closure's value  
    NOTE - None takes up exactly as much memory as if it were Some<T>. This is because Rust needs to know the Size of the Data you want to store and how much space it needs to allocate and for enums, which an option is, that means the space they need is as much as the largest variant And although you know that None will not change in this case, you could also swap it out with Some<T> any time later and then that new value needs to fit into that space
    NOTE - bodies in hyper are always streamed asynchronously and we have to collect them all together inside a buffer to deserialize from utf8 bytes to a pre defined struct
    NOTE - Box is a none dangling pointer with a usize size and will allocate T size (Box<T>) on the heap to store what's inside of it and allocate nothing on the heap if T is unsized  
    NOTE - to solve the `future is not `Send` as this value is used across an await` error we have to implement the Send trait for that type, since we don't know the type at compile time (it'll specify at runtime due to the logic of the code) we must put the trait inside the Box with the dyn keyword (object safe traits have unknown size at compile time) inside the return type of the function in second part of the Result 
    NOTE - Error, Send and Sync are object safe traits which must be bounded to a type, since we don't know the type in compile time (will be specified at runtime) we must put these trait inside a Box with the dyn keword behind them cause we don't know how much size they will take inside the memory
    NOTE - we can't return a trait inside the function cause the compiler needs to know how much space every function's return type requires and this can't be specified at compile time cause different implementations of a trait need different amount of memory
    NOTE - to return a trait inside the function we have to put it inside a Box with a dyn keyword (due to the dynamic size of the trait) cause a Box is just a reference to some memory in the heap and a reference has a statically known size thus compiler can guarantee it points to a heap allocated the trait that is not a dangling pointer
    NOTE - unwrapping a wrapped Option or Result type using ? will only work inside a method that will return Result or Option
    NOTE - always use &self (a shared immutable pointer or reference) or &mut self (a shared mutable pointer or reference) inside the struct methods' parameters to borrow the ownership of struct fields and the instance itself instead of moving (the borrowed form of struct fields) so we can call other methods' instance and have a valid lifetime for object; also the instance must be defined as mutable in order to use &mut self in methods' parameters
    NOTE - since mutable pointer to a type can change the value of the type thus if we want to mutate the struct field in its methods without losing its ownership we have to use &mut self as the first param of methods 
    NOTE - &self or &mut self will be converted automatically to self on compile time ++++ rust will call the drop() function at runtime for each type at the end of each scope
    NOTE - the trait Clone must be implemented for that struct in order to use & cause Clone is a super trait of Copy otherwise we can't borrow the ownership and take a reference to its field (see Api struct comments!)
    NOTE - a pointer takes usize size (64 bits target takes 64 bits or 8 bytes; 32 bits targets takes 32 bits or 4 bytes) to reference any location in memory 
    NOTE - the size of a boxed value or size_of_val(&Box::new(10)) is equals to the size of the Box which is just a pointer and a pointer takes usize (8 bytes or 4 bytes) to reference any location inside the memory
    NOTE - size of the value inside any smart pointer is equals to the size of the smart pointer itself which is usize  
    NOTE - usize is how many bytes it takes to reference any location in memory, on a 32 bit target, this is 4 bytes and on a 64 bit target, this is 8 bytes
    NOTE - generic type is needed for function returns and types and for those types that haven't fixed size in compile time we have to put them inside the Box or take a reference to them to borrow them using & and the size of the Box is usize and the size of the Box inside heap is the size of the T (on the heap) inside the Box and the Box will have a default valid lifetime for any type inside of it
    NOTE - if the size of the u8 is not specified we have to either use & with lifetime or put it inside a Box in which the lifetime will be handled automatically by the Box itself
    NOTE - since unsized types like traits, closures, str and [u8]s won't have fixed size at compile time they must be either used as a borrowed type using & with a valid lifetime or stored inside the Box which will be stored on the heap and a reference to that location will be returned from the Box thus in order to get the value inside the Box which is owned by the Box itself we have to dereference the Box using *
    NOTE - heap allocated types like String, Vec, traits and closures has 3 machine (usize) words wide which are pointer, length and capacity (for efficient resizing) inside the stack also they can be in their borrowed mode like &String, &Vec, &dyn Trait and &move || {}.
    NOTE - unsized borrowing for abstract types like object safe traits will be done using &dyn Trait/Closure + 'a or Box<dyn Trait/Closure + 'a> with a valid lifetime added at the end and for concrete type is done by using &Type or Box<Type>
    NOTE - we have to put the unknown size at compile time types like object safe traits and closures (which are of type traits) inside the Box due to the face that they don't bound to Sized traits and don't have fixed size at compile time
    NOTE - can't return &[u8] or [u8] in function signature due to unknown size of slice and lifetime constraints we could return either Vec<u8> or Box<[u8]> since Vec<u8> will be coerced to &'a [u8] with a valid lifetime (like 'a) at compile time
    NOTE - string (list) in rust can be either String (Vec) which will be stored on heap or str ([u8]) since beacuse of unknown size of the str ([u8]) we should take a pointer using & to the location of it which is either inside the binary, heap or the stack to pass them by reference between functions or store them inside a variable and they primarily uses are to create slices from String and Vec.
    NOTE - since str and [u8] must be in their borrowed form in the whole app runtime (their size would be 2 machine (usize) words wide; one for the pointer and the other for the length which both of them will be inside the stack) thus in order to return them inside a function we must put them inside the Box like &String, &Vec, &dyn Trait and &move || {} which must be inside the Box to return them in their borrowed form cause we can return them easily in their unborrowed form!
    NOTE - & is used for borrowing and taking a referencing to the location inside the memory of an unknown sized type like [u8] slices
    NOTE - since every type has its own lifetime which which will be destryoed whe it goes to out of its scope it'll prevent us to have a grabage collector system 
    NOTE - we have to pass by reference using & in function param to borrow the ownership of the type like passing Vec and String by & to borrow a slice of them and coerce them to &[u8] and &str
    NOTE - the size of a String allocated in memeory is 24 bytes; 64 bits or 8 bytes or usize (usize which is big enough to hold any pointer or offset) for each of pointer, len and capacity on 64 bits system
    NOTE - the size of the &str allocated in memeory (heap or binary or stack) is the total length of that str itself cause it's just the size of the str itself on either stack, heap or binary which is equals to its length of utf8 bytes for example the size of a none emoji word like "wildonion" is 9 bytes with 1 byte for each but the size of "wildn🥲oion" is 13 bytes which is 4 bytes more than the "wildonion" which is because of 🥲 emoji 
    NOTE - the size of the &str allocated in memeory (heap or binary or stack) is less than String and equals to the size of that str in bytes: size_of_val("wildonion") == size_of_val("wildonion".as_bytes()) 
    NOTE - shared reference for a type means that we have multiple owner across the whole app runtime and other threads and we can count them by putting the type inside a Rc<T> smart pointer also we can't dereference them
    NOTE - Vec and String are stored on the heap and their pointer, length and capacity (for resizing) will be stored on the stack; str and [u8] are stored on either heap, binary or the stack at runtime and since they are a slice of String or Vec they must be in their borrowed form using &.
    NOTE - we can't return pointer from the function body due to the borrowing and ownership (instead of garbage collection rule) rule which says that the lifetime of a type will be dropped once it goes out of its scope and due to this fact we can't return pointer from the function body cause it will be a dangled pointer after function scope which is pointing to an empty location with invalid lifetime; therefore we can use Box to put the borrowed type inside of it in order to return it from the function body.
    NOTE - to use unknown sized types like str, [u8], traits and closures at runtime they must have size at compile time and in order to fix this we have to either take a reference to them using & or put them inside the Box if we want to return them from function body.
    NOTE - shared reference can't dereference between threads and can't move out of it cause by moving or dereferencing it it'll lose its ownership and lifetime while some methods and threads are using it; we can sovle this using as_ref() method wich converts a &wrapped type into &T or by cloning the type




*/





// #![allow(unused)] //-- will let the unused vars be there - we have to put this on top of everything to affect the whole crate
#![macro_use] //-- apply the macro_use attribute to the root cause it's an inner attribute and will be effect on all things inside this crate 



use constants::MainResult;
use std::{net::SocketAddr, sync::Arc, env};
use chrono::Local;
use dotenv::dotenv;
use uuid::Uuid;
use log::{info, error};
use tokio::sync::oneshot;
use hyper::server::{Server, conn::AddrIncoming};
use self::contexts as ctx; // use crate::contexts as ctx;
use ctx::rafael::env::Serverless; // NOTE - based on orphan rule Serverless trait is required to use the run() method on the runtime instance





mod middlewares;
mod utils;
mod constants;
mod contexts;
mod schemas;
mod controllers;
mod routers;
mod services;
















#[tokio::main] //-- adding tokio proc macro attribute to make the main async
async fn main() -> MainResult<(), Box<dyn std::error::Error + Send + Sync + 'static>>{ //-- generic types can also be bounded to lifetimes ('static in this case) and traits inside the Box<dyn ... > - since the error that may be thrown has a dynamic size at runtime we've put all these traits inside the Box (a heap allocation pointer) and bound the error to Sync, Send and the static lifetime to be valid across the main function and sendable and implementable between threads
    
    



    



    


    // -------------------------------- environment variables setup
    //
    // ---------------------------------------------------------------------
    env::set_var("RUST_LOG", "trace");
    pretty_env_logger::init();
    dotenv().expect("⚠️ .env file not found");
    let io_buffer_size = env::var("IO_BUFFER_SIZE").expect("⚠️ no io buffer size variable set").parse::<u32>().unwrap() as usize; //-- usize is the minimum size in os which is 32 bits
    let environment = env::var("ENVIRONMENT").expect("⚠️ no environment variable set");
    let current_service = env::var("CURRENT_SERVICE").expect("⚠️ no current service variable set");
    let db_host = env::var("MONGODB_HOST").expect("⚠️ no db host variable set");
    let db_port = env::var("MONGODB_PORT").expect("⚠️ no db port variable set");
    let db_username = env::var("MONGODB_USERNAME").expect("⚠️ no db username variable set");
    let db_password = env::var("MONGODB_PASSWORD").expect("⚠️ no db password variable set");
    let db_engine = env::var("DB_ENGINE").expect("⚠️ no db engine variable set");
    let host = env::var("HOST").expect("⚠️ no host variable set");
    let auth_port = env::var("AYOUB_AUTH_PORT").expect("⚠️ no port variable set for auth service");
    let event_port = env::var("AYOUB_EVENT_PORT").expect("⚠️ no port variable set for event service");
    let game_port = env::var("AYOUB_GAME_PORT").expect("⚠️ no port variable set for game service");
    let nft_port = env::var("AYOUB_NFT_PORT").expect("⚠️ no port variable set for nft service");
    let auth_server_addr = format!("{}:{}", host, auth_port).as_str().parse::<SocketAddr>().unwrap(); //-- converting the host and port String into the as_str() then parse it based on SocketAddr generic type
    let event_server_addr = format!("{}:{}", host, event_port).as_str().parse::<SocketAddr>().unwrap(); //-- converting the host and port String into the as_str() then parse it based on SocketAddr generic type
    let game_server_addr = format!("{}:{}", host, game_port).as_str().parse::<SocketAddr>().unwrap(); //-- converting the host and port String into the as_str() then parse it based on SocketAddr generic type
    let nft_server_addr = format!("{}:{}", host, nft_port).as_str().parse::<SocketAddr>().unwrap(); //-- converting the host and port String into the as_str() then parse it based on SocketAddr generic type
    // let db_addr = format!("{}://{}:{}@{}:{}", db_engine, db_username, db_password, db_host, db_port); //------ UNCOMMENT THIS FOR PRODUCTION
    let db_addr = format!("{}://{}:{}", db_engine, db_host, db_port);
    let (sender, receiver) = oneshot::channel::<u8>(); //-- oneshot channel for handling server signals - we can't clone the receiver of the oneshot channel














    // -------------------------------- cli args setup
    //
    // ---------------------------------------------------------------------
    let args: Vec<String> = env::args().collect();
    let mut service_name = &args[1]; //-- since args[1] is of type String we must clone it or borrow its ownership using & to prevent args from moving, by assigning the first elem of args to service_name we'll lose the ownership of args (cause its ownership will be belonged to service_name) and args lifetime will be dropped from the ram 
    let service_port = &args[2];
    let mut server_addr: Option<SocketAddr> = if service_name != &"".to_string() && service_port != &"".to_string(){ //-- if none of the argument was empty we set the server_addr to the one that we've got from the cli input otherwise we set it to None to fill it later using the current_service as the service_name 
        Some(format!("{}:{}", host, service_port).as_str().parse::<SocketAddr>().unwrap()) //-- converting the host and port String into the as_str() then parse it based on SocketAddr generic type
    } else{
        service_name = &current_service; //-- setting the serivce_name to the current_service read from the .env file cause it's empty read from the cli input
        None
    };

    









    

    
    
    // -------------------------------- app storage setup
    //
    // ---------------------------------------------------------------------
    let empty_app_storage = Some( //-- putting the Arc-ed db inside the Option
        Arc::new( //-- cloning app_storage to move it between threads
            ctx::app::Storage{ //-- defining db context 
                id: Uuid::new_v4(),
                db: Some(
                    ctx::app::Db{
                        mode: ctx::app::Mode::Off,
                        instance: None,
                        engine: None,
                        url: None,
                    }
                ),
            }
        )
    );
    let db = if db_engine.as_str() == "mongodb"{
        info!("switching to mongodb - {}", chrono::Local::now().naive_local());
        match ctx::app::Db::new().await{
            Ok(mut init_db) => {
                init_db.engine = Some(db_engine);
                init_db.url = Some(db_addr);
                let mongodb_instance = init_db.GetMongoDbInstance().await; //-- the first argument of this method must be &self in order to have the init_db instance after calling this method, cause self as the first argument will move the instance after calling the related method and we don't have access to any field like init_db.url any more due to moved value error - we must always use & (like &self and &mut self) to borrotw the ownership instead of moving
                Some( //-- putting the Arc-ed db inside the Option
                    Arc::new( //-- cloning app_storage to move it between threads
                        ctx::app::Storage{ //-- defining db context 
                            id: Uuid::new_v4(),
                            db: Some(
                                ctx::app::Db{
                                    mode: init_db.mode,
                                    instance: Some(mongodb_instance),
                                    engine: init_db.engine,
                                    url: init_db.url,
                                }
                            ),
                        }
                    )
                )
            },
            Err(e) => {
                error!("init db error {} - {}", e, chrono::Local::now().naive_local());
                empty_app_storage //-- whatever the error is we have to return and empty app storage instance 
            }
        }
    } else{
        empty_app_storage
    };


    






       
    
    
    
    

    // -------------------------------- building services, signal channel handling and server setup
    //
    // --------------------------------------------------------------------------------------------------------
    if service_name.as_str() == "auth"{
        info!("running auth server on port {} - {}", service_port, chrono::Local::now().naive_local());
        let auth_serivce = services::auth::AuthSvc::new(db.clone()).await; //-- making a new auth server by passing the generated storage
        if server_addr == None{
            server_addr = Some(auth_server_addr);
        }
        let mut auth_server = Server::bind(&server_addr.unwrap()).serve(auth_serivce.clone()); //-- since Copy trait is not implemented we must clone the auth_service to prevent the type from moving - we have to define the auth_server as mutable cause we want to take a mutable raw pointer ot it
        // ------------------------------------------------
        //     BUILDING RUNTIME OBJECT FROM AUTH SERVICE
        // ------------------------------------------------
        let mut raw_pointer_to_server = &mut auth_server as *mut Server<AddrIncoming, services::auth::AuthSvc>; //-- taking a mutable raw pointer to the auth_server to cast it to usize later
        let runtime = Some(
            ctx::rafael::env::Runtime::<services::auth::AuthSvc>{ //-- building runtime instance for the auth server 
                current_service: Some(auth_serivce), //-- later we can bind this service to a an address to run the its server - by this pattern service actors can communicate with each other before running their servers 
                link_to_server: Some(ctx::rafael::env::LinkToService(raw_pointer_to_server as usize)), //-- creating a link to the auth service by casting its mutable raw pointer to a usize which can be either 64 bits (8 bytes) or 32 bits (4 bytes) based on arch of the system
                id: Uuid::new_v4(),
                error: None,
                node_addr: Some(auth_server.local_addr()), //-- local address of this server which has been bound to
                last_crash: None,
                first_init: Some(Local::now().timestamp()),
            }
        );
        // ------------------------------------------------
        // ------------------------------------------------
        let auth_graceful = auth_server.with_graceful_shutdown(ctx::app::shutdown_signal(receiver));
        if let Err(e) = auth_graceful.await{ //-- awaiting on the server to receive the shutdown signal
            error!("auth server error {} - {}", e, chrono::Local::now().naive_local());
        }
        // TODO - if the number of clients reached too many requests shutdown the server
        // TODO - call add_client() method to add an address into the clients vector
        // ...
        sender.send(0).unwrap(); //-- trigerring the shutdown signal on some bad event like DDOS or anything shitty 
        // sender.send(1); //-- freez feature
        // -----------------------------------------------------
        //   RUNNING SERVERLESS ENGINE OF THE RUNTIME INSTANCE 
        // -----------------------------------------------------
        let mut app = runtime.unwrap(); //-- to borrow the instance of the runtime as mutable we must define the app as mutable since the first param of the run() method is &mut self which is a mutable reference or pointer to all runtime instance fields
        app.run(); //-- run the runtime app in serverless mode  

        // 
        // ...
        //

        Ok(())

    } else if service_name.as_str() == "event"{
        info!("running event server on port {} - {}", service_port, chrono::Local::now().naive_local());
        let event_service = services::event::EventSvc::new(db.clone()).await;
        if server_addr == None{
            server_addr = Some(auth_server_addr);
        }
        let mut event_server = Server::bind(&server_addr.unwrap()).serve(event_service.clone()); //-- since Copy trait is not implemented we must clone the auth_service to prevent the type from moving - we have to define the event_server as mutable cause we want to take a mutable raw pointer ot it
        // ------------------------------------------------
        //    BUILDING RUNTIME OBJECT FROM EVENT SERVICE
        // ------------------------------------------------
        let mut raw_pointer_to_server = &mut event_server as *mut Server<AddrIncoming, services::event::EventSvc>; //-- taking a mutable raw pointer to the event_server to cast it to usize later
        let runtime = Some( //-- since the first param of the run() method of the Serverless trait defined as mutable the runtime object must be defined as mutable also
            ctx::rafael::env::Runtime::<services::event::EventSvc>{ //-- building runtime instance for the event server
                current_service: Some(event_service), //-- later we can bind this service to a an address to run the its server - by this pattern service actors can communicate with each other before running their servers 
                link_to_server: Some(ctx::rafael::env::LinkToService(raw_pointer_to_server as usize)), //-- creating a link to the auth service by casting its mutable raw pointer to a usize which can be either 64 bits (8 bytes) or 32 bits (4 bytes) based on arch of the system
                id: Uuid::new_v4(),
                error: None,
                node_addr: Some(event_server.local_addr()), //-- local address of this server which has been bound to
                last_crash: None,
                first_init: Some(Local::now().timestamp()),
            }
        );
        // ------------------------------------------------
        // ------------------------------------------------
        let event_graceful = event_server.with_graceful_shutdown(ctx::app::shutdown_signal(receiver));
        if let Err(e) = event_graceful.await{ //-- awaiting on the server to receive the shutdown signal
            error!("event server error {} - {}", e, chrono::Local::now().naive_local());
        }
        // TODO - if the number of clients reached too many requests shutdown the server
        // TODO - call add_client() method to add an address into the clients vector
        // ...
        sender.send(0).unwrap(); //-- trigerring the shutdown signal on some bad event like DDOS or anything shitty 
        // sender.send(1); //-- freez feature
        // -----------------------------------------------------
        //   RUNNING SERVERLESS ENGINE OF THE RUNTIME INSTANCE 
        // -----------------------------------------------------
        let mut app = runtime.unwrap(); //-- to borrow the instance of the runtime as mutable we must define the app as mutable since the first param of the run() method is &mut self which is a mutable reference or pointer to all runtime instance fields
        app.run(); //-- run the runtime app in serverless mode  

        // 
        // ...
        //

        Ok(())
    } else if service_name.as_str() == "game"{
        info!("running game server on port {} - {}", service_port, chrono::Local::now().naive_local());
        let game_service = services::game::PlayerSvc::new(db.clone()).await;
        if server_addr == None{
            server_addr = Some(auth_server_addr);
        }
        let mut game_server = Server::bind(&server_addr.unwrap()).serve(game_service.clone()); //-- since Copy trait is not implemented we must clone the auth_service to prevent the type from moving - we have to define the game_server as mutable cause we want to take a mutable raw pointer ot it
        // ------------------------------------------------
        //    BUILDING RUNTIME OBJECT FROM GAME SERVICE
        // ------------------------------------------------
        let mut raw_pointer_to_server = &mut game_server as *mut Server<AddrIncoming, services::game::PlayerSvc>; //-- taking a mutable raw pointer to the game_server to cast it to usize later
        let runtime = Some( //-- since the first param of the run() method of the Serverless trait defined as mutable the runtime object must be defined as mutable also
            ctx::rafael::env::Runtime::<services::game::PlayerSvc>{ //-- building runtime instance for the game server
                current_service: Some(game_service), //-- later we can bind this service to a an address to run the its server - by this pattern service actors can communicate with each other before running their servers 
                link_to_server: Some(ctx::rafael::env::LinkToService(raw_pointer_to_server as usize)), //-- creating a link to the auth service by casting its mutable raw pointer to a usize which can be either 64 bits (8 bytes) or 32 bits (4 bytes) based on arch of the system
                id: Uuid::new_v4(),
                error: None,
                node_addr: Some(game_server.local_addr()), //-- local address of this server which has been bound to
                last_crash: None,
                first_init: Some(Local::now().timestamp()),
            }
        );
        // ------------------------------------------------
        // ------------------------------------------------
        let game_graceful = game_server.with_graceful_shutdown(ctx::app::shutdown_signal(receiver));
        if let Err(e) = game_graceful.await{ //-- awaiting on the server to receive the shutdown signal
            error!("game server error {} - {}", e, chrono::Local::now().naive_local());
        }
        // TODO - if the number of clients reached too many requests shutdown the server
        // TODO - call add_client() method to add an address into the clients vector
        // ...
        sender.send(0).unwrap(); //-- trigerring the shutdown signal on some bad event like DDOS or anything shitty 
        // sender.send(1); //-- freez feature
        // -----------------------------------------------------
        //   RUNNING SERVERLESS ENGINE OF THE RUNTIME INSTANCE 
        // -----------------------------------------------------
        let mut app = runtime.unwrap(); //-- to borrow the instance of the runtime as mutable we must define the app as mutable since the first param of the run() method is &mut self which is a mutable reference or pointer to all runtime instance fields
        app.run(); //-- run the runtime app in serverless mode  

        // 
        // ...
        //

        Ok(())
    } else if service_name.as_str() == "nft"{
        info!("running nft server on port {} - {}", service_port, chrono::Local::now().naive_local());
        let nft_service = services::nft::NftSvc::new(db.clone()).await;
        if server_addr == None{
            server_addr = Some(auth_server_addr);
        }
        let mut nft_server = Server::bind(&server_addr.unwrap()).serve(nft_service.clone()); //-- since Copy trait is not implemented we must clone the auth_service to prevent the type from moving - we have to define the nft_server as mutable cause we want to take a mutable raw pointer ot it
        // ------------------------------------------------
        //    BUILDING RUNTIME OBJECT FROM GAME SERVICE
        // ------------------------------------------------
        let mut raw_pointer_to_server = &mut nft_server as *mut Server<AddrIncoming, services::nft::NftSvc>; //-- taking a mutable raw pointer to the nft_server to cast it to usize later
        let runtime = Some( //-- since the first param of the run() method of the Serverless trait defined as mutable the runtime object must be defined as mutable also
            ctx::rafael::env::Runtime::<services::nft::NftSvc>{ //-- building runtime instance for the nft server
                current_service: Some(nft_service), //-- building runtime instance for the nft server
                link_to_server: Some(ctx::rafael::env::LinkToService(raw_pointer_to_server as usize)), //-- creating a link to the auth service by casting its mutable raw pointer to a usize which can be either 64 bits (8 bytes) or 32 bits (4 bytes) based on arch of the system
                id: Uuid::new_v4(),
                error: None,
                node_addr: Some(nft_server.local_addr()), //-- local address of this server which has been bound to
                last_crash: None,
                first_init: Some(Local::now().timestamp()),
            }
        );
        // ------------------------------------------------
        // ------------------------------------------------
        let game_graceful = nft_server.with_graceful_shutdown(ctx::app::shutdown_signal(receiver));
        if let Err(e) = game_graceful.await{ //-- awaiting on the server to receive the shutdown signal
            error!("game server error {} - {}", e, chrono::Local::now().naive_local());
        }
        // TODO - if the number of clients reached too many requests shutdown the server
        // TODO - call add_client() method to add an address into the clients vector
        // ...
        sender.send(0).unwrap(); //-- trigerring the shutdown signal on some bad event like DDOS or anything shitty 
        // sender.send(1); //-- freez feature
        // -----------------------------------------------------
        //   RUNNING SERVERLESS ENGINE OF THE RUNTIME INSTANCE 
        // -----------------------------------------------------
        let mut app = runtime.unwrap(); //-- to borrow the instance of the runtime as mutable we must define the app as mutable since the first param of the run() method is &mut self which is a mutable reference or pointer to all runtime instance fields
        app.run(); //-- run the runtime app in serverless mode  


        // 
        // ...
        // 


        Ok(())
    } else{
        Ok(())
    }
    
    




}
