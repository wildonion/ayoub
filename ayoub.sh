#!/bin/bash



# -------------------------------
# a PaaS cli for ayoub framework
# -------------------------------


# ./ayoub.sh create --name <PROJECT_NAME> -> create the boilerplate code for a new project including mongodb as db backend, auth, event and nft service as servers
# ./ayoub.sh build --service <SERVICE_NAME> -> build and compile the selected service (game, auth, event or nft)
# ./ayoub.sh make --service <SERVICE_NAME> --middleware <MIDDLEWARE_NAME_TO_USE> -> make a service with controller, schemas, routers and service files with enabled middleware on its routes like auth
# ./ayoub.sh run --service <SERVICE_NAME> --port-number <PORT> -> run a built service on desired port (game, auth, event or nft)
# ./ayoub.sh deploy --service <SERVICE_NAME> --port-number <PORT> -> deploy the selected compiled wasm of runtime actor on cloud using docker
# ./ayoub.sh call  --service <SERVICE_NAME> --method-name <METHOD> -> call a serverless method from the compiled wasm of runtime actor of a selected service  
# ./ayoub.sh list --api --controller <CONTROLLER_NAME> -> list of all available apis related to a controller (game, auth, event and nft)


# sudo chmod 777 -R /home/$USER/ayoub
# sudo chown -R root:root /home/$USER/ayoub
# cargo build --bin ayoub --release



# ☢️ To run the `event` server: `cargo run event 7436`

# ☢️ To run the `auth` server: `cargo run auth 7435`

# ☢️ To run the `game` server: `cargo run game 7437`

# ☢️ To run the `nft` server: `cargo run nft 7438`

# -----------------------------------------------------------

# ☢️ To run the `event` server: `./ayoub event 7436`

# ☢️ To run the `auth` server: `./ayoub auth 7435`

# ☢️ To run the `game` server: `./ayoub game 7437`

# ☢️ To run the `nft` server: `./ayoub nft 7438`


cargo build --bin ayoub2 --release

sudo rm auth.ayoub/ayoub-auth
sudo rm event.ayoub/ayoub-event
sudo rm game.ayoub/ayoub-game
sudo cp target/release/ayoub auth.ayoub/ayoub-auth
sudo cp target/release/ayoub event.ayoub/ayoub-event
sudo cp target/release/ayoub game.ayoub/ayoub-game

sudo pm2 stop ayoub-auth
sudo pm2 stop ayoub-event
sudo pm2 stop ayoub-game
sudo pm2 start auth.ayoub/ayoub-auth --name ayoub-auth -- auth 8335
sudo pm2 start event.ayoub/ayoub-event --name ayoub-event -- event 8336
sudo pm2 start game.ayoub/ayoub-game --name ayoub-game -- game 8337




# -----------------------------------
# a PaaS cli for coiniXerr framework
# -----------------------------------