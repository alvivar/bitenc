mod conn;

use crate::conn::Connection;

use std::{net::TcpStream, thread::sleep, time::Duration};

fn main() {
    let server = TcpStream::connect("127.0.0.1:1984").unwrap();
    server.set_nonblocking(true).unwrap();

    let addr = server.local_addr().unwrap();
    let mut conn = Connection::new(0, server, addr);

    conn.try_write_message(b"s set Set!");
    conn.try_write_message(b"g set");

    sleep(Duration::from_secs(1));
    let response = conn.try_read().unwrap();
    println!("{:?}\n", response);
    println!("{}", String::from_utf8_lossy(&response));
    sleep(Duration::from_secs(1));

    conn.try_write_message(b"#g sub");
    conn.try_write_message(b"#j sub");
    conn.try_write_message(b"#k sub");
    conn.try_write_message(b"s sub Sub sent by set!");
    conn.try_write_message(b"! sub Sub sent by !");

    conn.try_write_message(b"j");
    conn.try_write_message(b"js");
    conn.try_write_message(b"k");

    sleep(Duration::from_secs(1));
    let response = conn.try_read().unwrap();
    println!("{:?}\n", response);
    println!("{}", String::from_utf8_lossy(&response));
    sleep(Duration::from_secs(1));
}

#[cfg(test)]
mod tests {
    use crate::conn::Connection;

    use std::{net::TcpStream, thread::sleep, time::Duration};

    #[test]
    fn set() {
        let server = TcpStream::connect("127.0.0.1:1984").unwrap();
        server.set_nonblocking(true).unwrap();

        let addr = server.local_addr().unwrap();
        let mut conn = Connection::new(0, server, addr);

        conn.try_write_message(b"s set Set!");
        conn.try_write_message(b"g set");
        conn.try_write_message(b"s set");
        conn.try_write_message(b"g set");

        sleep(Duration::from_secs(1));
        let response = conn.try_read().unwrap();
        println!("{:?}\n", response);
        println!("{}", String::from_utf8_lossy(&response));
        sleep(Duration::from_secs(1));

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

        conn.try_write_message(b"d maybe");
        conn.try_write_message(b"s? maybe Maybe!");
        conn.try_write_message(b"g maybe");
        conn.try_write_message(b"s? maybe New maybe!");
        conn.try_write_message(b"g maybe");

        sleep(Duration::from_secs(1));
        let response = conn.try_read().unwrap();
        println!("{:?}\n", response);
        println!("{}", String::from_utf8_lossy(&response));
        sleep(Duration::from_secs(1));

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

        conn.try_write_message(b"d inc");
        conn.try_write_message(b"+1 inc");
        conn.try_write_message(b"g inc");
        conn.try_write_message(b"+1 inc");
        conn.try_write_message(b"g inc");

        sleep(Duration::from_secs(1));
        let response = conn.try_read().unwrap();
        println!("{:?}\n", response);
        println!("{}", String::from_utf8_lossy(&response));
        sleep(Duration::from_secs(1));

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

        conn.try_write_message(b"d append Half");
        conn.try_write_message(b"+ append One");
        conn.try_write_message(b"+ append Two");

        conn.try_write_message(b"s append Three");
        conn.try_write_message(b"+ append Four");

        sleep(Duration::from_secs(1));
        let response = conn.try_read().unwrap();
        println!("{:?}\n", response);
        println!("{}", String::from_utf8_lossy(&response));
        sleep(Duration::from_secs(1));

        assert_eq!(
            response,
            &[
                79, 75, 10, 79, 110, 101, 10, 79, 110, 101, 84, 119, 111, 10, 79, 75, 10, 84, 104,
                114, 101, 101, 70, 111, 117, 114, 10
            ]
        );
    }

    #[test]
    fn delete() {
        let server = TcpStream::connect("127.0.0.1:1984").unwrap();
        server.set_nonblocking(true).unwrap();

        let addr = server.local_addr().unwrap();
        let mut conn = Connection::new(0, server, addr);

        conn.try_write_message(b"d set");
        conn.try_write_message(b"d maybe");
        conn.try_write_message(b"d inc");
        conn.try_write_message(b"d append");

        conn.try_write_message(b"s set Set!");
        conn.try_write_message(b"s? maybe Maybe!");
        conn.try_write_message(b"+1 inc");
        conn.try_write_message(b"+ append Append!");

        conn.try_write_message(b"g set");
        conn.try_write_message(b"g maybe");
        conn.try_write_message(b"g inc");
        conn.try_write_message(b"g append");

        conn.try_write_message(b"d set");
        conn.try_write_message(b"d maybe");
        conn.try_write_message(b"d inc");
        conn.try_write_message(b"d append");

        conn.try_write_message(b"g set");
        conn.try_write_message(b"g maybe");
        conn.try_write_message(b"g inc");
        conn.try_write_message(b"g append");

        sleep(Duration::from_secs(1));
        let response = conn.try_read().unwrap();
        println!("{:?}\n", response);
        println!("{}", String::from_utf8_lossy(&response));
        sleep(Duration::from_secs(1));

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
}
