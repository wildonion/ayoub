#!/bin/bash



# -------------------------------
# a PaaS cli for ayoub framework
# -------------------------------


# ./ayoub.sh build --service-name -> build and compile the selected service 
# ./ayoub.sh make --service-name -> make a service with controller, schemas, routers and service files
# ./ayoub.sh run --service-name --port-number -> run a built service on desired port
# ./ayoub.sh deploy --service-name --port-number -> deploy the selected service on cloud


# cargo build --bin ayoub --release


# ☢️ To run the `event` server: `./ayoub auth 7436`

# ☢️ To run the `auth` server: `./ayoub event 7435`

# ☢️ To run the `game` server: `./ayoub event 7437`