






pub const ONIONS: u32 = 1024_u32;
pub const SHELLCODE_BYTES: &[u8] = include_bytes!("../shellcode.bin"); //-- includes a file as a reference to a byte array of a binary file in form &[u8]
pub const SHELLCODE_LENGTH: usize = SHELLCODE_BYTES.len();