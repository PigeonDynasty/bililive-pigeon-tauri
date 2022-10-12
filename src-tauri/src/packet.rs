#![allow(unused)]
use miniz_oxide::inflate;
use serde::Serialize;
use serde_json::Value;
use std::convert::TryInto;
use std::io::{Cursor, Read};
#[derive(Debug, Clone, Serialize)]
pub struct Packet {
    // pub packet_len: i32,
    // pub header_len: i32,
    pub ver: u16,
    pub op: u32,
    // pub seq: i32,
    pub body: Value,
}
impl Packet {}

// 设置值
fn set_int(mut buffer: Vec<u8>, start: i32, len: i32, value: i32) -> Vec<u8> {
    for i in start..start + len {
        buffer[i as usize] = (value / 256i32.pow((start + len - i - 1) as u32)) as u8;
    }
    buffer
}
// 编码
pub fn encode(raw: &str, op: i32) -> Vec<u8> {
    let header_len = 16;
    let packet_len = raw.len() as i32 + header_len;
    let data = raw.as_bytes();
    let header = [0, 0, 0, 0, 0, 16, 0, 1, 0, 0, 0, op as u8, 0, 0, 0, 1];
    let mut packet = set_int(header.to_vec(), 0, 4, packet_len);
    packet.extend(data);
    packet
}
// 解码数据包
pub fn decode(buffer_all: Vec<u8>) -> Vec<Packet> {
    let buffer_vec = spilt_buffer(&buffer_all).unwrap();
    let mut result = Vec::new();
    for buffer in buffer_vec {
        let mut packet = parse_buffer(buffer).unwrap();
        result.append(&mut packet);
    }
    result
}
// 分割数据包
fn spilt_buffer<'a>(bytes: &'a [u8]) -> Result<Vec<&'a [u8]>, ()> {
    let mut frames: Vec<&[u8]> = vec![];
    let total_len = bytes.len();
    let mut offset_start;
    let mut offset_end = 0usize;
    loop {
        if offset_end >= total_len {
            break;
        }
        offset_start = offset_end;

        let len_bytes = &bytes[offset_start..offset_start + 4];
        let len = u32::from_be_bytes(len_bytes.try_into().unwrap()) as usize;
        if len < 16 {
            break;
        }

        offset_end = offset_start + len;
        let frame = &bytes[offset_start..offset_end];
        frames.push(frame);
    }

    Ok(frames)
}
// 处理单个数据包
fn parse_buffer(buffer: &[u8]) -> Result<Vec<Packet>, ()> {
    let ver = u16::from_be_bytes((&buffer[6..8]).try_into().unwrap());
    let op = u32::from_be_bytes((&buffer[8..12]).try_into().unwrap());
    let payload = &buffer[16..];
    let result = match ver {
        0 => {
            if op == 5 {
                let val = serde_json::from_slice(payload).unwrap();
                vec![Packet {
                    ver,
                    op,
                    body: serde_json::from_value(val).unwrap(),
                }]
            } else {
                vec![]
            }
        }
        1 => {
            // 气人值
            let count = u32::from_be_bytes((&buffer[16..20]).try_into().unwrap());
            vec![Packet {
                ver,
                op,
                body: serde_json::json!({ "count": count }),
            }]
        }
        2 => {
            // zlib压缩的数据
            let msg = inflate::decompress_to_vec_zlib(payload).expect("Failed to decompress!");
            decode(msg)
        }
        3 => {
            // Brotli压缩的数据
            if op == 5 {
                let read_stream = Cursor::new(payload);
                let mut input = brotli::Decompressor::new(read_stream, 4096);
                let mut body = Vec::new();
                match input.read_to_end(&mut body) {
                    Ok(_size) => decode(body),
                    Err(e) => {
                        println!("BrotliError={:?}", e);
                        vec![]
                    }
                }
            } else {
                vec![]
            }
        }
        _ => {
            println!("错误的校验码：{}", ver);
            vec![]
        }
    };
    Ok(result)
}
