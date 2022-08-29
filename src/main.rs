mod connections;
mod util;

use crate::connections::Connection;
use crate::util::{get_id, stamp_header};

use std::io::{stdin, stdout, Write};
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Duration;

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

        sleep(Duration::from_millis(200));
        let response = conn.try_read().unwrap();
        println!("\n{:?}", response);
        println!("\n{}", String::from_utf8_lossy(&response));
    }
}

#[cfg(test)]
mod bite_tests {
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

        conn.try_write(stamp_header(b"s kv.1 ONE".to_vec(), id, 0))
            .unwrap();
        conn.try_write(stamp_header(b"s kv.1.2 TWO".to_vec(), id, 0))
            .unwrap();
        conn.try_write(stamp_header(b"s kv.1.2.3 THREE".to_vec(), id, 0))
            .unwrap();
        conn.try_write(stamp_header(b"k kv".to_vec(), id, 0))
            .unwrap();

        let response = get_read(&mut conn);

        assert_eq!(
            response,
            &[
                0, id as u8, 0, 0, 0, 8, 79, 75, 0, id as u8, 0, 0, 0, 8, 79, 75, 0, id as u8, 0,
                0, 0, 8, 79, 75, 0, id as u8, 0, 0, 0, 25, 49, 32, 79, 78, 69, 0, 50, 32, 84, 87,
                79, 0, 51, 32, 84, 72, 82, 69, 69
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
                0, 0, 8, 79, 75, 0, id as u8, 0, 0, 0, 8, 79, 75, 0, id as u8, 0, 0, 0, 131, 123,
                34, 106, 115, 34, 58, 123, 34, 49, 34, 58, 91, 52, 57, 93, 44, 34, 50, 34, 58, 91,
                53, 48, 93, 44, 34, 51, 34, 58, 123, 34, 49, 34, 58, 91, 53, 49, 44, 52, 54, 44,
                52, 57, 93, 44, 34, 50, 34, 58, 91, 53, 49, 44, 52, 54, 44, 53, 48, 93, 125, 125,
                44, 34, 106, 115, 111, 110, 34, 58, 123, 34, 49, 34, 58, 91, 52, 57, 93, 44, 34,
                50, 34, 58, 91, 53, 48, 93, 44, 34, 51, 34, 58, 123, 34, 49, 34, 58, 91, 53, 49,
                44, 52, 54, 44, 52, 57, 93, 44, 34, 50, 34, 58, 91, 53, 49, 44, 52, 54, 44, 53, 48,
                93, 125, 125, 125
            ]
        );
    }

    // #[test]
    // fn subscriptions() {
    //     let server = TcpStream::connect("127.0.0.1:1984").unwrap();
    //     server.set_nonblocking(true).unwrap();

    //     let addr = server.local_addr().unwrap();
    //     let mut conn = Connection::new(0, server, addr);

    //     // This test fails when key_value has more children than expected. We
    //     // are assuming an empty database.

    //     conn.try_write(b"#g subs".to_vec()).unwrap();
    //     conn.try_write(b"#j subs".to_vec()).unwrap();
    //     conn.try_write(b"#k subs".to_vec()).unwrap();

    //     conn.try_write(b"s subs.user User".to_vec())
    //         .unwrap();
    //     conn.try_write(b"d subs.city".to_vec()).unwrap();
    //     conn.try_write(b"+ subs.city City".to_vec())
    //         .unwrap();
    //     conn.try_write(b"d subs.age".to_vec()).unwrap();
    //     conn.try_write(b"+1 subs.age Age".to_vec()).unwrap();
    //     conn.try_write(b"d subs.user.id".to_vec()).unwrap();
    //     conn.try_write(b"+1 subs.user.id User.ID".to_vec())
    //         .unwrap();
    //     conn.try_write(b"! subs.user.name User.Name".to_vec())
    //         .unwrap();
    //     conn.try_write(b"! subs.user.name.first User.Name.First".to_vec())
    //         .unwrap();
    //     conn.try_write(b"! subs.user.name.last User.Name.Last".to_vec())
    //         .unwrap();

    //     conn.try_write(b"#- subs".to_vec()).unwrap();

    //     conn.try_write(b"s subs.user User".to_vec())
    //         .unwrap();
    //     conn.try_write(b"d subs.city".to_vec()).unwrap();
    //     conn.try_write(b"+ subs.city City".to_vec())
    //         .unwrap();
    //     conn.try_write(b"d subs.age".to_vec()).unwrap();
    //     conn.try_write(b"+1 subs.age Age".to_vec()).unwrap();

    //     sleep(Duration::from_millis(200));
    //     let response = conn.try_read().unwrap();
    //     println!("{:?}\n", response);
    //     println!("{}", String::from_utf8_lossy(&response));
    //     sleep(Duration::from_millis(200));

    //     assert_eq!(
    //         response,
    //         &[
    //             79, 75, 10, 79, 75, 10, 79, 75, 10, 79, 75, 10, 85, 115, 101, 114, 10, 123, 34,
    //             117, 115, 101, 114, 34, 58, 34, 85, 115, 101, 114, 34, 125, 10, 117, 115, 101, 114,
    //             32, 85, 115, 101, 114, 10, 79, 75, 10, 67, 105, 116, 121, 10, 67, 105, 116, 121,
    //             10, 123, 34, 99, 105, 116, 121, 34, 58, 34, 67, 105, 116, 121, 34, 125, 10, 99,
    //             105, 116, 121, 32, 67, 105, 116, 121, 10, 79, 75, 10, 0, 0, 0, 0, 0, 0, 0, 0, 10,
    //             0, 0, 0, 0, 0, 0, 0, 0, 10, 123, 34, 97, 103, 101, 34, 58, 34, 92, 117, 48, 48, 48,
    //             48, 92, 117, 48, 48, 48, 48, 92, 117, 48, 48, 48, 48, 92, 117, 48, 48, 48, 48, 92,
    //             117, 48, 48, 48, 48, 92, 117, 48, 48, 48, 48, 92, 117, 48, 48, 48, 48, 92, 117, 48,
    //             48, 48, 48, 34, 125, 10, 97, 103, 101, 32, 0, 0, 0, 0, 0, 0, 0, 0, 10, 79, 75, 10,
    //             0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 10, 123, 34, 105, 100, 34, 58,
    //             34, 92, 117, 48, 48, 48, 48, 92, 117, 48, 48, 48, 48, 92, 117, 48, 48, 48, 48, 92,
    //             117, 48, 48, 48, 48, 92, 117, 48, 48, 48, 48, 92, 117, 48, 48, 48, 48, 92, 117, 48,
    //             48, 48, 48, 92, 117, 48, 48, 48, 48, 34, 125, 10, 105, 100, 32, 0, 0, 0, 0, 0, 0,
    //             0, 0, 10, 79, 75, 10, 85, 115, 101, 114, 46, 78, 97, 109, 101, 10, 123, 34, 110,
    //             97, 109, 101, 34, 58, 34, 85, 115, 101, 114, 46, 78, 97, 109, 101, 34, 125, 10,
    //             110, 97, 109, 101, 32, 85, 115, 101, 114, 46, 78, 97, 109, 101, 10, 79, 75, 10, 85,
    //             115, 101, 114, 46, 78, 97, 109, 101, 46, 70, 105, 114, 115, 116, 10, 123, 34, 102,
    //             105, 114, 115, 116, 34, 58, 34, 85, 115, 101, 114, 46, 78, 97, 109, 101, 46, 70,
    //             105, 114, 115, 116, 34, 125, 10, 102, 105, 114, 115, 116, 32, 85, 115, 101, 114,
    //             46, 78, 97, 109, 101, 46, 70, 105, 114, 115, 116, 10, 79, 75, 10, 85, 115, 101,
    //             114, 46, 78, 97, 109, 101, 46, 76, 97, 115, 116, 10, 123, 34, 108, 97, 115, 116,
    //             34, 58, 34, 85, 115, 101, 114, 46, 78, 97, 109, 101, 46, 76, 97, 115, 116, 34, 125,
    //             10, 108, 97, 115, 116, 32, 85, 115, 101, 114, 46, 78, 97, 109, 101, 46, 76, 97,
    //             115, 116, 10, 79, 75, 10, 79, 75, 10, 79, 75, 10, 67, 105, 116, 121, 10, 79, 75,
    //             10, 0, 0, 0, 0, 0, 0, 0, 0, 10
    //         ]
    //     );
    // }
}
