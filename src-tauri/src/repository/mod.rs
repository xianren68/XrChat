use crate::api;
use crate::model;
use crate::pb::relation;
use rusqlite::{Connection, Result};
pub struct MyConn {
    pub conn: Connection,
    pub id: usize,
}
// return a connection.
pub fn connect_sql(id: usize) -> Result<MyConn> {
    let conn = Connection::open(format!("{}_data.db", id))?;
    Ok(MyConn { conn, id })
}
fn table_exist(conn: &MyConn, table_name: &str) -> Result<bool> {
    let sql = format!(
        "SELECT count(*) FROM sqlite_master WHERE type='table' AND name='{}'",
        table_name
    );
    let count: i64 = conn.conn.query_row(&sql, [], |row| row.get(0))?;
    Ok(count > 0)
}
// get session list.
pub fn get_session_list(conn: &MyConn) -> Result<Vec<model::Session>> {
    let table_name = "session";
    let mut res: Vec<model::Session> = Vec::new();
    if table_exist(conn, table_name)? {
        // get all session
        let mut statement = conn.conn.prepare(&format!("SELECT * FROM {}",table_name))?;
        let session_list = statement.query_map([], |row| {
            Ok(model::Session {
                id: row.get(0)?,
                avatar: row.get(1)?,
                last: row.get(2)?,
                last_time: row.get(3)?,
                un_read: row.get(4)?,
                name: row.get(5)?,
                remark: row.get(6)?,
                session_type: row.get(7)?,
            })
        })?;
        for session in session_list {
            res.push(session.expect("failed to get session"));
        }
        Ok(res)
    } else {
        create_session_table(conn)?;
        Ok(res)
    }
}

pub fn get_group_list(conn: &MyConn) -> Result<Vec<model::Group>> {
    let table_name = "group";
    let mut res: Vec<model::Group> = Vec::new();
    if table_exist(conn, table_name)? {
        // get all group.
        let mut statement = conn.conn.prepare("SELECT * FROM (?1)")?;
        let group_list = statement.query_map([table_name], |row| {
            Ok(model::Group {
                id: row.get(0)?,
                name: row.get(1)?,
                avatar: row.get(2)?,
                desc: row.get(3)?,
                ownerid: row.get(4)?,
            })
        })?;
        for group in group_list {
            res.push(group.expect("failed to get group"));
        }
        Ok(res)
    } else {
        let mut statement = conn.conn.prepare("");
        let sql = "";
        conn.conn.execute(sql, [""])?;
        Ok(res)
    }
}

pub fn get_friend_list(conn: &MyConn) -> Result<Vec<model::Friend>> {
    let table_name = "friend";
    let mut res: Vec<model::Friend> = Vec::new();
    if table_exist(conn, table_name)? {
        // get all friend.
        let mut statement = conn.conn.prepare(&format!("SELECT * FROM {}",table_name))?;
        let friend_list = statement.query_map([], |row| {
            Ok(model::Friend {
                id: row.get(0)?,
                name: row.get(1)?,
                remark: row.get(2)?,
                avatar: row.get(3)?,
                email: row.get(4)?,
                phone: row.get(5)?,
                gender: row.get(6)?,
                line: row.get(7)?,
            })
        })?;

        for friend in friend_list {
            res.push(friend.expect("failed to get friend"));
        }
        Ok(res)
    } else {
        create_friend_table(conn)?;
        // get friend_list from mysql.
        if let Ok(friends) = api::relation::get_friend_list(conn.id as u64) {
            Ok(save_all_friend(friends, conn)?)
        }else {
            Ok(res)
        }
    }
}

