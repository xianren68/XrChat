use crate::model;
use crate::repository;
#[tauri::command]
pub fn get_session_list(id: usize) -> Vec<model::Session> {
    let conn = repository::connect_sql(id).expect("err to connect database");
    let res = repository::get_session_list(&conn);
    match res {
        Ok(r) => {
            conn.conn.close().expect("err to close database");
            r
        }
        Err(e) => {
            panic!("err to read data:{}", e)
        }
    }
}

#[tauri::command]
pub fn get_friend_list(id: usize) -> Vec<model::Friend> {
    let conn = repository::connect_sql(id).expect("err to connect database");
    let res = repository::get_friend_list(&conn);
    match res {
        Ok(r) => {
            conn.conn.close().expect("err to close database");
            r
        }
        Err(e) => {
            panic!("err to read data:{}", e);
        }
    }
}

#[tauri::command]
pub fn get_group_list(id: usize) -> Vec<model::Group> {
    let conn = repository::connect_sql(id).expect("err to connect database");
    let res = repository::get_group_list(&conn);
    match res {
        Ok(r) => r,
        Err(e) => {
            panic!("err to read data:{}", e)
        }
    }
}
