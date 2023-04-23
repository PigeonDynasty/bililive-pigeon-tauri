use super::connect;
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};
#[derive(Serialize, Debug)]
pub struct DbSetting {
    pub _id: u32,
    pub room_id: u32,
    pub config: String,
    pub timestamp: u64,
}

pub fn select_by_roomid(room_id: u32) -> Option<DbSetting> {
    let conn = connect();
    let sql: &str = "SELECT * FROM setting WHERE room_id = ?1 LIMIT 1";
    let mut stmt = conn.prepare(sql).unwrap();
    let result = stmt.query_row([room_id], |row| {
        Ok(DbSetting {
            _id: row.get("_id")?,
            room_id: row.get("room_id")?,
            config: row.get("config")?,
            timestamp: row.get("timestamp")?, // add other columns here
        })
    });
    match result {
        Ok(row) => Some(row),
        Err(_) => None,
    }
}

pub fn update(room_id: u32, config: &str) {
    let conn = connect();
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let mut stmt = conn
        .prepare("SELECT COUNT(room_id) FROM setting WHERE room_id = ?1")
        .unwrap();
    let count = stmt
        .query_row([room_id], |row| row.get(0) as Result<u8, _>)
        .unwrap();
    if count > 0 {
        // 有则更新
        stmt = conn
            .prepare("UPDATE setting SET config=?1, timestamp=?2 WHERE room_id = ?3")
            .unwrap();
        stmt.execute([config.to_owned(), time.to_string(), room_id.to_string()])
            .unwrap();
    } else {
        // 无则插入
        stmt = conn
            .prepare("INSERT INTO setting (room_id, config,timestamp) VALUES (?1, ?2, ?3)")
            .unwrap();
        stmt.execute([room_id.to_string(), config.to_owned(), time.to_string()])
            .unwrap();
    }
}
