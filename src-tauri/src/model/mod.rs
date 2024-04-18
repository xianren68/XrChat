use serde::{Serialize,Deserialize};

/// Session info model
#[derive(Serialize, Deserialize)]
pub struct Session {
    pub id:usize,
    pub avatar:String,
    pub last:String,
    pub last_time:String,
    pub un_read:usize,
    pub name:String,
    pub remark:String,
    pub session_type:bool
}

pub struct Group {
    pub id:usize,
    pub avatar:String,
    pub desc:String,
}
pub struct TableName {
    pub name:String
}