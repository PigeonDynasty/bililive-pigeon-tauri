use super::connect;

#[derive(Debug)]
#[allow(dead_code)]
pub struct DbPlugin {
    id: i32,
    pub path: String,
    pub visible: i8,
}
pub fn select_all() -> Vec<DbPlugin> {
    let mut res = vec![];
    let conn = connect();
    let sql: &str = "SELECT * FROM plugin";
    let mut stmt = conn.prepare(sql).unwrap();
    let db_plugin_iter = stmt
        .query_map([], |row| {
            Ok(DbPlugin {
                id: row.get("id").unwrap(),
                path: row.get("path").unwrap(),
                visible: row.get("visible").unwrap(),
            })
        })
        .unwrap();
    for db_plugin in db_plugin_iter {
        println!("Found DbPlugin {:?}", &db_plugin);
        res.push(db_plugin.unwrap());
    }
    res
}
pub fn insert(data: &Vec<DbPlugin>) {
    let conn = connect();
    let mut stmt = conn
        .prepare("INSERT INTO plugin (path, visible) VALUES (?1,?2)")
        .unwrap();
    for item in data {
        stmt.execute([&item.path, &item.visible.to_string()])
            .unwrap();
    }
}
pub fn update_visible(path: &str, visible: &i8) {
    let conn = connect();
    let mut stmt = conn
        .prepare("UPDATE plugin SET visible=?1 WHERE path=?2")
        .unwrap();
    stmt.execute([visible.to_string(), path.to_string()])
        .unwrap();
}

pub fn delete(paths: Vec<String>) {
    let conn = connect();
    let mut stmt = conn.prepare("DELETE FROM plugin WHERE path=?").unwrap();
    for path in paths {
        stmt.execute([&path]).unwrap();
    }
}
