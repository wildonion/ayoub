












pub mod jwt{



    use serde::{Serialize, Deserialize};



    #[derive(Debug, Serialize, Deserialize)]
    pub struct Claims{}



    pub fn encode(payload: Claims){}


}
