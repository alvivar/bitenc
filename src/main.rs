mod connections;
mod util;

use crate::connections::Connection;
use crate::util::{get_id, get_read, stamp_header};

use std::io::{stdin, stdout, Write};
use std::net::TcpStream;

fn main() {
    let server = TcpStream::connect("127.0.0.1:1984").unwrap();
    server.set_nonblocking(true).unwrap();

    let addr = server.local_addr().unwrap();
    let mut conn = Connection::new(0, server, addr);

    let id = get_id(&mut conn);

    loop {
        let mut input = String::new();

        print!("\n> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();

        let message = input.trim().as_bytes().to_vec();
        let message = stamp_header(message, id, 0);
        println!("> {:?}", message);
        if let Ok(count) = conn.try_write(message) {
            println!("{} bytes written", count);
        }

        get_read(&mut conn);
    }
}

#[cfg(test)]
mod bite_tests {
    use rand::{thread_rng, Rng};

    use crate::connections::Connection;
    use crate::util::{get_id, get_read, stamp_header};

    use std::net::TcpStream;

    #[test]
    fn empty_message() {
        let server = TcpStream::connect("127.0.0.1:1984").unwrap();
        server.set_nonblocking(true).unwrap();

        let addr = server.local_addr().unwrap();
        let mut conn = Connection::new(0, server, addr);

        get_id(&mut conn);

        conn.try_write(b"".to_vec()).unwrap();

        let response = get_read(&mut conn);

        assert!(
            response.is_empty(),
            "A message without protocol should get disconnected!"
        );
    }

    #[test]
    fn set() {
        let server = TcpStream::connect("127.0.0.1:1984").unwrap();
        server.set_nonblocking(true).unwrap();

        let addr = server.local_addr().unwrap();
        let mut conn = Connection::new(0, server, addr);

        let id = get_id(&mut conn);

        conn.try_write(stamp_header(b"s set SET".to_vec(), id, 0))
            .unwrap();
        conn.try_write(stamp_header(b"g set".to_vec(), id, 0))
            .unwrap();
        conn.try_write(stamp_header(b"s set".to_vec(), id, 0))
            .unwrap();
        conn.try_write(stamp_header(b"g set".to_vec(), id, 0))
            .unwrap();

        let response = get_read(&mut conn);

        assert_eq!(
            response,
            &[
                0, id as u8, 0, 0, 0, 8, 79, 75, 0, id as u8, 0, 0, 0, 9, 83, 69, 84, 0, id as u8,
                0, 0, 0, 8, 79, 75, 0, id as u8, 0, 0, 0, 6
            ]
        );
    }

    #[test]
    fn set_if_none() {
        let server = TcpStream::connect("127.0.0.1:1984").unwrap();
        server.set_nonblocking(true).unwrap();

        let addr = server.local_addr().unwrap();
        let mut conn = Connection::new(0, server, addr);

        let id = get_id(&mut conn);

        conn.try_write(stamp_header(b"d maybe".to_vec(), id, 0))
            .unwrap();
        conn.try_write(stamp_header(b"s? maybe MAYBE".to_vec(), id, 0))
            .unwrap();
        conn.try_write(stamp_header(b"g maybe".to_vec(), id, 0))
            .unwrap();
        conn.try_write(stamp_header(b"s? maybe NEW".to_vec(), id, 0))
            .unwrap();
        conn.try_write(stamp_header(b"g maybe".to_vec(), id, 0))
            .unwrap();

        let response = get_read(&mut conn);

        assert_eq!(
            response,
            &[
                0, id as u8, 0, 0, 0, 8, 79, 75, 0, id as u8, 0, 0, 0, 8, 79, 75, 0, id as u8, 0,
                0, 0, 11, 77, 65, 89, 66, 69, 0, id as u8, 0, 0, 0, 8, 79, 75, 0, id as u8, 0, 0,
                0, 11, 77, 65, 89, 66, 69
            ]
        );
    }

    #[test]
    fn inc() {
        let server = TcpStream::connect("127.0.0.1:1984").unwrap();
        server.set_nonblocking(true).unwrap();

        let addr = server.local_addr().unwrap();
        let mut conn = Connection::new(0, server, addr);

        let id = get_id(&mut conn);

        conn.try_write(stamp_header(b"d inc".to_vec(), id, 0))
            .unwrap();
        conn.try_write(stamp_header(b"+1 inc".to_vec(), id, 0))
            .unwrap();
        conn.try_write(stamp_header(b"g inc".to_vec(), id, 0))
            .unwrap();
        conn.try_write(stamp_header(b"+1 inc".to_vec(), id, 0))
            .unwrap();
        conn.try_write(stamp_header(b"g inc".to_vec(), id, 0))
            .unwrap();

        let response = get_read(&mut conn);

        assert_eq!(
            response,
            &[
                0, id as u8, 0, 0, 0, 8, 79, 75, 0, id as u8, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 1,
                0, id as u8, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 1, 0, id as u8, 0, 0, 0, 14, 0, 0,
                0, 0, 0, 0, 0, 2, 0, id as u8, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 2
            ]
        );
    }

    #[test]
    fn inc_small_key() {
        let server = TcpStream::connect("127.0.0.1:1984").unwrap();
        server.set_nonblocking(true).unwrap();

        let addr = server.local_addr().unwrap();
        let mut conn = Connection::new(0, server, addr);

        let id = get_id(&mut conn);

        conn.try_write(stamp_header(b"d key".to_vec(), id, 0))
            .unwrap();
        conn.try_write(stamp_header(b"s key 1".to_vec(), id, 0))
            .unwrap();
        conn.try_write(stamp_header(b"+1 key".to_vec(), id, 0))
            .unwrap();

        let response = get_read(&mut conn);

        assert_eq!(
            response,
            &[
                0, id as u8, 0, 0, 0, 8, 79, 75, 0, id as u8, 0, 0, 0, 8, 79, 75, 0, id as u8, 0,
                0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 2
            ]
        );
    }

    #[test]
    fn append() {
        let server = TcpStream::connect("127.0.0.1:1984").unwrap();
        server.set_nonblocking(true).unwrap();

        let addr = server.local_addr().unwrap();
        let mut conn = Connection::new(0, server, addr);

        let id = get_id(&mut conn);

        conn.try_write(stamp_header(b"d append".to_vec(), id, 0))
            .unwrap();
        conn.try_write(stamp_header(b"+ append APP".to_vec(), id, 0))
            .unwrap();
        conn.try_write(stamp_header(b"+ append END".to_vec(), id, 0))
            .unwrap();

        let response = get_read(&mut conn);

        assert_eq!(
            response,
            &[
                0, id as u8, 0, 0, 0, 8, 79, 75, 0, id as u8, 0, 0, 0, 9, 65, 80, 80, 0, id as u8,
                0, 0, 0, 12, 65, 80, 80, 69, 78, 68
            ]
        );
    }

    #[test]
    fn get_delete() {
        let server = TcpStream::connect("127.0.0.1:1984").unwrap();
        server.set_nonblocking(true).unwrap();

        let addr = server.local_addr().unwrap();
        let mut conn = Connection::new(0, server, addr);

        let id = get_id(&mut conn);

        conn.try_write(stamp_header(b"s delete DELETE".to_vec(), id, 0))
            .unwrap();
        conn.try_write(stamp_header(b"d delete".to_vec(), id, 0))
            .unwrap();
        conn.try_write(stamp_header(b"g delete".to_vec(), id, 0))
            .unwrap();

        let response = get_read(&mut conn);

        assert_eq!(
            response,
            &[
                0, id as u8, 0, 0, 0, 8, 79, 75, 0, id as u8, 0, 0, 0, 8, 79, 75, 0, id as u8, 0,
                0, 0, 6
            ]
        );
    }

    #[test]
    fn key_value() {
        let server = TcpStream::connect("127.0.0.1:1984").unwrap();
        server.set_nonblocking(true).unwrap();

        let addr = server.local_addr().unwrap();
        let mut conn = Connection::new(0, server, addr);

        // This test fails when key_value has more children than expected. We
        // are assuming an empty database.

        let id = get_id(&mut conn);

        conn.try_write(stamp_header(b"s kv.1 1".to_vec(), id, 0))
            .unwrap();
        conn.try_write(stamp_header(b"s kv.1.2 1.2".to_vec(), id, 0))
            .unwrap();
        conn.try_write(stamp_header(b"s kv.1.2.3 1.2.3".to_vec(), id, 0))
            .unwrap();
        conn.try_write(stamp_header(b"k kv".to_vec(), id, 0))
            .unwrap();

        let response = get_read(&mut conn);

        assert_eq!(
            response,
            &[
                0, id as u8, 0, 0, 0, 8, 79, 75, 0, id as u8, 0, 0, 0, 8, 79, 75, 0, id as u8, 0,
                0, 0, 8, 79, 75, 0, id as u8, 0, 0, 0, 23, 49, 32, 49, 0, 50, 32, 49, 46, 50, 0,
                51, 32, 49, 46, 50, 46, 51
            ]
        );
    }

    #[test]
    fn json_j() {
        let server = TcpStream::connect("127.0.0.1:1984").unwrap();
        server.set_nonblocking(true).unwrap();

        let addr = server.local_addr().unwrap();
        let mut conn = Connection::new(0, server, addr);

        // This test fails when key_value has more children than expected. We
        // are assuming an empty database.

        let id = get_id(&mut conn);

        conn.try_write(stamp_header(b"s j.1 1".to_vec(), id, 0))
            .unwrap();
        conn.try_write(stamp_header(b"s j.2 2".to_vec(), id, 0))
            .unwrap();
        conn.try_write(stamp_header(b"s j.3.1 3.1".to_vec(), id, 0))
            .unwrap();
        conn.try_write(stamp_header(b"s j.3.2 3.2".to_vec(), id, 0))
            .unwrap();
        conn.try_write(stamp_header(b"j j".to_vec(), id, 0))
            .unwrap();

        let response = get_read(&mut conn);

        assert_eq!(
            response,
            &[
                0, id as u8, 0, 0, 0, 8, 79, 75, 0, id as u8, 0, 0, 0, 8, 79, 75, 0, id as u8, 0,
                0, 0, 8, 79, 75, 0, id as u8, 0, 0, 0, 8, 79, 75, 0, id as u8, 0, 0, 0, 61, 123,
                34, 49, 34, 58, 91, 52, 57, 93, 44, 34, 50, 34, 58, 91, 53, 48, 93, 44, 34, 51, 34,
                58, 123, 34, 49, 34, 58, 91, 53, 49, 44, 52, 54, 44, 52, 57, 93, 44, 34, 50, 34,
                58, 91, 53, 49, 44, 52, 54, 44, 53, 48, 93, 125, 125
            ]
        );
    }

    #[test]
    fn json_js() {
        let server = TcpStream::connect("127.0.0.1:1984").unwrap();
        server.set_nonblocking(true).unwrap();

        let addr = server.local_addr().unwrap();
        let mut conn = Connection::new(0, server, addr);

        // This test fails when key_value has more children than expected. We
        // are assuming an empty database.

        let id = get_id(&mut conn);

        conn.try_write(stamp_header(b"s js.1 1".to_vec(), id, 0))
            .unwrap();
        conn.try_write(stamp_header(b"s js.2 2".to_vec(), id, 0))
            .unwrap();
        conn.try_write(stamp_header(b"s js.3.1 3.1".to_vec(), id, 0))
            .unwrap();
        conn.try_write(stamp_header(b"s js.3.2 3.2".to_vec(), id, 0))
            .unwrap();
        conn.try_write(stamp_header(b"js js".to_vec(), id, 0))
            .unwrap();

        let response = get_read(&mut conn);

        assert_eq!(
            response,
            &[
                0, id as u8, 0, 0, 0, 8, 79, 75, 0, id as u8, 0, 0, 0, 8, 79, 75, 0, id as u8, 0,
                0, 0, 8, 79, 75, 0, id as u8, 0, 0, 0, 8, 79, 75, 0, id as u8, 0, 0, 0, 68, 123,
                34, 106, 115, 34, 58, 123, 34, 49, 34, 58, 91, 52, 57, 93, 44, 34, 50, 34, 58, 91,
                53, 48, 93, 44, 34, 51, 34, 58, 123, 34, 49, 34, 58, 91, 53, 49, 44, 52, 54, 44,
                52, 57, 93, 44, 34, 50, 34, 58, 91, 53, 49, 44, 52, 54, 44, 53, 48, 93, 125, 125,
                125
            ]
        );
    }

    #[test]
    fn subscriptions() {
        let server = TcpStream::connect("127.0.0.1:1984").unwrap();
        server.set_nonblocking(true).unwrap();

        let addr = server.local_addr().unwrap();
        let mut conn = Connection::new(0, server, addr);

        // This test fails when key_value has more children than expected. We
        // are assuming an empty database.

        let id = get_id(&mut conn);

        conn.try_write(stamp_header(b"#g subs".to_vec(), id, 0))
            .unwrap();
        conn.try_write(stamp_header(b"#k subs".to_vec(), id, 0))
            .unwrap();
        conn.try_write(stamp_header(b"#j subs".to_vec(), id, 0))
            .unwrap();

        conn.try_write(stamp_header(b"s subs SET".to_vec(), id, 0))
            .unwrap();
        conn.try_write(stamp_header(b"+1 subs".to_vec(), id, 0))
            .unwrap();
        conn.try_write(stamp_header(b"+ subs +".to_vec(), id, 0))
            .unwrap();
        conn.try_write(stamp_header(b"! subs CALL".to_vec(), id, 0))
            .unwrap();

        let response = get_read(&mut conn);

        assert_eq!(
            response,
            &[
                0, id as u8, 0, 0, 0, 8, 79, 75, 0, id as u8, 0, 0, 0, 8, 79, 75, 0, id as u8, 0,
                0, 0, 8, 79, 75, 0, id as u8, 0, 0, 0, 8, 79, 75, 0, id as u8, 0, 0, 0, 9, 83, 69,
                84, 0, id as u8, 0, 0, 0, 14, 115, 117, 98, 115, 32, 83, 69, 84, 0, id as u8, 0, 0,
                0, 20, 123, 34, 115, 117, 98, 115, 34, 58, 34, 83, 69, 84, 34, 125, 0, id as u8, 0,
                0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 1, 0, id as u8, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 1,
                0, id as u8, 0, 0, 0, 19, 115, 117, 98, 115, 32, 0, 0, 0, 0, 0, 0, 0, 1, 0,
                id as u8, 0, 0, 0, 65, 123, 34, 115, 117, 98, 115, 34, 58, 34, 92, 117, 48, 48, 48,
                48, 92, 117, 48, 48, 48, 48, 92, 117, 48, 48, 48, 48, 92, 117, 48, 48, 48, 48, 92,
                117, 48, 48, 48, 48, 92, 117, 48, 48, 48, 48, 92, 117, 48, 48, 48, 48, 92, 117, 48,
                48, 48, 49, 34, 125, 0, id as u8, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 1, 43, 0,
                id as u8, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 1, 43, 0, id as u8, 0, 0, 0, 20, 115,
                117, 98, 115, 32, 0, 0, 0, 0, 0, 0, 0, 1, 43, 0, id as u8, 0, 0, 0, 66, 123, 34,
                115, 117, 98, 115, 34, 58, 34, 92, 117, 48, 48, 48, 48, 92, 117, 48, 48, 48, 48,
                92, 117, 48, 48, 48, 48, 92, 117, 48, 48, 48, 48, 92, 117, 48, 48, 48, 48, 92, 117,
                48, 48, 48, 48, 92, 117, 48, 48, 48, 48, 92, 117, 48, 48, 48, 49, 43, 34, 125, 0,
                id as u8, 0, 0, 0, 8, 79, 75, 0, id as u8, 0, 0, 0, 10, 67, 65, 76, 76, 0,
                id as u8, 0, 0, 0, 15, 115, 117, 98, 115, 32, 67, 65, 76, 76, 0, id as u8, 0, 0, 0,
                21, 123, 34, 115, 117, 98, 115, 34, 58, 34, 67, 65, 76, 76, 34, 125
            ]
        );
    }

    #[test]
    fn big_message_with_wrong_protocol() {
        let server = TcpStream::connect("127.0.0.1:1984").unwrap();
        server.set_nonblocking(true).unwrap();

        let addr = server.local_addr().unwrap();
        let mut conn = Connection::new(0, server, addr);

        let id = get_id(&mut conn);

        for _ in 0..64 {
            let mut data = [0u8; 65535 - 6];
            thread_rng().try_fill(&mut data[..]).unwrap();

            conn.try_write(stamp_header(data.to_vec(), id, 0)).unwrap();
        }

        let response = get_read(&mut conn);

        let mut expected = Vec::new();
        for _ in 0..64 {
            expected.extend_from_slice(&[0, id as u8, 0, 0, 0, 8, 78, 79]);
        }

        assert_eq!(response, expected);
    }

    #[test]
    fn biggest_sets_128() {
        let server = TcpStream::connect("127.0.0.1:1984").unwrap();
        server.set_nonblocking(true).unwrap();

        let addr = server.local_addr().unwrap();
        let mut conn = Connection::new(0, server, addr);

        let id = get_id(&mut conn);

        let max = 128;

        for i in 0..max {
            let mut data = [0u8; 65535];
            thread_rng().try_fill(&mut data[..]).unwrap();

            let mut set = format!("s set.{} ", i).as_bytes().to_vec();
            set.append(&mut data.to_vec());
            set.truncate(65535 - 6);

            conn.try_write(stamp_header(set, id, 0)).unwrap();
        }

        let response = get_read(&mut conn);

        let mut expected = Vec::new();
        for _ in 0..max {
            expected.extend_from_slice(&[0, id as u8, 0, 0, 0, 8, 79, 75]);
        }

        assert_eq!(response, expected);
    }
}
