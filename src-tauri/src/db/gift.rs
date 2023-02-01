use super::connect;

pub struct DbGift {
    pub _id: u64,
    pub room_id: u32,
    pub timestamp: u64,
    pub uid: String,
    pub uname: String,
    pub name: String,
    pub num: u64,
    pub coin_type: String,
}
pub fn select_by_roomid(room_id: u32) -> Vec<DbGift> {
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
                coin_type: row.get("coin_type").unwrap(),
            })
        })
        .unwrap();
    for db_gift in db_gift_iter {
        res.push(db_gift.unwrap());
    }
    res
}

pub fn select_by_timestamp(room_id: u32, start_timestamp: u32, end_timestamp: u32) -> Vec<DbGift> {
    let mut res = vec![];
    let conn = connect();
    let sql: &str = "SELECT * FROM gift WHERE room_id = ?1 AND timestamp >= ?2 AND timestamp <= ?3";
    let mut stmt = conn.prepare(sql).unwrap();
    let db_gift_iter = stmt
        .query_map([room_id, start_timestamp, end_timestamp], |row| {
            Ok(DbGift {
                _id: row.get("_id").unwrap(),
                timestamp: row.get("timestamp").unwrap(),
                room_id: row.get("room_id").unwrap(),
                uid: row.get("uid").unwrap(),
                uname: row.get("uname").unwrap(),
                name: row.get("name").unwrap(),
                num: row.get("num").unwrap(),
                coin_type: row.get("coin_type").unwrap(),
            })
        })
        .unwrap();
    for db_gift in db_gift_iter {
        res.push(db_gift.unwrap());
    }
    res
}

pub fn update(
    room_id: &u32,
    timestamp: u64,
    uid: &str,
    uname: &str,
    name: &str,
    num: u64,
    coin_type: &str,
) {
    let conn = connect();
    let select_sql: &str = "SELECT num FROM gift WHERE room_id=?1 AND timestamp=?2 AND uid=?3 AND name=?4 AND coin_type=?5";
    let mut stmt = conn.prepare(select_sql).unwrap();
    let count = match stmt.query_row(
        [
            room_id.to_string().as_str(),
            (timestamp / 10).to_string().as_str(),
            uid,
            name,
            coin_type,
        ],
        |row| row.get(0) as Result<u64, _>,
    ) {
        Ok(num) => num,
        Err(_e) => 0,
    };
    if count > 0 {
        stmt = conn
            .prepare("UPDATE gift SET uname=?1, num=?2 WHERE room_id=?3 AND timestamp=?4 AND uid=?5 AND name=?6 AND coin_type=?7")
            .unwrap();
        stmt.execute([
            uname,
            (count + num).to_string().as_str(),
            room_id.to_string().as_str(),
            (timestamp / 10).to_string().as_str(),
            uid,
            name,
            coin_type,
        ])
        .unwrap();
    } else {
        stmt = conn
        .prepare("INSERT INTO gift (room_id, timestamp, uid, uname, name, num, coin_type) VALUES (?1,?2,?3,?4,?5,?6,?7)")
        .unwrap();
        stmt.execute([
            room_id.to_string(),
            (timestamp / 10).to_string(),
            uid.to_string(),
            uname.to_owned(),
            name.to_owned(),
            num.to_string(),
            coin_type.to_owned(),
        ])
        .unwrap();
    }
}
