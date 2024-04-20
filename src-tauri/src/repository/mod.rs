use rusqlite::{Connection,Result};
use crate::model;

// return a connection.
pub fn connect_sql(id:usize)->Result<Connection>{
    let conn = Connection::open(format!("{}_data.db",id))?;
    Ok(conn)
}
fn table_exist(conn:&Connection,table_name:&str)->Result<bool>{
    let mut statement = conn.prepare(
        "PRAGMA table_list",
    )?;
    // 判断指定表是否存在
    let table_list = statement.query_map([], |row| {
        Ok(model::TableName{
            name:row.get(0)?,
        })
    })?;
    for table in table_list {
        if table.expect("failed to get table name").name.eq(table_name) {
            return Ok(true);
        }
    }
    Ok(false)
}
// get session list.
pub fn get_session_list(conn:&Connection)->Result<Vec<model::Session>>{
    let table_name = "session";
    let mut res: Vec<model::Session> = Vec::new();
    if table_exist(conn, table_name)?{
        // get all session
        let mut statement = conn.prepare(
            "SELECT * FROM (?1)",
        )?;
        let session_list = statement.query_map([table_name], |row| {
            Ok(model::Session{
                id:row.get(0)?,
                avatar:row.get(1)?,
                last:row.get(2)?,
                last_time:row.get(3)?,
                un_read:row.get(4)?,
                name:row.get(5)?,
                remark:row.get(6)?,
                session_type:row.get(7)?
        })
    })?;
    for session in session_list {
        res.push(session.expect("failed to get session"));
    }
    Ok(res)

    }else {
        let sql = "CREATE TABLE IF NOT EXISTS session (id,avatar,last,last_time,un_read,name,remark,session_type)";
        conn.execute(sql, [])?;
        Ok(res)
    }
}

pub fn get_group_list(conn:&Connection)->Result<Vec<model::Group>>{
    let table_name = "group";
    let mut res: Vec<model::Group> = Vec::new();
    if table_exist(conn, table_name)?{
        // get all group.
        let mut statement = conn.prepare(
            "SELECT * FROM (?1)",
        )?;
        let group_list = statement.query_map([table_name], |row| {
            Ok(model::Group{
                id:row.get(0)?,
                avatar:row.get(1)?,
                desc:row.get(2)?,
            })
        })?;
        for group in group_list {
            res.push(group.expect("failed to get group"));
        }
        Ok(res)
    }else {
        let mut statement = conn.prepare("");
        let sql = "";
        conn.execute(sql, [""])?;
        Ok(res)
    }
}

pub fn get_friend_list(conn:&Connection)->Result<Vec<model::Friend>>{
    let table_name = "friend";
    let mut res: Vec<model::Friend> = Vec::new();
    if table_exist(conn, table_name)?{
        // get all friend.
        let mut statement = conn.prepare(
            "SELECT * FROM (?1)",
            
        )?;
        let friend_list = statement.query_map([table_name], |row| {
            Ok(model::Friend{
                id:row.get(0)?,
                avatar:row.get(1)?,
                name:row.get(2)?,
                remark:row.get(3)?,
            })
        })?;

    for friend in friend_list {
        res.push(friend.expect("failed to get friend"));
    }
    Ok(res)
    }else {
        let sql = "CREATE TABLE IF NOT EXISTS friend (id,avatar,name,remark)";
        conn.execute(sql, [])?;
        Ok(res)
    }
}