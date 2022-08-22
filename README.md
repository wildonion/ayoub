

# ☢️ Run in Dev Mode

> ```sudo chown -R root:root /home/ayoub/ayoub && sudo chmod 777 -R /home/ayoub/ayoub```

> ayoub PaaS: ```cargo run --bin ayoub``` 

* If you don't pass the required arguments, it'll set the current service and its port from `.env` file.

# ☣️ Build for Production

> ayoub PaaS: ```cargo build --bin ayoub --release```

* To update a user access level to dev first signup the user using `/auth/signup` API then run the binary like so: `./ayoub wildonion 0`

> To Run and Setup Ayoub PaaS: ```sudo bash ayoub.sh```

# 🗄️Ayoub PaaS CLI

> ```./ayoub.sh --help``` to see all available commands

# 🌀 PaaS 

Core Backend of the Ayoub PaaS Framework with Flexible Design Pattern for Pay-As-You-Go Requests