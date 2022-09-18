#!/bin/bash



# -------------------------------
# a PaaS cli for ayoub framework
# -------------------------------


# ./ayoub.sh create --name <PROJECT_NAME> -> create the boilerplate code for a new project including mongodb as db backend, auth as server
# ./ayoub.sh build -> build and compile the ayoub server with all defined api routes
# ./ayoub.sh make --service <SERVICE_NAME> --middleware <MIDDLEWARE_NAME_TO_USE> -> make a service with controller, schemas and routers files with enabled middleware on its routes like auth
# ./ayoub.sh run --port-number <PORT> -> run the ayoub server on desired port
# ./ayoub.sh deploy --service <SERVICE_NAME> --port-number <PORT> -> deploy the selected compiled wasm of runtime actor on cloud using docker and k8s
# ./ayoub.sh call --service <SERVICE_NAME> --method-name <METHOD> -> call a serverless method from the compiled wasm of runtime actor of a selected service  
# ./ayoub.sh list --api --controller <CONTROLLER_NAME> -> list of all available apis related to a controller





# ---------------------------
# compile rafael FaaS to wasm 
# ---------------------------
curl https://wasmtime.dev/install.sh -sSf | bash
rustup target add wasm32-wasi
cargo build --bin rafael --target wasm32-wasi --release
sudo cp target/wasm32-wasi/release/rafael.wasm ./rafael.awasm
wasmtime rafael.wasm




# ----------------
# build ayoub PaaS
# ----------------
sudo chown -R root:root . && sudo chmod -R 777 .
cargo build --bin ayoub --release
sudo rm app
sudo cp target/release/ayoub ./app
sudo pm2 delete ayoub
sudo pm2 start app --name ayoub




# -----------------------------------
# a PaaS cli for coiniXerr framework
# -----------------------------------