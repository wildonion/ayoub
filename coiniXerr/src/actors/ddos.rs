





use crate::*;
use super::*; //-- super referes to the root directory which is the actors itself - loading all modules which are loaded inside the actors.rs







// https://medium.com/@ukpaiugochi0/building-a-cli-from-scratch-with-clapv3-fb9dc5938c82   
// https://blog.logrocket.com/command-line-argument-parsing-rust-using-clap/

// ddos attack macrors, closures, Box, traits and clap
// ...

//--------------------------------------
// building and calling at the same time
//--------------------------------------
// (|num: u8| async move{
//     num;
// })(32).await;



//-------------------------------------------------
// fill the struct fields using unpacking and match 
//-------------------------------------------------
// let name = Some("wildonion".to_string());
// struct User{
//     username: String,
//     age: u8,
// }

// let user = User{
//     username: match name{
//         Some(name) => name,
//         None => "".to_string(),
//     },
//     age: 26,
// };


// let User{username, age} = user; //-- unpacking struct