# ayoub Skeleton Development Setup

### Requirements

* **Remember that _auth_/_suproxy_ microservice is responsible for handling all CRUD operations related to _postgres_/_cassandra_**

* **Install _rustup_, _pm2_, _postgres_, _cassandra_**

* **Install prerequisite packages on Linux:** ```sudo apt install openssl libssl-dev cmake libpq-dev```

* **Install _openssl_ for _diesel_ using ```choco install openssl``` and _gcc_/_g++_ with _mingw_ using ```choco install mingw``` on Windows** 

* **Put _postgres_ lib and bin path into environment variable on Windows:** ```C:\Program Files\PostgreSQL\<version>\lib``` and ```C:\Program Files\PostgreSQL\<version>\bin```

* **Install _cargo_ extra packages:** ```cargo install diesel_cli --no-default-features --features postgres && cargo install systemfd cargo-watch```  

### Updating `auth` Microservice API Acess Level

* **Updating access level to admin access:** ```cd auth/ && cargo run <USERNAME> <ACCESS_LEVEL>```
    * **eg - change access level of user _wildonion_ to admin level:** ```cd auth/ && cargo run wildonion 2```

### Running Microservices Commands

* **Run _auth_ microservice using one the following commands:** 
    * ```systemfd --no-pid -s http::7366 -- cargo watch -C auth -x run```
    * ```cargo watch -C auth -x run```

* **Run _suproxy_ load balancer using one the following commands:**
    * ```systemfd --no-pid -s http::7368 -- cargo watch -C suproxy -x run```
    * ```cargo watch -C suproxy -x run```

* **Run _coiniXerr_ network:**
    * ```cargo watch -C coiniXerr -x run```

* **Run _ayoub_ server:**
    * ```cargo watch -C ayoub -x run```

# ayoub Skeleton Production Setup

### Setup Postgres DB and User

```
CREATE DATABASE ayoub;
CREATE USER ayoub WITH ENCRYPTED PASSWORD 'ayoub';
GRANT ALL PRIVILEGES ON DATABASE ayoub TO ayoub;
ALTER USER ayoub WITH SUPERUSER;
```

* **Build & run each microservice:** ```sudo chmod +x deploy.sh && ./deploy.sh```

### ayoub Skeleton Postgres Database Setup

* **Generate _migrations_ folder, create ayoub postgres db, `diesel.toml` file on first run or run existing migrations into the database:** 

    * ```cd server/skeleton/microservices && diesel setup --migration-dir auth/migrations/```

* **Generate SQL files for your table operations:** ```diesel migration generate SQL-OPERATION_TABLE-NAME```

    * **eg - create users table for _auth_ microservice:** ```diesel migration generate create_users --migration-dir auth/migrations/```

* **Migrate tables into postgres db and generate(update) `schema.rs` file inside _src_ folder:** ```diesel migration run```

    * **eg - migrate all SQL files of operations of _auth_ microservice into the database:** ```diesel migration run --migration-dir auth/migrations/```
    * **note - in order to generate the `schema.rs` in _src_ folder the ```diesel migration run``` command must have a successful result**
    * **note - you can also create sql files (`up.sql` and `down.sql`) for your table in each migrations folder by hand then run the ```diesel setup``` command to migrate them all into the db at once**
    * **note - down migration command for each table is: ```diesel migration down```**

* **Check diesel migrations errors:** ```diesel migration list```

    * **eg - check migrations errors for _auth_ microservice:** ```diesel migration list --migration-dir auth/migrations/```