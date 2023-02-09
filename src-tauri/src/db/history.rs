use super::connect;
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};
#[derive(Serialize)]
pub struct DbHistory {
    pub _id: u32,
    pub room_id: u32,
    pub uid: String,
    pub uname: String,
    pub timestamp: u64,
}

pub fn select_all() -> Vec<DbHistory> {
    let mut res = vec![];
    let conn = connect();
    let mut stmt = conn
        .prepare("SELECT * FROM history ORDER BY timestamp DESC")
        .unwrap();
    let db_history_iter = stmt
        .query_map([], |row| {
            Ok(DbHistory {
                _id: row.get("_id").unwrap(),
                room_id: row.get("room_id").unwrap(),
                uid: row.get("uid").unwrap(),
                uname: row.get("uname").unwrap(),
                timestamp: row.get("timestamp").unwrap(),
            })
        })
        .unwrap();
    for db_history in db_history_iter {
        res.push(db_history.unwrap());
    }
    res
}

pub fn update(room_id: u32, uid: &str, uname: &str) {
    let conn = connect();
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let mut stmt = conn
        .prepare("SELECT COUNT(room_id) FROM history WHERE room_id = ?1")
        .unwrap();
    let count = stmt
        .query_row([room_id], |row| row.get(0) as Result<u8, _>)
        .unwrap();
    if count > 0 {
        // 有则更新
        stmt = conn
            .prepare("UPDATE history SET uid=?1, uname=?2, timestamp=?3 WHERE room_id = ?4")
            .unwrap();
        stmt.execute([
            uid.to_owned(),
            uname.to_owned(),
            time.to_string(),
            room_id.to_string(),
        ])
        .unwrap();
    } else {
        // 无则插入
        stmt = conn
            .prepare("INSERT INTO history (room_id, uid, uname,timestamp) VALUES (?1, ?2, ?3, ?4)")
            .unwrap();
        stmt.execute([
            room_id.to_string(),
            uid.to_owned(),
            uname.to_owned(),
            time.to_string(),
        ])
        .unwrap();
    }
}
