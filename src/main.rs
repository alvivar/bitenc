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
    fn get_delete() {
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

    #[test]
    fn key_value() {
        let server = TcpStream::connect("127.0.0.1:1984").unwrap();
        server.set_nonblocking(true).unwrap();

        let addr = server.local_addr().unwrap();
        let mut conn = Connection::new(0, server, addr);

        // This test fails when key_value has more children than expected. We
        // are assuming an empty database.

        conn.try_write_message(b"s key_value.1 One");
        conn.try_write_message(b"s key_value.2 Two");
        conn.try_write_message(b"s key_value.3 Three");
        conn.try_write_message(b"s key_value.3.1 Three.One");
        conn.try_write_message(b"s key_value.3.2 Three.Two");
        conn.try_write_message(b"k key_value");

        sleep(Duration::from_secs(1));
        let response = conn.try_read().unwrap();
        println!("{:?}\n", response);
        println!("{}", String::from_utf8_lossy(&response));
        sleep(Duration::from_secs(1));

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

        conn.try_write_message(b"s json.user User");
        conn.try_write_message(b"s json.city City");
        conn.try_write_message(b"s json.age Age");
        conn.try_write_message(b"s json.user.id User.ID");
        conn.try_write_message(b"s json.user.name User.Name");
        conn.try_write_message(b"s json.user.name.first User.Name.First");
        conn.try_write_message(b"s json.user.name.last User.Name.Last");

        conn.try_write_message(b"j json.user");
        conn.try_write_message(b"j json.user.name.last");
        conn.try_write_message(b"j json");

        conn.try_write_message(b"js json.user");
        conn.try_write_message(b"js json.user.name.last");
        conn.try_write_message(b"js json");

        sleep(Duration::from_secs(1));
        let response = conn.try_read().unwrap();
        println!("{:?}\n", response);
        println!("{}", String::from_utf8_lossy(&response));
        sleep(Duration::from_secs(1));

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

    // #[test]
    // fn subscriptions() {
    //     let server = TcpStream::connect("127.0.0.1:1984").unwrap();
    //     server.set_nonblocking(true).unwrap();

    //     let addr = server.local_addr().unwrap();
    //     let mut conn = Connection::new(0, server, addr);

    //     // This test fails when key_value has more children than expected. We
    //     // are assuming an empty database.

    //     conn.try_write_message(b"#g subs");
    //     conn.try_write_message(b"#j subs");
    //     conn.try_write_message(b"#k subs");

    //     conn.try_write_message(b"s subs.user User");
    //     conn.try_write_message(b"s subs.city City");
    //     conn.try_write_message(b"s subs.age Age");
    //     conn.try_write_message(b"s subs.user.id User.ID");
    //     conn.try_write_message(b"s subs.user.name User.Name");
    //     conn.try_write_message(b"s subs.user.name.first User.Name.First");
    //     conn.try_write_message(b"s subs.user.name.last User.Name.Last");

    //     sleep(Duration::from_secs(1));
    //     let response = conn.try_read().unwrap();
    //     println!("{:?}\n", response);
    //     println!("{}", String::from_utf8_lossy(&response));
    //     sleep(Duration::from_secs(1));

    //     assert_eq!(
    //         response,
    //         &[
    //             79, 75, 10, 79, 75, 10, 79, 75, 10, 79, 75, 10, 79, 75, 10, 79, 75, 10, 79, 75, 10,
    //             91, 55, 57, 44, 49, 49, 48, 44, 49, 48, 49, 93, 10, 91, 56, 52, 44, 49, 48, 52, 44,
    //             49, 49, 52, 44, 49, 48, 49, 44, 49, 48, 49, 44, 52, 54, 44, 56, 52, 44, 49, 49, 57,
    //             44, 49, 49, 49, 44, 52, 54, 44, 55, 57, 44, 49, 49, 48, 44, 49, 48, 49, 93, 10,
    //             123, 34, 49, 34, 58, 91, 55, 57, 44, 49, 49, 48, 44, 49, 48, 49, 93, 44, 34, 50,
    //             34, 58, 91, 56, 52, 44, 49, 49, 57, 44, 49, 49, 49, 93, 44, 34, 51, 34, 58, 123,
    //             34, 49, 34, 58, 91, 56, 52, 44, 49, 48, 52, 44, 49, 49, 52, 44, 49, 48, 49, 44, 49,
    //             48, 49, 44, 52, 54, 44, 55, 57, 44, 49, 49, 48, 44, 49, 48, 49, 93, 44, 34, 50, 34,
    //             58, 123, 34, 49, 34, 58, 91, 56, 52, 44, 49, 48, 52, 44, 49, 49, 52, 44, 49, 48,
    //             49, 44, 49, 48, 49, 44, 52, 54, 44, 56, 52, 44, 49, 49, 57, 44, 49, 49, 49, 44, 52,
    //             54, 44, 55, 57, 44, 49, 49, 48, 44, 49, 48, 49, 93, 125, 125, 125, 10, 123, 34,
    //             106, 115, 111, 110, 34, 58, 123, 34, 49, 34, 58, 91, 55, 57, 44, 49, 49, 48, 44,
    //             49, 48, 49, 93, 125, 125, 10, 123, 34, 106, 115, 111, 110, 34, 58, 123, 34, 51, 34,
    //             58, 123, 34, 50, 34, 58, 123, 34, 49, 34, 58, 91, 56, 52, 44, 49, 48, 52, 44, 49,
    //             49, 52, 44, 49, 48, 49, 44, 49, 48, 49, 44, 52, 54, 44, 56, 52, 44, 49, 49, 57, 44,
    //             49, 49, 49, 44, 52, 54, 44, 55, 57, 44, 49, 49, 48, 44, 49, 48, 49, 93, 125, 125,
    //             125, 125, 10, 123, 34, 106, 115, 111, 110, 34, 58, 123, 34, 49, 34, 58, 91, 55, 57,
    //             44, 49, 49, 48, 44, 49, 48, 49, 93, 44, 34, 50, 34, 58, 91, 56, 52, 44, 49, 49, 57,
    //             44, 49, 49, 49, 93, 44, 34, 51, 34, 58, 123, 34, 49, 34, 58, 91, 56, 52, 44, 49,
    //             48, 52, 44, 49, 49, 52, 44, 49, 48, 49, 44, 49, 48, 49, 44, 52, 54, 44, 55, 57, 44,
    //             49, 49, 48, 44, 49, 48, 49, 93, 44, 34, 50, 34, 58, 123, 34, 49, 34, 58, 91, 56,
    //             52, 44, 49, 48, 52, 44, 49, 49, 52, 44, 49, 48, 49, 44, 49, 48, 49, 44, 52, 54, 44,
    //             56, 52, 44, 49, 49, 57, 44, 49, 49, 49, 44, 52, 54, 44, 55, 57, 44, 49, 49, 48, 44,
    //             49, 48, 49, 93, 125, 125, 125, 125, 10
    //         ]
    //     );
    // }
}
