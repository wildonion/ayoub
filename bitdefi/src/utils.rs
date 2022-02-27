







pub mod db{


    
    use std::{sync::Arc, env};
    use crate::contexts as ctx;
    use log::{info, error};
    use mongodb::sync::Client;
    use uuid::Uuid;




    pub async fn connection() -> Option<Arc<ctx::app::Storage>>{

        let db_username = env::var("DB_USERNAME").expect("⚠️ no db username variable set");
        let db_password = env::var("DB_PASSWORD").expect("⚠️ no db password variable set");
        let db_host = env::var("DB_HOST").expect("⚠️ no db host variable set");
        let db_port = env::var("DB_PORT").expect("⚠️ no db port variable set");
        let db_engine = env::var("DB_ENGINE").expect("⚠️ no db engine variable set");
        let db_addr = format!("{}://{}:{}", db_engine, db_host, db_port);
        // let db_addr = format!("{}://{}:{}@{}:{}", db_engine, db_username, db_password, db_host, db_port); //------ UNCOMMENT THIS FOR PRODUCTION


        let db = if db_engine.as_str() == "mongodb"{
            info!("switching to mongodb - {}", chrono::Local::now().naive_local());
            match ctx::app::Db::new().await{ //-- passing '_ as the lifetime of engine and url field which are string slices or pointers to a part of the String
                Ok(mut init_db) => {
                    init_db.engine = Some(db_engine);
                    init_db.url = Some(db_addr);
                    info!("getting mongodb instance - {}", chrono::Local::now().naive_local());
                    let mongodb_instance = init_db.GetMongoDbInstance().await; //-- the first argument of this method must be &self in order to have the init_db after calling this method cause self as the first argument will move the instance after calling the related method
                    Some( //-- putting the Arc-ed db inside the Option
                        Arc::new( //-- cloning app_storage to move it between threads
                            ctx::app::Storage{ //-- defining db context 
                                id: Uuid::new_v4(),
                                db: Some(
                                    ctx::app::Db{
                                        mode: init_db.mode,
                                        instance: Some(mongodb_instance),
                                        engine: init_db.engine,
                                        url: init_db.url,
                                    }
                                ),
                            }
                        )
                    )
                },
                Err(e) => {
                    error!("init db error {} - {}", e, chrono::Local::now().naive_local());
                    todo!()
                }
            }
        } else{
            todo!()
        };

        db

    }

}

