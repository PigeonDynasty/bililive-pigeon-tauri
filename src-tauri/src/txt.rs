use crate::doc_dir;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

pub fn room_path(room_id: &i32) -> PathBuf {
    let mut path = doc_dir();
    path.push(room_id.to_string());
    path
}
pub fn room_date_path(room_id: &i32, date: &String) -> PathBuf {
    let mut path = room_path(room_id);
    path.push(date.to_owned() + ".txt");
    path
}

pub fn write_danmaku_txt(room_id: i32, date: String, data: Vec<String>) {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(room_date_path(&room_id, &date))
        .unwrap();
    let mut bw = BufWriter::new(file);
    for str in data {
        bw.write(("".to_owned() + &str + "\r\n").as_bytes())
            .expect(&("write ".to_owned() + &str + " failed"));
    }
    bw.flush()
        .expect(&("write ".to_owned() + &room_id.to_string() + " " + &date + " failed"));
}
