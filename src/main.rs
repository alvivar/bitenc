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

        let id = get_id(&mut conn);
        for _ in 0..10 {
            conn.try_write(b"!".to_vec()).unwrap();
            // conn.try_write(stamp_header(b"".to_vec(), id, 0)).unwrap();
        }

        let response = get_read(&mut conn);

        assert_eq!(
            &response,
            &[
                0, 1, 0, 0, 0, 8, 78, 79, 0, 1, 0, 0, 0, 8, 78, 79, 0, 1, 0, 0, 0, 8, 78, 79, 0, 1,
                0, 0, 0, 8, 78, 79, 0, 1, 0, 0, 0, 8, 78, 79, 0, 1, 0, 0, 0, 8, 78, 79, 0, 1, 0, 0,
                0, 8, 78, 79, 0, 1, 0, 0, 0, 8, 78, 79, 0, 1, 0, 0, 0, 8, 78, 79, 0, 1, 0, 0, 0, 8,
                78, 79
            ]
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
                0, 2, 0, 0, 0, 8, 79, 75, 0, 2, 0, 0, 0, 9, 83, 69, 84, 0, 2, 0, 0, 0, 8, 79, 75,
                0, 2, 0, 0, 0, 6
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
                0, 3, 0, 0, 0, 8, 79, 75, 0, 3, 0, 0, 0, 8, 79, 75, 0, 3, 0, 0, 0, 11, 77, 65, 89,
                66, 69, 0, 3, 0, 0, 0, 8, 79, 75, 0, 3, 0, 0, 0, 11, 77, 65, 89, 66, 69
            ]
        );
    }

    // #[test]
    // fn inc() {
    //     let server = TcpStream::connect("127.0.0.1:1984").unwrap();
    //     server.set_nonblocking(true).unwrap();

    //     let addr = server.local_addr().unwrap();
    //     let mut conn = Connection::new(0, server, addr);

    //     conn.try_write(b"d inc".to_vec()).unwrap();
    //     conn.try_write(b"+1 inc".to_vec()).unwrap();
    //     conn.try_write(b"g inc".to_vec()).unwrap();
    //     conn.try_write(b"+1 inc".to_vec()).unwrap();
    //     conn.try_write(b"g inc".to_vec()).unwrap();

    //     sleep(Duration::from_millis(200));
    //     let response = conn.try_read().unwrap();
    //     println!("{:?}\n", response);
    //     println!("{}", String::from_utf8_lossy(&response));
    //     sleep(Duration::from_millis(200));

    //     assert_eq!(
    //         response,
    //         &[
    //             0, 4, 79, 75, 0, 10, 0, 0, 0, 0, 0, 0, 0, 1, 0, 10, 0, 0, 0, 0, 0, 0, 0, 1, 0, 10,
    //             0, 0, 0, 0, 0, 0, 0, 2, 0, 10, 0, 0, 0, 0, 0, 0, 0, 2
    //         ]
    //     );
    // }

    // #[test]
    // fn inc_small_key() {
    //     let server = TcpStream::connect("127.0.0.1:1984").unwrap();
    //     server.set_nonblocking(true).unwrap();

    //     let addr = server.local_addr().unwrap();
    //     let mut conn = Connection::new(0, server, addr);

    //     conn.try_write(b"d key".to_vec()).unwrap();
    //     conn.try_write(b"s key 1".to_vec()).unwrap();
    //     conn.try_write(b"+1 key".to_vec()).unwrap();

    //     conn.try_write(b"d key".to_vec()).unwrap();
    //     conn.try_write(b"+1 key".to_vec()).unwrap();

    //     sleep(Duration::from_millis(200));
    //     let response = conn.try_read().unwrap();
    //     println!("{:?}\n", response);
    //     println!("{}", String::from_utf8_lossy(&response));
    //     sleep(Duration::from_millis(200));

    //     assert_eq!(
    //         response,
    //         &[
    //             0, 4, 79, 75, 0, 4, 79, 75, 0, 10, 0, 0, 0, 0, 0, 0, 0, 2, 0, 4, 79, 75, 0, 10, 0,
    //             0, 0, 0, 0, 0, 0, 1
    //         ]
    //     );
    // }

    // #[test]
    // fn append() {
    //     let server = TcpStream::connect("127.0.0.1:1984").unwrap();
    //     server.set_nonblocking(true).unwrap();

    //     let addr = server.local_addr().unwrap();
    //     let mut conn = Connection::new(0, server, addr);

    //     conn.try_write(b"d append".to_vec()).unwrap();
    //     conn.try_write(b"+ append One".to_vec()).unwrap();
    //     conn.try_write(b"+ append Two".to_vec()).unwrap();

    //     conn.try_write(b"s append Three".to_vec()).unwrap();
    //     conn.try_write(b"+ append Four".to_vec()).unwrap();

    //     sleep(Duration::from_millis(200));
    //     let response = conn.try_read().unwrap();
    //     println!("{:?}\n", response);
    //     println!("{}", String::from_utf8_lossy(&response));
    //     sleep(Duration::from_millis(200));

    //     assert_eq!(
    //         response,
    //         &[
    //             0, 4, 79, 75, 0, 5, 79, 110, 101, 0, 8, 79, 110, 101, 84, 119, 111, 0, 4, 79, 75,
    //             0, 11, 84, 104, 114, 101, 101, 70, 111, 117, 114
    //         ]
    //     );
    // }

    // #[test]
    // fn get_delete() {
    //     let server = TcpStream::connect("127.0.0.1:1984").unwrap();
    //     server.set_nonblocking(true).unwrap();

    //     let addr = server.local_addr().unwrap();
    //     let mut conn = Connection::new(0, server, addr);

    //     conn.try_write(b"d set".to_vec()).unwrap();
    //     conn.try_write(b"d maybe".to_vec()).unwrap();
    //     conn.try_write(b"d inc".to_vec()).unwrap();
    //     conn.try_write(b"d append".to_vec()).unwrap();

    //     conn.try_write(b"s set Set!".to_vec()).unwrap();
    //     conn.try_write(b"s? maybe Maybe!".to_vec()).unwrap();
    //     conn.try_write(b"+1 inc".to_vec()).unwrap();
    //     conn.try_write(b"+ append Append!".to_vec()).unwrap();

    //     conn.try_write(b"g set".to_vec()).unwrap();
    //     conn.try_write(b"g maybe".to_vec()).unwrap();
    //     conn.try_write(b"g inc".to_vec()).unwrap();
    //     conn.try_write(b"g append".to_vec()).unwrap();

    //     conn.try_write(b"d set".to_vec()).unwrap();
    //     conn.try_write(b"d maybe".to_vec()).unwrap();
    //     conn.try_write(b"d inc".to_vec()).unwrap();
    //     conn.try_write(b"d append".to_vec()).unwrap();

    //     conn.try_write(b"g set".to_vec()).unwrap();
    //     conn.try_write(b"g maybe".to_vec()).unwrap();
    //     conn.try_write(b"g inc".to_vec()).unwrap();
    //     conn.try_write(b"g append".to_vec()).unwrap();

    //     sleep(Duration::from_millis(200));
    //     let response = conn.try_read().unwrap();
    //     println!("{:?}\n", response);
    //     println!("{}", String::from_utf8_lossy(&response));
    //     sleep(Duration::from_millis(200));

    //     assert_eq!(
    //         response,
    //         &[
    //             79, 75, 79, 75, 79, 75, 79, 75, 79, 75, 79, 75, 0, 0, 0, 0, 0, 0, 0, 0, 65, 112,
    //             112, 101, 110, 100, 33, 83, 101, 116, 33, 77, 97, 121, 98, 101, 33, 0, 0, 0, 0, 0,
    //             0, 0, 0, 65, 112, 112, 101, 110, 100, 33, 79, 75, 79, 75, 79, 75, 79, 75
    //         ]
    //     );
    // }

    // #[test]
    // fn key_value() {
    //     let server = TcpStream::connect("127.0.0.1:1984").unwrap();
    //     server.set_nonblocking(true).unwrap();

    //     let addr = server.local_addr().unwrap();
    //     let mut conn = Connection::new(0, server, addr);

    //     // This test fails when key_value has more children than expected. We
    //     // are assuming an empty database.

    //     conn.try_write(b"s key_value.1 One".to_vec())
    //         .unwrap();
    //     conn.try_write(b"s key_value.2 Two".to_vec())
    //         .unwrap();
    //     conn.try_write(b"s key_value.3 Three".to_vec())
    //         .unwrap();
    //     conn.try_write(b"s key_value.3.1 Three.One".to_vec())
    //         .unwrap();
    //     conn.try_write(b"s key_value.3.2 Three.Two".to_vec())
    //         .unwrap();
    //     conn.try_write(b"k key_value".to_vec()).unwrap();

    //     sleep(Duration::from_millis(200));
    //     let response = conn.try_read().unwrap();
    //     println!("{:?}\n", response);
    //     println!("{}", String::from_utf8_lossy(&response));
    //     sleep(Duration::from_millis(200));

    //     assert_eq!(
    //         response,
    //         &[
    //             79, 75, 79, 75, 79, 75, 79, 75, 79, 75, 49, 32, 79, 110, 101, 0, 50, 32, 84, 119,
    //             111, 0, 51, 32, 84, 104, 114, 101, 101, 0, 49, 32, 84, 104, 114, 101, 101, 46, 79,
    //             110, 101, 0, 50, 32, 84, 104, 114, 101, 101, 46, 84, 119, 111
    //         ]
    //     );
    // }

    // #[test]
    // fn json() {
    //     let server = TcpStream::connect("127.0.0.1:1984").unwrap();
    //     server.set_nonblocking(true).unwrap();

    //     let addr = server.local_addr().unwrap();
    //     let mut conn = Connection::new(0, server, addr);

    //     // This test fails when key_value has more children than expected. We
    //     // are assuming an empty database.

    //     conn.try_write(b"s json.user User".to_vec())
    //         .unwrap();
    //     conn.try_write(b"s json.city City".to_vec())
    //         .unwrap();
    //     conn.try_write(b"s json.age Age".to_vec()).unwrap();
    //     conn.try_write(b"s json.user.id User.ID".to_vec())
    //         .unwrap();
    //     conn.try_write(b"s json.user.name User.Name".to_vec())
    //         .unwrap();
    //     conn.try_write(b"s json.user.name.first User.Name.First".to_vec())
    //         .unwrap();
    //     conn.try_write(b"s json.user.name.last User.Name.Last".to_vec())
    //         .unwrap();

    //     conn.try_write(b"j json.user".to_vec()).unwrap();
    //     conn.try_write(b"j json.user.name.last".to_vec())
    //         .unwrap();
    //     conn.try_write(b"j json".to_vec()).unwrap();

    //     conn.try_write(b"js json.user".to_vec()).unwrap();
    //     conn.try_write(b"js json.user.name.last".to_vec())
    //         .unwrap();
    //     conn.try_write(b"js json".to_vec()).unwrap();

    //     sleep(Duration::from_millis(200));
    //     let response = conn.try_read().unwrap();
    //     println!("{:?}\n", response);
    //     println!("{}", String::from_utf8_lossy(&response));
    //     sleep(Duration::from_millis(200));

    //     assert_eq!(
    //         response,
    //         &[
    //             79, 75, 10, 79, 75, 10, 79, 75, 10, 79, 75, 10, 79, 75, 10, 79, 75, 10, 79, 75, 10,
    //             123, 34, 105, 100, 34, 58, 91, 56, 53, 44, 49, 49, 53, 44, 49, 48, 49, 44, 49, 49,
    //             52, 44, 52, 54, 44, 55, 51, 44, 54, 56, 93, 44, 34, 110, 97, 109, 101, 34, 58, 123,
    //             34, 102, 105, 114, 115, 116, 34, 58, 91, 56, 53, 44, 49, 49, 53, 44, 49, 48, 49,
    //             44, 49, 49, 52, 44, 52, 54, 44, 55, 56, 44, 57, 55, 44, 49, 48, 57, 44, 49, 48, 49,
    //             44, 52, 54, 44, 55, 48, 44, 49, 48, 53, 44, 49, 49, 52, 44, 49, 49, 53, 44, 49, 49,
    //             54, 93, 44, 34, 108, 97, 115, 116, 34, 58, 91, 56, 53, 44, 49, 49, 53, 44, 49, 48,
    //             49, 44, 49, 49, 52, 44, 52, 54, 44, 55, 56, 44, 57, 55, 44, 49, 48, 57, 44, 49, 48,
    //             49, 44, 52, 54, 44, 55, 54, 44, 57, 55, 44, 49, 49, 53, 44, 49, 49, 54, 93, 125,
    //             125, 10, 91, 56, 53, 44, 49, 49, 53, 44, 49, 48, 49, 44, 49, 49, 52, 44, 52, 54,
    //             44, 55, 56, 44, 57, 55, 44, 49, 48, 57, 44, 49, 48, 49, 44, 52, 54, 44, 55, 54, 44,
    //             57, 55, 44, 49, 49, 53, 44, 49, 49, 54, 93, 10, 123, 34, 97, 103, 101, 34, 58, 91,
    //             54, 53, 44, 49, 48, 51, 44, 49, 48, 49, 93, 44, 34, 99, 105, 116, 121, 34, 58, 91,
    //             54, 55, 44, 49, 48, 53, 44, 49, 49, 54, 44, 49, 50, 49, 93, 44, 34, 117, 115, 101,
    //             114, 34, 58, 123, 34, 105, 100, 34, 58, 91, 56, 53, 44, 49, 49, 53, 44, 49, 48, 49,
    //             44, 49, 49, 52, 44, 52, 54, 44, 55, 51, 44, 54, 56, 93, 44, 34, 110, 97, 109, 101,
    //             34, 58, 123, 34, 102, 105, 114, 115, 116, 34, 58, 91, 56, 53, 44, 49, 49, 53, 44,
    //             49, 48, 49, 44, 49, 49, 52, 44, 52, 54, 44, 55, 56, 44, 57, 55, 44, 49, 48, 57, 44,
    //             49, 48, 49, 44, 52, 54, 44, 55, 48, 44, 49, 48, 53, 44, 49, 49, 52, 44, 49, 49, 53,
    //             44, 49, 49, 54, 93, 44, 34, 108, 97, 115, 116, 34, 58, 91, 56, 53, 44, 49, 49, 53,
    //             44, 49, 48, 49, 44, 49, 49, 52, 44, 52, 54, 44, 55, 56, 44, 57, 55, 44, 49, 48, 57,
    //             44, 49, 48, 49, 44, 52, 54, 44, 55, 54, 44, 57, 55, 44, 49, 49, 53, 44, 49, 49, 54,
    //             93, 125, 125, 125, 10, 123, 34, 106, 115, 111, 110, 34, 58, 123, 34, 117, 115, 101,
    //             114, 34, 58, 123, 34, 105, 100, 34, 58, 91, 56, 53, 44, 49, 49, 53, 44, 49, 48, 49,
    //             44, 49, 49, 52, 44, 52, 54, 44, 55, 51, 44, 54, 56, 93, 44, 34, 110, 97, 109, 101,
    //             34, 58, 123, 34, 102, 105, 114, 115, 116, 34, 58, 91, 56, 53, 44, 49, 49, 53, 44,
    //             49, 48, 49, 44, 49, 49, 52, 44, 52, 54, 44, 55, 56, 44, 57, 55, 44, 49, 48, 57, 44,
    //             49, 48, 49, 44, 52, 54, 44, 55, 48, 44, 49, 48, 53, 44, 49, 49, 52, 44, 49, 49, 53,
    //             44, 49, 49, 54, 93, 44, 34, 108, 97, 115, 116, 34, 58, 91, 56, 53, 44, 49, 49, 53,
    //             44, 49, 48, 49, 44, 49, 49, 52, 44, 52, 54, 44, 55, 56, 44, 57, 55, 44, 49, 48, 57,
    //             44, 49, 48, 49, 44, 52, 54, 44, 55, 54, 44, 57, 55, 44, 49, 49, 53, 44, 49, 49, 54,
    //             93, 125, 125, 125, 125, 10, 123, 34, 106, 115, 111, 110, 34, 58, 123, 34, 117, 115,
    //             101, 114, 34, 58, 123, 34, 110, 97, 109, 101, 34, 58, 123, 34, 108, 97, 115, 116,
    //             34, 58, 91, 56, 53, 44, 49, 49, 53, 44, 49, 48, 49, 44, 49, 49, 52, 44, 52, 54, 44,
    //             55, 56, 44, 57, 55, 44, 49, 48, 57, 44, 49, 48, 49, 44, 52, 54, 44, 55, 54, 44, 57,
    //             55, 44, 49, 49, 53, 44, 49, 49, 54, 93, 125, 125, 125, 125, 10, 123, 34, 106, 115,
    //             111, 110, 34, 58, 123, 34, 97, 103, 101, 34, 58, 91, 54, 53, 44, 49, 48, 51, 44,
    //             49, 48, 49, 93, 44, 34, 99, 105, 116, 121, 34, 58, 91, 54, 55, 44, 49, 48, 53, 44,
    //             49, 49, 54, 44, 49, 50, 49, 93, 44, 34, 117, 115, 101, 114, 34, 58, 123, 34, 105,
    //             100, 34, 58, 91, 56, 53, 44, 49, 49, 53, 44, 49, 48, 49, 44, 49, 49, 52, 44, 52,
    //             54, 44, 55, 51, 44, 54, 56, 93, 44, 34, 110, 97, 109, 101, 34, 58, 123, 34, 102,
    //             105, 114, 115, 116, 34, 58, 91, 56, 53, 44, 49, 49, 53, 44, 49, 48, 49, 44, 49, 49,
    //             52, 44, 52, 54, 44, 55, 56, 44, 57, 55, 44, 49, 48, 57, 44, 49, 48, 49, 44, 52, 54,
    //             44, 55, 48, 44, 49, 48, 53, 44, 49, 49, 52, 44, 49, 49, 53, 44, 49, 49, 54, 93, 44,
    //             34, 108, 97, 115, 116, 34, 58, 91, 56, 53, 44, 49, 49, 53, 44, 49, 48, 49, 44, 49,
    //             49, 52, 44, 52, 54, 44, 55, 56, 44, 57, 55, 44, 49, 48, 57, 44, 49, 48, 49, 44, 52,
    //             54, 44, 55, 54, 44, 57, 55, 44, 49, 49, 53, 44, 49, 49, 54, 93, 125, 125, 125, 125,
    //             10
    //         ]
    //     );
    // }

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
