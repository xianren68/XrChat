use serde::{Deserialize, Serialize};

/// Session info model
#[derive(Serialize, Deserialize)]
pub struct Session {
    pub id: usize,
    pub avatar: String,
    pub last: String,
    pub last_time: u64,
    pub un_read: usize,
    pub name: String,
    pub remark: String,
    pub session_type: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Group {
    pub id: usize,
    pub name: String,
    pub avatar: String,
    pub desc: String,
    pub ownerid: usize,
}

#[derive(Serialize, Deserialize)]
pub struct Friend {
    pub id: usize,
    pub name: String,
    pub remark: String,
    pub avatar: String,
    pub email: String,
    pub phone: String,
    pub gender: bool,
    pub line: String,
}

#[derive(Serialize, Deserialize)]
pub struct TableName {
    pub name: String,
}
