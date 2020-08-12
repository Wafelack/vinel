use std::net::UdpSocket;
use std::str;

#[allow(unused_variables)]
fn main() {
    let socket = UdpSocket::bind("127.0.0.1:60000").expect("");
    let mut buf = [0; 1000];

    loop {
        let (number_of_bytes, src_addr) = socket.recv_from(&mut buf).expect("");

        let data_recv = &mut buf[..number_of_bytes];
        let command_received = match str::from_utf8(data_recv) {
            Ok(v) => v,
            Err(e) => panic!("Invalid utf8 seq : {}", e),
        };
        println!("{}", command_received);
    }
}
