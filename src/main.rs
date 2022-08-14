mod conn;

use crate::conn::Connection;

use std::io::{stdin, stdout, Write};
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let server = TcpStream::connect("127.0.0.1:1984").unwrap();
    server.set_nonblocking(true).unwrap();

    let addr = server.local_addr().unwrap();
    let mut conn = Connection::new(0, server, addr);

    loop {
        let mut input = String::new();

        print!("> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();

        let message = input.trim().as_bytes().to_vec();
        println!("> {:?}", message);
        conn.try_write_message(message).unwrap();

        sleep(Duration::from_millis(200));
        let response = conn.try_read().unwrap();
        println!("\n{:?}", response);
        println!("\n{}", String::from_utf8_lossy(&response));
    }
}

#[cfg(test)]
mod tests {
    use crate::conn::Connection;

    use std::{net::TcpStream, thread::sleep, time::Duration};

    #[test]
    fn empty_message() {
        let server = TcpStream::connect("127.0.0.1:1984").unwrap();
        server.set_nonblocking(true).unwrap();

        let addr = server.local_addr().unwrap();
        let mut conn = Connection::new(0, server, addr);

        for _ in 0..10 {
            conn.try_write_message(b"".to_vec()).unwrap();
        }

        sleep(Duration::from_millis(200));
        let response = conn.try_read().unwrap();
        println!("{:?}\n", response);
        println!("{}", String::from_utf8_lossy(&response));
        sleep(Duration::from_millis(200));

        assert_eq!(response, &[]);
    }

    #[test]
    fn set() {
        let server = TcpStream::connect("127.0.0.1:1984").unwrap();
        server.set_nonblocking(true).unwrap();

        let addr = server.local_addr().unwrap();
        let mut conn = Connection::new(0, server, addr);

        conn.try_write_message(b"s set Set!".to_vec()).unwrap();
        conn.try_write_message(b"g set".to_vec()).unwrap();
        conn.try_write_message(b"s set".to_vec()).unwrap();
        conn.try_write_message(b"g set".to_vec()).unwrap();

        sleep(Duration::from_millis(200));
        let response = conn.try_read().unwrap();
        println!("{:?}\n", response);
        println!("{}", String::from_utf8_lossy(&response));
        sleep(Duration::from_millis(200));

        assert_eq!(
            response,
            &[79, 75, 10, 83, 101, 116, 33, 10, 79, 75, 10, 10]
        );
    }

    #[test]
    fn set_if_none() {
        let server = TcpStream::connect("127.0.0.1:1984").unwrap();
        server.set_nonblocking(true).unwrap();

        let addr = server.local_addr().unwrap();
        let mut conn = Connection::new(0, server, addr);

        conn.try_write_message(b"d maybe".to_vec()).unwrap();
        conn.try_write_message(b"s? maybe Maybe!".to_vec()).unwrap();
        conn.try_write_message(b"g maybe".to_vec()).unwrap();
        conn.try_write_message(b"s? maybe New maybe!".to_vec())
            .unwrap();
        conn.try_write_message(b"g maybe".to_vec()).unwrap();

        sleep(Duration::from_millis(200));
        let response = conn.try_read().unwrap();
        println!("{:?}\n", response);
        println!("{}", String::from_utf8_lossy(&response));
        sleep(Duration::from_millis(200));

        assert_eq!(
            response,
            &[
                79, 75, 10, 79, 75, 10, 77, 97, 121, 98, 101, 33, 10, 79, 75, 10, 77, 97, 121, 98,
                101, 33, 10
            ]
        );
    }

