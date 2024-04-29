use crate::model;
use rusqlite::{Connection, Result};

// return a connection.
pub fn connect_sql(id: usize) -> Result<Connection> {
    let conn = Connection::open(format!("{}_data.db", id))?;
    Ok(conn)
}
fn table_exist(conn: &Connection, table_name: &str) -> Result<bool> {
    let mut statement = conn.prepare("PRAGMA table_list")?;
    // 判断指定表是否存在
    let table_list = statement.query_map([], |row| Ok(model::TableName { name: row.get(0)? }))?;
    for table in table_list {
        if table.expect("failed to get table name").name.eq(table_name) {
            return Ok(true);
        }
    }
    Ok(false)
}
// get session list.
pub fn get_session_list(conn: &Connection) -> Result<Vec<model::Session>> {
    let table_name = "session";
    let mut res: Vec<model::Session> = Vec::new();
    if table_exist(conn, table_name)? {
        // get all session
        let mut statement = conn.prepare("SELECT * FROM (?1)")?;
        let session_list = statement.query_map([table_name], |row| {
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

pub fn get_group_list(conn: &Connection) -> Result<Vec<model::Group>> {
    let table_name = "group";
    let mut res: Vec<model::Group> = Vec::new();
    if table_exist(conn, table_name)? {
        // get all group.
        let mut statement = conn.prepare("SELECT * FROM (?1)")?;
        let group_list = statement.query_map([table_name], |row| {
            Ok(model::Group {
                id: row.get(0)?,
                name: row.get(1)?,
                avatar: row.get(2)?,
                desc: row.get(3)?,
                ownerid:row.get(4)?,
            })
        })?;
        for group in group_list {
            res.push(group.expect("failed to get group"));
        }
        Ok(res)
    } else {
        let mut statement = conn.prepare("");
        let sql = "";
        conn.execute(sql, [""])?;
        Ok(res)
    }
}

pub fn get_friend_list(conn: &Connection) -> Result<Vec<model::Friend>> {
    let table_name = "friend";
    let mut res: Vec<model::Friend> = Vec::new();
    if table_exist(conn, table_name)? {
        // get all friend.
        let mut statement = conn.prepare("SELECT * FROM (?1)")?;
        let friend_list = statement.query_map([table_name], |row| {
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
        Ok(res)
    }
}


pub fn get_friend(conn: &Connection, id: usize)-> Result<model::Friend> {
    let table_name = "friend";
    if table_exist(conn, table_name)? {
        Ok(get_only_friend(conn, id)?)
    }else {
        create_friend_table(conn)?;
        // TODO get friends from backend.
        Ok(get_only_friend(conn, id)?)
    }
}
// get only friend by id.
fn get_only_friend(conn: &Connection, id: usize)-> Result<model::Friend> {
    let table_name = "friend";
    let sql = "SELECT * FROM (?1) WHERE id = ?";
    let friend = conn.query_row(&sql,&[&id] ,|row| {
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

pub fn get_group(conn: &Connection, id: usize)-> Result<model::Group> {
    let table_name = "group";
    if table_exist(conn, table_name)? {
        Ok(get_only_group(conn, id)?)
    }else {
        create_group_table(conn)?;
        // TODO get groups from backend.
        Ok(get_only_group(conn, id)?)
    }
}
fn get_only_group(conn: &Connection, id: usize)-> Result<model::Group> {
    let table_name = "group";
    let sql = "SELECT * FROM (?1) WHERE id = ?";
    let group = conn.query_row(&sql,&[&id] ,|row| {
        Ok(model::Group {
            id: row.get(0)?,
            name: row.get(1)?,
            desc: row.get(2)?,
            avatar: row.get(3)?,
            ownerid:row.get(4)?,
        })
    });
    Ok(group.expect("failed to get group"))
}

fn create_session_table(conn: &Connection) -> Result<()> {
    let sql = "CREATE TABLE Session (
        id INT UNSIGNED  PRIMARY KEY,
        avatar VARCHAR(255),
        last VARCHAR(255),
        last_time BIGINT UNSIGNED,
        un_read INT UNSIGNED,
        name VARCHAR(255),
        remark TEXT,
        session_type INT
    );";
    conn.execute(sql, [])?;
    Ok(())
}
fn create_friend_table(conn: &Connection) -> Result<()> {
    let sql = "CREATE TABLE Friend (
        id INT UNSIGNED  PRIMARY KEY,
        name VARCHAR(255) NOT NULL,
        remark TEXT,
        avatar VARCHAR(255),
        email VARCHAR(255),
        phone VARCHAR(20),
        gender BOOLEAN,
        line VARCHAR(255)
    );";
    conn.execute(sql, [])?;
    Ok(())
}
fn create_group_table(conn: &Connection) -> Result<()> {
    let sql = "CREATE TABLE IF NOT EXISTS groups (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        avatar TEXT,
        desc TEXT,
        ownerid INTEGER
    );";
    conn.execute(sql, [])?;
    Ok(())
}