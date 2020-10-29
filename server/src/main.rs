use std::net::UdpSocket;
use std::str;

mod commands;
use commands::{match_command, run_command, CommandList};

#[allow(unused_variables)]
#[allow(unused_assignments)]
fn main() {
    let localaddr: &str = "127.0.0.1:31499";
    let socket = UdpSocket::bind(localaddr).expect("");
    println!("Listening on {}...", localaddr);

    let command_received = "";

    loop {
        let mut buf = [0; 1000];
        let command_splited: Vec<&str>;

        let (number_of_bytes, src_addr) = socket.recv_from(&mut buf).expect("");
        let data_recv: &[u8] = &mut buf[..number_of_bytes];
        let command_received: &str = match str::from_utf8(data_recv) {
            Ok(v) => v,
            Err(e) => panic!("Invalid utf8 seq : {}", e),
        };
        let retmatch = match match_command(command_received) {
            CommandList::Error(e) => panic!("{}", e),
            _ => match_command(command_received),
        };

        let cmd = match retmatch.clone() {
            CommandList::Exec(st) => CommandList::Exec(st),
            CommandList::Kill => CommandList::Kill,
            CommandList::Error(e) => CommandList::Error(e),
        };

        let cmd2 = match retmatch.clone() {
            CommandList::Exec(st) => CommandList::Exec(st),
            CommandList::Kill => CommandList::Kill,
            CommandList::Error(m) => CommandList::Error(m),
        };

        match run_command(cmd2).len() {
            0 => (),
            _ => {
                socket
                    .send_to(&run_command(cmd), src_addr)
                    .expect("Failed to send output");
            }
        }
    }
}
