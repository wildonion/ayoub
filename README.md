

# ☢️ Run in Dev Mode

> ```sudo chown -R root:root /home/$USER/ayoub && sudo chmod 777 -R /home/$USER/ayoub```

> ayoub PaaS: ```cargo run --bin ayoub``` or ```cargo run --bin ayoub -- <SERVICE_NAME> <SERVICE_PORT>``` like ```cargo run --bin ayoub -- auth 37465``` 

* If you don't pass the required arguments, it'll set the current service and its port from `.env` file.

> coiniXerr: ```cargo run --bin coiniXerr```

> tests: ```cargo run --bin tests```

> Rafael WASM Runtime: ```sudo chmod +x PaaS/src/runtime/build.sh && cd PaaS/src/runtime/ && ./build.sh```

# ☣️ Build for Production

> ayoub PaaS: ```cargo build --bin ayoub --release```

* To update a user access level to dev first signup the user using `/auth/signup` API then run the binary like so: `./ayoub wildonion 0`

* 🆔 Run `auth` service: ```./ayoub auth 8335```

* 🗓️ Run `event` service: ```./ayoub event 8336```

* 🎲 Run `game` service: ```./ayoub game 8337```

> To Run and Setup All ayoub PaaS services: ```sudo bash ayoub.sh```

> coiniXerr: ```cargo build --bin coiniXerr --release```

> tests: ```cargo build --bin tests --release```

# 🗄️Run All in Production

> ```./ayoub.sh --help``` to see all available commands

# 💰 coiniXerr 

An Actor and Sharded Based Design Pattern Runtime and Engine for uniXerr Cryptocurrency Coin, CRC20, CRC21 and CRC22 Smart Contract; to Mint NFT and FT for Digital Assests inside uniXerr Protocol on top of coiniXerr Blockchain Network

# 🌀 PaaS 

Core Backend of the Ayoub PaaS Framework with Flexible Design Pattern for Pay-As-You-Go Requests  

# 📌 TODOs

* coiniXerr TODOs