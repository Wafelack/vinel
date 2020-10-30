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
