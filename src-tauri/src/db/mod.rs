use crate::db_path;
use rusqlite::Connection;
use std::fs::OpenOptions;
pub mod gift;
pub mod history;
pub mod plugin;

// 连接数据库
pub fn connect() -> Connection {
    let db_path = db_path();
    Connection::open(db_path).unwrap()
}
// 初始化数据库
pub fn init() {
    let _file = OpenOptions::new().create(true).open(db_path());
    let conn = connect();
    // 创建表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS plugin (
            _id INTEGER PRIMARY KEY AUTOINCREMENT,
            path TEXT NOT NULL,
            visible INTEGER
        )",
        (), // empty list of parameters.
    )
    .unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS gift (
            _id INTEGER PRIMARY KEY AUTOINCREMENT,
            room_id INTEGER,
            timestamp INTEGER,
            uid TEXT NOT NULL,
            uname TEXT NOT NULL,
            name TEXT NOT NULL,
            num INTEGER
        )",
        (), // empty list of parameters.
    )
    .unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS history (
            _id INTEGER PRIMARY KEY AUTOINCREMENT,
            room_id INTEGER,
            uid TEXT NOT NULL,
            uname TEXT NOT NULL,
            timestamp INTEGER
        )",
        (), // empty list of parameters.
    )
    .unwrap();
    // 中途新增字段
    // if !check_table_column_existed("plugin", "status", &conn) {
    //     conn.execute(
    //         "ALTER TABLE plugin ADD COLUMN status INTEGER",
    //         (), // empty list of parameters.
    //     )
    //     .unwrap();
    // }
    conn.close().unwrap();
}
// 检查数据表是否创建
// fn check_table_existed(table_name: &str, conn: &Connection) -> bool {
//     let sql: &str = "SELECT COUNT(name) FROM sqlite_master WHERE type = 'table' AND name=?";
//     let mut stmt = conn.prepare(sql).unwrap();
//     let count = stmt
//         .query_row([table_name], |row| row.get(0) as Result<u32>)
//         .unwrap();
//     count > 0
// }
// 检查是否存在表字段
// fn check_table_column_existed(table_name: &str, column_name: &str, conn: &Connection) -> bool {
//     let sql: &str = "SELECT COUNT(*) FROM sqlite_master WHERE name=? and sql like ?";
//     let mut stmt = conn.prepare(sql).unwrap();
//     let count = stmt
//         .query_row(
//             [table_name, ("%".to_owned() + column_name + "%").as_ref()],
//             |row| row.get(0) as Result<u32>,
//         )
//         .unwrap();
//     count > 0
// }
