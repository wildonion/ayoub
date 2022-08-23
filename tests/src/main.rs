






use utils;







fn main(){
   

    
    utils::trash();
    utils::mactrait();
    utils::unsafer();



    /*
    

        types bounded to Sync and Send:
            Arc
            Mutex
            RwLock
        types not bounded to Sync and Send:
            Rc
            RefCell
            Cell


        if a type is not send and sync it means we can't move its references between threads safely and we have to put it inside Arc since &Arc<T> is Send thus Arc<T> is also Sync


        in rust everything is all about having a type and size thus must be generic 
        and borrowing them using & to share them between other scopes like threads and functions
        a shareable data means it can be copied or cloned and safe to send and mutate between threads

        Arc will be used instead of Rc in multi threading to avoid data races and is Send means all its references can be shared between threads and is an atomic reference to a type
        if &T is Send then T can be also Sync thus in order to share a data between threads safely the type must be bounded to Send + Sync + 'static means it must be 
        cloneable or shareable between threads means we can simply borrow it to move it between threads and sync with other threads to avoid mutating it by multiple threads
        at the same time
        if there is no possibility of undefined behavior like data races when passing &T between threads means &T must be Send then T is alos Sync and &mut T is Sync if T is Sync
    


        - a type might be mutated by other threads thus we have to put it inside Mutex or RwLock to avoid data races means that only one thread can mutate the state of a type
        - shareable rules : data which are Send + Sync + 'static must be share and trasferred between threads using mpsc channel
        - instead of moving types into the thread we can borrow them to have them outside the threads
        - based on mpsc rust has defined the rule which says multiple immutable can be inside a scope but only one of them can be mutable
        - in order to share data (T must have shareable rules) between threads we have to use mpsc channel 
    
    
    
    */
    

    
    // thread scope
    // const Mutex instead of lazy_static
    // fn foo<T>(value: T, f: impl Copy)
    // ...
    // 
















}