    #[test]
    fn inc() {
        let server = TcpStream::connect("127.0.0.1:1984").unwrap();
        server.set_nonblocking(true).unwrap();

        let addr = server.local_addr().unwrap();
        let mut conn = Connection::new(0, server, addr);

        conn.try_write_message(b"d inc".to_vec()).unwrap();
        conn.try_write_message(b"+1 inc".to_vec()).unwrap();
        conn.try_write_message(b"g inc".to_vec()).unwrap();
        conn.try_write_message(b"+1 inc".to_vec()).unwrap();
        conn.try_write_message(b"g inc".to_vec()).unwrap();

        sleep(Duration::from_millis(200));
        let response = conn.try_read().unwrap();
        println!("{:?}\n", response);
        println!("{}", String::from_utf8_lossy(&response));
        sleep(Duration::from_millis(200));

        assert_eq!(
            response,
            &[
                79, 75, 10, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0,
                0, 0, 1, 10, 0, 0, 0, 0, 0, 0, 0, 1, 10
            ]
        );
    }

    #[test]
    fn append() {
        let server = TcpStream::connect("127.0.0.1:1984").unwrap();
        server.set_nonblocking(true).unwrap();

        let addr = server.local_addr().unwrap();
        let mut conn = Connection::new(0, server, addr);

        conn.try_write_message(b"d append Half".to_vec()).unwrap();
        conn.try_write_message(b"+ append One".to_vec()).unwrap();
        conn.try_write_message(b"+ append Two".to_vec()).unwrap();

        conn.try_write_message(b"s append Three".to_vec()).unwrap();
        conn.try_write_message(b"+ append Four".to_vec()).unwrap();

        sleep(Duration::from_millis(200));
        let response = conn.try_read().unwrap();
        println!("{:?}\n", response);
        println!("{}", String::from_utf8_lossy(&response));
        sleep(Duration::from_millis(200));

        assert_eq!(
            response,
            &[
                79, 75, 10, 79, 110, 101, 10, 79, 110, 101, 84, 119, 111, 10, 79, 75, 10, 84, 104,
                114, 101, 101, 70, 111, 117, 114, 10
            ]
        );
    }

