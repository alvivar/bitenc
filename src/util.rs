use std::{thread::sleep, time::Duration};

use crate::connections::Connection;

fn get_header(from: u32, id: u32, size: u32) -> [u8; 6] {
    let byte0 = ((from & 0xFF00) >> 8) as u8;
    let byte1 = (from & 0x00FF) as u8;

    let byte2 = ((id & 0xFF00) >> 8) as u8;
    let byte3 = (id & 0x00FF) as u8;

    let byte4 = ((size & 0xFF00) >> 8) as u8;
    let byte5 = (size & 0x00FF) as u8;

    [byte0, byte1, byte2, byte3, byte4, byte5]
}

pub fn stamp_header(mut bytes: Vec<u8>, from: u32, id: u32) -> Vec<u8> {
    let size = (bytes.len() + 6) as u32;
    bytes.splice(0..0, get_header(from, id, size));
    bytes
}

pub fn get_id(conn: &mut Connection) -> u32 {
    sleep(Duration::from_millis(200));
    let response = conn.try_read().unwrap();
    let id = u64::from_be_bytes(response[6..14].try_into().unwrap()) as u32;
    sleep(Duration::from_millis(200));

    id
}