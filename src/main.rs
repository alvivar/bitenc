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

    conn.try_write_message(b"s maybe Delete me");
    conn.try_write_message(b"d maybe");
    conn.try_write_message(b"s? maybe Maybe?");
    conn.try_write_message(b"g maybe");
    conn.try_write_message(b"s? maybe Other thing?");
    conn.try_write_message(b"g maybe");

    conn.try_write_message(b"+1 inc");
    conn.try_write_message(b"g inc");
    conn.try_write_message(b"+1 inc");
    conn.try_write_message(b"g inc");

    conn.try_write_message(b"#g sub");
    conn.try_write_message(b"#j sub");
    conn.try_write_message(b"#k sub");
    conn.try_write_message(b"s sub Sub sent by set!");
    conn.try_write_message(b"! sub Sub sent by !");

    conn.try_write_message(b"s append Half");
    conn.try_write_message(b"+ append -");
    conn.try_write_message(b"+ append Life");

    conn.try_write_message(b"js");
    conn.try_write_message(b"k");

    sleep(Duration::from_secs(1));
    let response = conn.try_read().unwrap();
    println!("{:?}\n", response);
    println!("{}", String::from_utf8_lossy(&response));
    sleep(Duration::from_secs(1));
}