    #[test]
    fn get_delete() {
        let server = TcpStream::connect("127.0.0.1:1984").unwrap();
        server.set_nonblocking(true).unwrap();

        let addr = server.local_addr().unwrap();
        let mut conn = Connection::new(0, server, addr);

        conn.try_write_message(b"d set".to_vec()).unwrap();
        conn.try_write_message(b"d maybe".to_vec()).unwrap();
        conn.try_write_message(b"d inc".to_vec()).unwrap();
        conn.try_write_message(b"d append".to_vec()).unwrap();

        conn.try_write_message(b"s set Set!".to_vec()).unwrap();
        conn.try_write_message(b"s? maybe Maybe!".to_vec()).unwrap();
        conn.try_write_message(b"+1 inc".to_vec()).unwrap();
        conn.try_write_message(b"+ append Append!".to_vec())
            .unwrap();

        conn.try_write_message(b"g set".to_vec()).unwrap();
        conn.try_write_message(b"g maybe".to_vec()).unwrap();
        conn.try_write_message(b"g inc".to_vec()).unwrap();
        conn.try_write_message(b"g append".to_vec()).unwrap();

        conn.try_write_message(b"d set".to_vec()).unwrap();
        conn.try_write_message(b"d maybe".to_vec()).unwrap();
        conn.try_write_message(b"d inc".to_vec()).unwrap();
        conn.try_write_message(b"d append".to_vec()).unwrap();

        conn.try_write_message(b"g set".to_vec()).unwrap();
        conn.try_write_message(b"g maybe".to_vec()).unwrap();
        conn.try_write_message(b"g inc".to_vec()).unwrap();
        conn.try_write_message(b"g append".to_vec()).unwrap();

        sleep(Duration::from_millis(200));
        let response = conn.try_read().unwrap();
        println!("{:?}\n", response);
        println!("{}", String::from_utf8_lossy(&response));
        sleep(Duration::from_millis(200));

        assert_eq!(
            response,
            &[
                79, 75, 10, 79, 75, 10, 79, 75, 10, 79, 75, 10, 79, 75, 10, 79, 75, 10, 0, 0, 0, 0,
                0, 0, 0, 0, 10, 65, 112, 112, 101, 110, 100, 33, 10, 83, 101, 116, 33, 10, 77, 97,
                121, 98, 101, 33, 10, 0, 0, 0, 0, 0, 0, 0, 0, 10, 65, 112, 112, 101, 110, 100, 33,
                10, 79, 75, 10, 79, 75, 10, 79, 75, 10, 79, 75, 10, 10, 10, 10, 10
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

        conn.try_write_message(b"s key_value.1 One".to_vec())
            .unwrap();
        conn.try_write_message(b"s key_value.2 Two".to_vec())
            .unwrap();
        conn.try_write_message(b"s key_value.3 Three".to_vec())
            .unwrap();
        conn.try_write_message(b"s key_value.3.1 Three.One".to_vec())
            .unwrap();
        conn.try_write_message(b"s key_value.3.2 Three.Two".to_vec())
            .unwrap();
        conn.try_write_message(b"k key_value".to_vec()).unwrap();

        sleep(Duration::from_millis(200));
        let response = conn.try_read().unwrap();
        println!("{:?}\n", response);
        println!("{}", String::from_utf8_lossy(&response));
        sleep(Duration::from_millis(200));

        assert_eq!(
            response,
            &[
                79, 75, 10, 79, 75, 10, 79, 75, 10, 79, 75, 10, 79, 75, 10, 49, 32, 79, 110, 101,
                0, 50, 32, 84, 119, 111, 0, 51, 32, 84, 104, 114, 101, 101, 0, 49, 32, 84, 104,
                114, 101, 101, 46, 79, 110, 101, 0, 50, 32, 84, 104, 114, 101, 101, 46, 84, 119,
                111, 10
            ]
        );
    }

    #[test]
    fn json() {
        let server = TcpStream::connect("127.0.0.1:1984").unwrap();
        server.set_nonblocking(true).unwrap();

        let addr = server.local_addr().unwrap();
        let mut conn = Connection::new(0, server, addr);

        // This test fails when key_value has more children than expected. We
        // are assuming an empty database.

        conn.try_write_message(b"s json.user User".to_vec())
            .unwrap();
        conn.try_write_message(b"s json.city City".to_vec())
            .unwrap();
        conn.try_write_message(b"s json.age Age".to_vec()).unwrap();
        conn.try_write_message(b"s json.user.id User.ID".to_vec())
            .unwrap();
        conn.try_write_message(b"s json.user.name User.Name".to_vec())
            .unwrap();
        conn.try_write_message(b"s json.user.name.first User.Name.First".to_vec())
            .unwrap();
        conn.try_write_message(b"s json.user.name.last User.Name.Last".to_vec())
            .unwrap();

        conn.try_write_message(b"j json.user".to_vec()).unwrap();
        conn.try_write_message(b"j json.user.name.last".to_vec())
            .unwrap();
        conn.try_write_message(b"j json".to_vec()).unwrap();

        conn.try_write_message(b"js json.user".to_vec()).unwrap();
        conn.try_write_message(b"js json.user.name.last".to_vec())
            .unwrap();
        conn.try_write_message(b"js json".to_vec()).unwrap();

        sleep(Duration::from_millis(200));
        let response = conn.try_read().unwrap();
        println!("{:?}\n", response);
        println!("{}", String::from_utf8_lossy(&response));
        sleep(Duration::from_millis(200));

        assert_eq!(
            response,
            &[
                79, 75, 10, 79, 75, 10, 79, 75, 10, 79, 75, 10, 79, 75, 10, 79, 75, 10, 79, 75, 10,
                123, 34, 105, 100, 34, 58, 91, 56, 53, 44, 49, 49, 53, 44, 49, 48, 49, 44, 49, 49,
                52, 44, 52, 54, 44, 55, 51, 44, 54, 56, 93, 44, 34, 110, 97, 109, 101, 34, 58, 123,
                34, 102, 105, 114, 115, 116, 34, 58, 91, 56, 53, 44, 49, 49, 53, 44, 49, 48, 49,
                44, 49, 49, 52, 44, 52, 54, 44, 55, 56, 44, 57, 55, 44, 49, 48, 57, 44, 49, 48, 49,
                44, 52, 54, 44, 55, 48, 44, 49, 48, 53, 44, 49, 49, 52, 44, 49, 49, 53, 44, 49, 49,
                54, 93, 44, 34, 108, 97, 115, 116, 34, 58, 91, 56, 53, 44, 49, 49, 53, 44, 49, 48,
                49, 44, 49, 49, 52, 44, 52, 54, 44, 55, 56, 44, 57, 55, 44, 49, 48, 57, 44, 49, 48,
                49, 44, 52, 54, 44, 55, 54, 44, 57, 55, 44, 49, 49, 53, 44, 49, 49, 54, 93, 125,
                125, 10, 91, 56, 53, 44, 49, 49, 53, 44, 49, 48, 49, 44, 49, 49, 52, 44, 52, 54,
                44, 55, 56, 44, 57, 55, 44, 49, 48, 57, 44, 49, 48, 49, 44, 52, 54, 44, 55, 54, 44,
                57, 55, 44, 49, 49, 53, 44, 49, 49, 54, 93, 10, 123, 34, 97, 103, 101, 34, 58, 91,
                54, 53, 44, 49, 48, 51, 44, 49, 48, 49, 93, 44, 34, 99, 105, 116, 121, 34, 58, 91,
                54, 55, 44, 49, 48, 53, 44, 49, 49, 54, 44, 49, 50, 49, 93, 44, 34, 117, 115, 101,
                114, 34, 58, 123, 34, 105, 100, 34, 58, 91, 56, 53, 44, 49, 49, 53, 44, 49, 48, 49,
                44, 49, 49, 52, 44, 52, 54, 44, 55, 51, 44, 54, 56, 93, 44, 34, 110, 97, 109, 101,
                34, 58, 123, 34, 102, 105, 114, 115, 116, 34, 58, 91, 56, 53, 44, 49, 49, 53, 44,
                49, 48, 49, 44, 49, 49, 52, 44, 52, 54, 44, 55, 56, 44, 57, 55, 44, 49, 48, 57, 44,
                49, 48, 49, 44, 52, 54, 44, 55, 48, 44, 49, 48, 53, 44, 49, 49, 52, 44, 49, 49, 53,
                44, 49, 49, 54, 93, 44, 34, 108, 97, 115, 116, 34, 58, 91, 56, 53, 44, 49, 49, 53,
                44, 49, 48, 49, 44, 49, 49, 52, 44, 52, 54, 44, 55, 56, 44, 57, 55, 44, 49, 48, 57,
                44, 49, 48, 49, 44, 52, 54, 44, 55, 54, 44, 57, 55, 44, 49, 49, 53, 44, 49, 49, 54,
                93, 125, 125, 125, 10, 123, 34, 106, 115, 111, 110, 34, 58, 123, 34, 117, 115, 101,
                114, 34, 58, 123, 34, 105, 100, 34, 58, 91, 56, 53, 44, 49, 49, 53, 44, 49, 48, 49,
                44, 49, 49, 52, 44, 52, 54, 44, 55, 51, 44, 54, 56, 93, 44, 34, 110, 97, 109, 101,
                34, 58, 123, 34, 102, 105, 114, 115, 116, 34, 58, 91, 56, 53, 44, 49, 49, 53, 44,
                49, 48, 49, 44, 49, 49, 52, 44, 52, 54, 44, 55, 56, 44, 57, 55, 44, 49, 48, 57, 44,
                49, 48, 49, 44, 52, 54, 44, 55, 48, 44, 49, 48, 53, 44, 49, 49, 52, 44, 49, 49, 53,
                44, 49, 49, 54, 93, 44, 34, 108, 97, 115, 116, 34, 58, 91, 56, 53, 44, 49, 49, 53,
                44, 49, 48, 49, 44, 49, 49, 52, 44, 52, 54, 44, 55, 56, 44, 57, 55, 44, 49, 48, 57,
                44, 49, 48, 49, 44, 52, 54, 44, 55, 54, 44, 57, 55, 44, 49, 49, 53, 44, 49, 49, 54,
                93, 125, 125, 125, 125, 10, 123, 34, 106, 115, 111, 110, 34, 58, 123, 34, 117, 115,
                101, 114, 34, 58, 123, 34, 110, 97, 109, 101, 34, 58, 123, 34, 108, 97, 115, 116,
                34, 58, 91, 56, 53, 44, 49, 49, 53, 44, 49, 48, 49, 44, 49, 49, 52, 44, 52, 54, 44,
                55, 56, 44, 57, 55, 44, 49, 48, 57, 44, 49, 48, 49, 44, 52, 54, 44, 55, 54, 44, 57,
                55, 44, 49, 49, 53, 44, 49, 49, 54, 93, 125, 125, 125, 125, 10, 123, 34, 106, 115,
                111, 110, 34, 58, 123, 34, 97, 103, 101, 34, 58, 91, 54, 53, 44, 49, 48, 51, 44,
                49, 48, 49, 93, 44, 34, 99, 105, 116, 121, 34, 58, 91, 54, 55, 44, 49, 48, 53, 44,
                49, 49, 54, 44, 49, 50, 49, 93, 44, 34, 117, 115, 101, 114, 34, 58, 123, 34, 105,
                100, 34, 58, 91, 56, 53, 44, 49, 49, 53, 44, 49, 48, 49, 44, 49, 49, 52, 44, 52,
                54, 44, 55, 51, 44, 54, 56, 93, 44, 34, 110, 97, 109, 101, 34, 58, 123, 34, 102,
                105, 114, 115, 116, 34, 58, 91, 56, 53, 44, 49, 49, 53, 44, 49, 48, 49, 44, 49, 49,
                52, 44, 52, 54, 44, 55, 56, 44, 57, 55, 44, 49, 48, 57, 44, 49, 48, 49, 44, 52, 54,
                44, 55, 48, 44, 49, 48, 53, 44, 49, 49, 52, 44, 49, 49, 53, 44, 49, 49, 54, 93, 44,
                34, 108, 97, 115, 116, 34, 58, 91, 56, 53, 44, 49, 49, 53, 44, 49, 48, 49, 44, 49,
                49, 52, 44, 52, 54, 44, 55, 56, 44, 57, 55, 44, 49, 48, 57, 44, 49, 48, 49, 44, 52,
                54, 44, 55, 54, 44, 57, 55, 44, 49, 49, 53, 44, 49, 49, 54, 93, 125, 125, 125, 125,
                10
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

        conn.try_write_message(b"#g subs".to_vec()).unwrap();
        conn.try_write_message(b"#j subs".to_vec()).unwrap();
        conn.try_write_message(b"#k subs".to_vec()).unwrap();

        conn.try_write_message(b"s subs.user User".to_vec())
            .unwrap();
        conn.try_write_message(b"d subs.city".to_vec()).unwrap();
        conn.try_write_message(b"+ subs.city City".to_vec())
            .unwrap();
        conn.try_write_message(b"d subs.age".to_vec()).unwrap();
        conn.try_write_message(b"+1 subs.age Age".to_vec()).unwrap();
        conn.try_write_message(b"d subs.user.id".to_vec()).unwrap();
        conn.try_write_message(b"+1 subs.user.id User.ID".to_vec())
            .unwrap();
        conn.try_write_message(b"! subs.user.name User.Name".to_vec())
            .unwrap();
        conn.try_write_message(b"! subs.user.name.first User.Name.First".to_vec())
            .unwrap();
        conn.try_write_message(b"! subs.user.name.last User.Name.Last".to_vec())
            .unwrap();

        conn.try_write_message(b"#- subs".to_vec()).unwrap();

        conn.try_write_message(b"s subs.user User".to_vec())
            .unwrap();
        conn.try_write_message(b"d subs.city".to_vec()).unwrap();
        conn.try_write_message(b"+ subs.city City".to_vec())
            .unwrap();
        conn.try_write_message(b"d subs.age".to_vec()).unwrap();
        conn.try_write_message(b"+1 subs.age Age".to_vec()).unwrap();

        sleep(Duration::from_millis(200));
        let response = conn.try_read().unwrap();
        println!("{:?}\n", response);
        println!("{}", String::from_utf8_lossy(&response));
        sleep(Duration::from_millis(200));

        assert_eq!(
            response,
            &[
                79, 75, 10, 79, 75, 10, 79, 75, 10, 79, 75, 10, 85, 115, 101, 114, 10, 123, 34,
                117, 115, 101, 114, 34, 58, 34, 85, 115, 101, 114, 34, 125, 10, 117, 115, 101, 114,
                32, 85, 115, 101, 114, 10, 79, 75, 10, 67, 105, 116, 121, 10, 67, 105, 116, 121,
                10, 123, 34, 99, 105, 116, 121, 34, 58, 34, 67, 105, 116, 121, 34, 125, 10, 99,
                105, 116, 121, 32, 67, 105, 116, 121, 10, 79, 75, 10, 0, 0, 0, 0, 0, 0, 0, 0, 10,
                0, 0, 0, 0, 0, 0, 0, 0, 10, 123, 34, 97, 103, 101, 34, 58, 34, 92, 117, 48, 48, 48,
                48, 92, 117, 48, 48, 48, 48, 92, 117, 48, 48, 48, 48, 92, 117, 48, 48, 48, 48, 92,
                117, 48, 48, 48, 48, 92, 117, 48, 48, 48, 48, 92, 117, 48, 48, 48, 48, 92, 117, 48,
                48, 48, 48, 34, 125, 10, 97, 103, 101, 32, 0, 0, 0, 0, 0, 0, 0, 0, 10, 79, 75, 10,
                0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 10, 123, 34, 105, 100, 34, 58,
                34, 92, 117, 48, 48, 48, 48, 92, 117, 48, 48, 48, 48, 92, 117, 48, 48, 48, 48, 92,
                117, 48, 48, 48, 48, 92, 117, 48, 48, 48, 48, 92, 117, 48, 48, 48, 48, 92, 117, 48,
                48, 48, 48, 92, 117, 48, 48, 48, 48, 34, 125, 10, 105, 100, 32, 0, 0, 0, 0, 0, 0,
                0, 0, 10, 79, 75, 10, 85, 115, 101, 114, 46, 78, 97, 109, 101, 10, 123, 34, 110,
                97, 109, 101, 34, 58, 34, 85, 115, 101, 114, 46, 78, 97, 109, 101, 34, 125, 10,
                110, 97, 109, 101, 32, 85, 115, 101, 114, 46, 78, 97, 109, 101, 10, 79, 75, 10, 85,
                115, 101, 114, 46, 78, 97, 109, 101, 46, 70, 105, 114, 115, 116, 10, 123, 34, 102,
                105, 114, 115, 116, 34, 58, 34, 85, 115, 101, 114, 46, 78, 97, 109, 101, 46, 70,
                105, 114, 115, 116, 34, 125, 10, 102, 105, 114, 115, 116, 32, 85, 115, 101, 114,
                46, 78, 97, 109, 101, 46, 70, 105, 114, 115, 116, 10, 79, 75, 10, 85, 115, 101,
                114, 46, 78, 97, 109, 101, 46, 76, 97, 115, 116, 10, 123, 34, 108, 97, 115, 116,
                34, 58, 34, 85, 115, 101, 114, 46, 78, 97, 109, 101, 46, 76, 97, 115, 116, 34, 125,
                10, 108, 97, 115, 116, 32, 85, 115, 101, 114, 46, 78, 97, 109, 101, 46, 76, 97,
                115, 116, 10, 79, 75, 10, 79, 75, 10, 79, 75, 10, 67, 105, 116, 121, 10, 79, 75,
                10, 0, 0, 0, 0, 0, 0, 0, 0, 10
            ]
        );
    }
}
