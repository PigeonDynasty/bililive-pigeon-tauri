use super::connect;
use crate::emoji_path;
use serde::Serialize;
use std::fs::File;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
#[derive(Serialize)]
pub struct DbEmoji {
    pub _id: u64,
    pub emoji: String,
    pub emoticon_id: u32,
    pub emoticon_unique: String,
    pub url: String,
    pub timestamp: u64,
}
#[allow(unused_must_use)]
pub fn init() {
    // 文档内没有emoji.json的时候把项目内默认的json移入
    if Path::new(emoji_path().as_os_str().to_str().unwrap()).exists() == false {
        std::fs::copy("emoji.json", emoji_path());
    }
    // 没有数据的情况初始化
    // let conn = connect();
    // let mut stmt = conn.prepare("SELECT COUNT(timestamp) FROM emoji").unwrap();
    // let count = stmt
    //     .query_row([], |row| row.get(0) as Result<u8, _>)
    //     .unwrap();
    // if count <= 0 {
    //     init_data()
    // }

    // 每次启动重新初始化数据
    init_data();
}
pub fn init_data() {
    let conn = connect();
    // 清空表
    let mut stmt = conn.prepare("DELETE FROM emoji").unwrap();
    stmt.execute([]).unwrap();
    // 重新插入
    stmt = conn
        .prepare(
            "INSERT INTO emoji (emoji, emoticon_id, emoticon_unique, url, timestamp) VALUES (?1,?2,?3,?4,?5)",
        )
        .unwrap();
    let json_file = File::open(emoji_path()).unwrap();
    let res: serde_json::Value = serde_json::from_reader(json_file).unwrap();
    let data = res["data"]["data"][0]["emoticons"]
        .as_array()
        .unwrap()
        .to_owned();
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    for item in data {
        stmt.execute([
            item["emoji"].as_str().unwrap(),
            item["emoticon_id"].as_u64().unwrap().to_string().as_str(),
            item["emoticon_unique"].as_str().unwrap(),
            item["url"].as_str().unwrap(),
            time.to_string().as_str(),
        ])
        .unwrap();
    }
}

pub fn select_by_emojis(emojis: Vec<&str>) -> Vec<DbEmoji> {
    let mut res = vec![];
    let conn = connect();
    let mut _emojis = vec![];
    for emoji in emojis {
        _emojis.push(format!("\"{}\"", emoji));
    }
    let sql: &str = &format!(
        "SELECT * FROM emoji WHERE emoji in ({});",
        _emojis.join(",")
    );
    let mut stmt = conn.prepare(sql).unwrap();
    let db_emoji_iter = stmt
        .query_map([], |row| {
            Ok(DbEmoji {
                _id: row.get("_id").unwrap(),
                emoji: row.get("emoji").unwrap(),
                emoticon_id: row.get("emoticon_id").unwrap(),
                emoticon_unique: row.get("emoticon_unique").unwrap(),
                url: row.get("url").unwrap(),
                timestamp: row.get("timestamp").unwrap(),
            })
        })
        .unwrap();
    for db_emoji in db_emoji_iter {
        res.push(db_emoji.unwrap());
    }
    res
}
