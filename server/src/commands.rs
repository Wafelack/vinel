use std::fs::File;
use std::io::{Error, ErrorKind};

pub enum CommandList {
    Display(String),
    CreateFile(String),
    CreateDirectory(String),
    Error(String),
}

pub fn match_command(command: &str) -> CommandList {
    let sliced: Vec<&str> = command.split(' ').collect();
    match sliced[0] {
        "display" => match sliced.len() {
            1 => return CommandList::Error(String::from("Usage: display <message>")),
            _ => {
                let mut to_display = String::new();
                for i in 1..sliced.len() {
                    to_display.push_str(sliced[i])
                }
                return CommandList::Display(to_display);
            }
        },
        "touch" => match sliced.len() {
            1 => return CommandList::Error(String::from("Usage: touch <filename>")),
            2 => return CommandList::CreateFile(sliced[1].into()),
            _ => return CommandList::Error(String::from("Usage: touch <filename>")),
        },
        "mkdir" => match sliced.len() {
            1 => return CommandList::Error(String::from("Usage: mkdir <dirname>")),
            2 => return CommandList::CreateDirectory(sliced[1].into()),
            _ => return CommandList::Error(String::from("Usage: mkdir <dirname>")),
        },
        &_ => return CommandList::Error(String::from("Not in commands")),
    }
}

fn newfile(filename: String) -> std::io::Result<()> {
    File::create(filename)?;
    Ok(())
}
fn newdir(dirname: String) -> std::io::Result<()> {
    std::fs::create_dir(dirname)?;
    Ok(())
}

pub fn run_command(command: CommandList) -> std::io::Result<()> {
    match command {
        CommandList::Display(s) => {
            println!("{}", s);
            return Ok(());
        }
        CommandList::CreateFile(filename) => match newfile(filename) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        },
        CommandList::CreateDirectory(dirname) => match newdir(dirname) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        },
        CommandList::Error(e) => return Err(Error::new(ErrorKind::Other, e)),
    }
}
