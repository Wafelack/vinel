use std::process::Command;

#[derive(Clone)]
pub enum CommandList {
    Exec(String),
    Error(String),
    Kill,
}

pub fn match_command(command: &str) -> CommandList {
    let sliced: Vec<&str> = command.split(' ').collect();
    match sliced[0] {
        "exec" => {
            let mut concat: String = String::new();

            for i in 1..sliced.len() {
                concat.push_str(sliced[i]);
                concat.push_str(" ");
            }

            CommandList::Exec(concat)
        }
        "kill" => CommandList::Kill,
        &_ => return CommandList::Error(String::from("Not in commands")),
    }
}

pub fn run_command(command: CommandList) -> Vec<u8> {
    match command {
        CommandList::Exec(command) => {
            let output = Command::new("cmd")
                .args(&["/C", &command])
                .output()
                .expect("failed to run command");
            return output.stdout;
        }
        CommandList::Kill => {
            std::process::exit(0);
        }
        CommandList::Error(_e) => {
            std::process::exit(-1);
        }
    }
}
