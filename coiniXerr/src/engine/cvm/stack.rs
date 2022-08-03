




// TODO - VM, interpreter and #[wowasm] proc macro attribute to write smart contracts with wo syntax to compile to wasm to run on near
// ...


use crate::*; // loading all defined crates, structs and functions from the root crate which is lib.rs in our case








#[macro_export]
macro_rules! wowasm {
    ($iden:ident, $ty: tt) => {
        
        // https://doc.rust-lang.org/reference/procedural-macros.html
        // TODO - build function like macro like query!() and custom inner and outter trait like proc macro attributes and derive like; on structs, fields, modules and functions like #[near_bindgen] and #[borsh_skip] proc macro attribute, #[custom(token_stream)] and #[derive(Clone)] style 
        // TODO - write proc macro attributes and derives with TokenStream arg using proc_macro2 crate and proc-macro = true flag inside the lib.rs file by using #[proc_macro], #[proc_macro_attribute] and #[proc_macro_derive] attributes  
        // TODO - a proc macro attribute to convert a trait into a module and its methods into static methods of that module and add extra args like the ones for nft_on_transfer() and nft_on_approve() methods when the user is implementing these methods
        // NOTE - #[derive(Trait, SomeMacro)] bounds a struct to a trait or a macro
        // NOTE - #[..] applies an attribute to the thing after it (struct, struct fields or crate) and  #![..] applies an attribute to the containing thing or crate
        // ...
        pub struct $iden(pub $ty);
        impl Default for $iden{
            fn default() -> Self{
                todo!()
            }
        }
        
    };
}







#[macro_export]
macro_rules! query { // NOTE - this is a macro with multiple syntax support and if any pattern matches with the caller pattern, then the code block of that pattern will be emitted
    
    ( $value_0:expr, $value_1:expr, $value_2:expr ) => { //-- passing multiple object syntax
        // ...
    };

    ( $($name:expr => $value:expr),* ) => { //-- passing multiple key => value syntax 
        // ...

    };

}


#[macro_export]
macro_rules! log {
    ($arg:tt) => { //-- passing single String message 
        $crate::env::log($arg.as_bytes()) //-- log function only accepts utf8 bytes
    };
    ($($arg:tt)*) => { //-- passing multiple String messages 
        $crate::env::log(format!($($arg)*).as_bytes()) //-- log function only accepts utf8 bytes
    };
}


#[macro_export]
macro_rules! impl_engine_constructor {
    ($( $new:ident: [ $( $pos:expr ),* ] anchored at $anchor:expr; )*) => {
        $(
            pub fn $new() -> Self{
                Self{
                    positions: [$( $pos ),*].into_iter().collect(),
                    anchor: $anchor,
                }
            }
        )* //-- * m//eans defining function for every new Pos
    };
}


// #[derive(Debug, Clone)]
// pub struct Shape{
//     typ: &'static str,
//     positions: HashSet<Pos>,
//     anchor: Pos,
// }


// #[derive(Debug, Clone, Copy)]
// pub struct Pos(pub i32, pub i32);



// impl Shape {
//     impl_engine_constructor! {
//       new_i "🟦": [Pos(0, 0), Pos(1, 0), Pos(2, 0), Pos(3, 0)] @ Pos(1, 0);
//       new_o "🟨": [Pos(0, 0), Pos(1, 0), Pos(0, 1), Pos(1, 1)] @ Pos(0, 0);
//       new_t "🟫": [Pos(0, 0), Pos(1, 0), Pos(2, 0), Pos(1, 1)] @ Pos(1, 0);
//       new_j "🟪": [Pos(0, 0), Pos(0, 1), Pos(0, 2), Pos(-1, 2)] @ Pos(0, 1);
//       new_l "🟧": [Pos(0, 0), Pos(0, 1), Pos(0, 2), Pos(1, 2)] @ Pos(0, 1);
//       new_s "🟩": [Pos(0, 0), Pos(1, 0), Pos(0, 1), Pos(-1, 1)] @ Pos(0, 0);
//       new_z "🟥": [Pos(0, 0), Pos(-1, 0), Pos(0, 1), Pos(1, 1)] @ Pos(0, 0);
//     }
// }