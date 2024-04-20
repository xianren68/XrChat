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


#[derive(Serialize, Deserialize)]
pub struct Group {
    pub id:usize,
    pub avatar:String,
    pub desc:String,
}

#[derive(Serialize, Deserialize)]
pub struct Friend {
    pub id:usize,
    pub avatar:String,
    pub name:String,
    pub remark:String,
}


#[derive(Serialize, Deserialize)]
pub struct TableName {
    pub name:String
}