







/* 

    NOTE - for calling private method current_account_id must be equal to predecessor_account_id (account of the contract)
    NOTE - Box<T> is one of the smart pointers in the Rust standard library, it provides a way to allocate enough memory on the heap to store a value of the corresponding type, and then it serves as a handle, a pointer to that memory
    NOTE - Box<T> owns the data it points to; when it is dropped, the corresponding piece of memory on the heap is deallocated and we can use dereference operator to reach their contents
    NOTE - every method call is a transaction in smart contract concepts
    NOTE - since can't compile socket in lib (wasm and bpf) mode contracts can't interact with their outside worlds  thus we can't have whether tokio or any web framework
    NOTE - bytecodes like .wasm and .so are compiled codes (on RAM instructions) from other langs must be loaded into a buffer to execute them on RAM using VMs
    NOTE - this contract (a family tree contract) is our campaign in which will catch a commission from incoming lamports and transfer the rest to the family tree owner account
    NOTE - funder will send a transaction also contains some instruction data to transfer lamports from his/her address to our campaign address (escrow)
    NOTE - our campaign contract contains some methods like TransferingWithCommission(), LockWallet() and MakeCampaignEmpty()
    NOTE - our campaign contract's methods will be called on a specific event or condition and that's what a smart contract does!
    NOTE - near uses actor based model to call smart contract methods and pass data between them asyncly using their address (Addr object) which means we can have multi threading in wasm file
    NOTE - sharded blockchain means that every shard is an actor which contains multiple blocks and each will communicates with each other using addr object like passing data by calling a contract method inside block a to another contract method inside the block b asyncly

*/










mod utils; //-- or crate::utils
mod contracts;

