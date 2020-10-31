use std::io::prelude::*;
use std::io::{self, Write};
use std::net::TcpStream;

fn main() {
    print!("Write the target IP>");
    io::stdout().flush().unwrap();
    let mut server_ip = String::new();
    io::stdin()
        .read_line(&mut server_ip)
        .expect("Failed to read server IP");
    let server: &str = server_ip.trim();

    let mut socket = TcpStream::connect(server)
        .expect("Cannot connect to ip. Verify if the server is installed on the target machine");

    let mut command = String::new();
    loop {
        command.clear();
        print!("remote@{} $ ", server);
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read command. please retry");
        let trimmed_cmd: &str = command.trim();
        if trimmed_cmd == "exit" {
            break;
        }
        socket
            .write(trimmed_cmd.as_bytes())
            .expect("Could not send command");

        let mut buf = [0; 1024];
        let number_of_bytes = socket.read(&mut buf).unwrap();
        let filled = &mut buf[..number_of_bytes];
        println!("{}", std::str::from_utf8(&filled).unwrap());
    }
}
