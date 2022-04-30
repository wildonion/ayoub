#!/bin/bash



# -------------------------------
# a PaaS cli for ayoub framework
# -------------------------------


# ./ayoub.sh build --service <SERVICE_NAME> -> build and compile the selected service (game, auth, event or nft)
# ./ayoub.sh make --service <SERVICE_NAME> -> make a service with controller, schemas, routers and service files
# ./ayoub.sh run --service <SERVICE_NAME> --port-number <PORT> -> run a built service on desired port (game, auth, event or nft)
# ./ayoub.sh deploy --service <SERVICE_NAME> --port-number <PORT> -> deploy the selected compiled wasm of runtime actor on cloud using docker
# ./ayoub.sh call  --service <SERVICE_NAME> --method-name <METHOD> -> call a serverless method from the compiled wasm of runtime actor of a selected service  
# ./ayoub.sh list --api --controller <CONTROLLER_NAME> -> list all available apis related to a controller (game, auth, event and nft)


# cargo build --bin ayoub --release

# ☢️ To run the `event` server: `cargo run auth 7436`

# ☢️ To run the `auth` server: `cargo run event 7435`

# ☢️ To run the `game` server: `cargo run event 7437`

# ☢️ To run the `nft` server: `cargo run nft 7438`

# -----------------------------------------------------------

# ☢️ To run the `event` server: `./ayoub auth 7436`

# ☢️ To run the `auth` server: `./ayoub event 7435`

# ☢️ To run the `game` server: `./ayoub event 7437`

# ☢️ To run the `nft` server: `./ayoub nft 7438`
