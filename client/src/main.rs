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

    let mut socket = match TcpStream::connect(server) {
        Ok(s) => {
            println!("[+] Succesfully connected to {}", server);
            s
        }
        Err(_e) => {
            println!("Cannot resolve server");
            std::process::exit(-66);
        }
    };

    let mut command = String::new();
    loop {
        command.clear();
        print!("remote@{} $ ", server);
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut command) {
            Ok(_s) => (),
            Err(_e) => {
                println!("Failed to read line, please retry");
                continue;
            }
        }
        let trimmed_cmd: &str = command.trim();
        if trimmed_cmd == "exit" {
            break;
        }
        match socket.write(trimmed_cmd.as_bytes()) {
            Ok(_s) => (),
            Err(_e) => {
                println!("Failed to send command. Please retry");
                continue;
            }
        }

        let mut buf = [0; 1024];
        let number_of_bytes = socket.read(&mut buf).unwrap();
        let filled = &mut buf[..number_of_bytes];
        println!("{}", std::str::from_utf8(&filled).unwrap());
    }
}
