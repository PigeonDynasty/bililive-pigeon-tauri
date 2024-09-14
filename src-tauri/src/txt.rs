use crate::doc_dir;
use std::fs::{create_dir_all, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

pub fn room_path(room_id: &u32) -> PathBuf {
    let mut path = doc_dir();
    path.push(room_id.to_string());
    path
}
pub fn room_date_path(room_id: &u32, date: &String) -> PathBuf {
    let mut path = room_path(room_id);
    path.push(date.to_owned() + ".txt");
    path
}

pub fn write_danmaku_txt(room_id: u32, date: String, data: Vec<String>) {
    let path: PathBuf = room_date_path(&room_id, &date);
    let dir = path.parent().unwrap(); // Get the parent directory
    if !dir.exists() {
        create_dir_all(dir).expect("Failed to create the directory"); // Create the directory if it doesn't exist
    }
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .expect("Failed to open or create the file");
    let mut bw = BufWriter::new(file);
    for str in data {
        bw.write(("".to_owned() + &str + "\r\n").as_bytes())
            .expect(&("write ".to_owned() + &str + " failed"));
    }
    bw.flush()
        .expect(&("write ".to_owned() + &room_id.to_string() + " " + &date + " failed"));
}
