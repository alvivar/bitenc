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
    sleep(Duration::from_millis(250));
    let response = conn.try_read().unwrap();
    let id = (response[0] as u32) << 8 | response[1] as u32;
    sleep(Duration::from_millis(250));

    id
}

pub fn get_read(conn: &mut Connection) -> Option<Vec<u8>> {
    sleep(Duration::from_millis(250));
    let response = conn.try_read();

    match response {
        Ok(data) => {
            println!("{:?}\n", data);
            println!("{}", String::from_utf8_lossy(&data));
            sleep(Duration::from_millis(250));

            Some(data)
        }

        Err(_) => None,
    }
}