pub fn get_friend(conn: &MyConn, id: usize) -> Result<model::Friend> {
    let table_name = "friend";
    if table_exist(conn, table_name)? {
        Ok(get_only_friend(conn, id)?)
    } else {
        create_friend_table(conn)?;
        // TODO get friends from backend.
        Ok(get_only_friend(conn, id)?)
    }
}
// get only friend by id.
fn get_only_friend(conn: &MyConn, id: usize) -> Result<model::Friend> {
    let table_name = "friend";
    let sql = "SELECT * FROM (?1) WHERE id = ?";
    let friend = conn.conn.query_row(&sql, &[table_name,&id.to_string()], |row| {
        Ok(model::Friend {
            id: row.get(0)?,
            name: row.get(1)?,
            remark: row.get(2)?,
            avatar: row.get(3)?,
            email: row.get(4)?,
            phone: row.get(5)?,
            gender: row.get(6)?,
            line: row.get(7)?,
        })
    });
    Ok(friend.expect("failed to get friend"))
}

pub fn get_group(conn: &MyConn, id: usize) -> Result<model::Group> {
    let table_name = "group";
    if table_exist(conn, table_name)? {
        Ok(get_only_group(conn, id)?)
    } else {
        create_group_table(conn)?;
        // TODO get groups from backend.
        Ok(get_only_group(conn, id)?)
    }
}
fn get_only_group(conn: &MyConn, id: usize) -> Result<model::Group> {
    let table_name = "group";
    let sql = "SELECT * FROM (?1) WHERE id = ?";
    let group = conn.conn.query_row(&sql, &[table_name,&id.to_string()], |row| {
        Ok(model::Group {
            id: row.get(0)?,
            name: row.get(1)?,
            desc: row.get(2)?,
            avatar: row.get(3)?,
            ownerid: row.get(4)?,
        })
    });
    Ok(group.expect("failed to get group"))
}

fn create_session_table(conn: &MyConn) -> Result<()> {
    let sql = "CREATE TABLE session (
        id INT UNSIGNED  PRIMARY KEY,
        avatar VARCHAR(255),
        last VARCHAR(255),
        last_time BIGINT UNSIGNED,
        un_read INT UNSIGNED,
        name VARCHAR(255),
        remark TEXT,
        session_type INT
    );";
    conn.conn.execute(sql, [])?;
    Ok(())
}
fn create_friend_table(conn: &MyConn) -> Result<()> {
    let sql = "CREATE TABLE friend (
        id INT UNSIGNED  PRIMARY KEY,
        name VARCHAR(255) NOT NULL,
        remark TEXT,
        avatar VARCHAR(255),
        email VARCHAR(255),
        phone VARCHAR(20),
        gender BOOLEAN,
        line VARCHAR(255)
    );";
    conn.conn.execute(sql, [])?;
    Ok(())
}
fn create_group_table(conn: &MyConn) -> Result<()> {
    let sql = "CREATE TABLE IF NOT EXISTS group (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        avatar TEXT,
        desc TEXT,
        ownerid INTEGER
    );";
    conn.conn.execute(sql, [])?;
    Ok(())
}

fn save_all_friend(friends: Vec<relation::Friend>, conn: &MyConn) -> Result<Vec<model::Friend>> {
    let mut friend_list:Vec<model::Friend> = Vec::new();
    for friend in friends {
        let f =  model::Friend {
            id:friend.id as usize,
            name:friend.name,
            remark:friend.remark,
            avatar:friend.avatar,
            email:friend.email,
            phone:friend.phone,
            gender:friend.gender,
            line:friend.line,
        };
        friend_list.push(f);
    }
    // 开始事务
    let mut new_conn = Connection::open(format!("{}_data.db",conn.id))?;
    let tx = new_conn.transaction()?;

    // 执行批量插入操作
    for friend in &friend_list {
        tx.execute(
            "INSERT INTO friend (id, name, remark, avatar, email, phone, gender, line) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            (&friend.id, &friend.name, &friend.remark, &friend.avatar, &friend.email, &friend.phone, &friend.gender, &friend.line),
        )?;
    }

    // 提交事务
    tx.commit()?;
    Ok(friend_list)
}
