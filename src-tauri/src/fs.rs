use std::error::Error;
use std::fs::{metadata, read_dir};
use std::path::PathBuf;
// 读取目录下文件
pub fn read_all_path(root_path: &PathBuf) -> Result<Vec<String>, Box<dyn Error>> {
    let mut path_list = vec![root_path
        .as_path()
        .as_os_str()
        .to_str()
        .unwrap()
        .to_string()];
    let mut start_index = 0;
    loop {
        let list_len = path_list.len();
        for index in start_index..path_list.len() {
            let path = &path_list[index];
            if metadata(path)?.is_dir() {
                for child_dir in read_dir(&path)? {
                    path_list.push(String::from(
                        child_dir?.path().as_os_str().to_str().expect(""),
                    ));
                }
            }
        }
        if list_len == start_index {
            break;
        }
        start_index = list_len;
    }
    return Ok(path_list);
}
