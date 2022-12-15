use super::connect;

pub struct DbGift {
    pub _id: u64,
    pub timestamp: u64,
    pub room_id: u64,
    pub uid: String,
    pub uname: String,
    pub name: String,
    pub num: u64,
}

pub fn select_by_roomid(room_id: u8) -> Vec<DbGift> {
    let mut res = vec![];
    let conn = connect();
    let sql: &str = "SELECT * FROM gift WHERE roomid = ?1";
    let mut stmt = conn.prepare(sql).unwrap();
    let db_gift_iter = stmt
        .query_map([room_id], |row| {
            Ok(DbGift {
                _id: row.get("_id").unwrap(),
                timestamp: row.get("timestamp").unwrap(),
                room_id: row.get("room_id").unwrap(),
                uid: row.get("uid").unwrap(),
                uname: row.get("uname").unwrap(),
                name: row.get("name").unwrap(),
                num: row.get("num").unwrap(),
            })
        })
        .unwrap();
    for db_gift in db_gift_iter {
        res.push(db_gift.unwrap());
    }
    res
}
