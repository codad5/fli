use crate::{
    command::{FliCallbackData, FliCommand},
    option_parser::{InputArgsParser, ValueTypes},
};

// app.rs
pub struct Fli {
    pub name: String,
    pub version: String,
    pub description: String,
    pub root_command: FliCommand, // this is like a normal command but the command is an empty string
}

impl Fli {
    // to get a new command
    pub fn command(&mut self, name: &str, description: &str) -> &mut FliCommand {
        let command = FliCommand::new(name, description);
        self.add_command(command);
        self.root_command.sub_commands.get_mut(name).unwrap()
    }

    pub fn add_command(&mut self, command: FliCommand) {
        self.root_command.add_sub_command(command);
    }

    pub fn set_callback(&mut self, callback: fn(&FliCallbackData)) {
        self.root_command.set_callback(callback);
    }

    pub fn add_option(
        &mut self,
        name: &str,
        description: &str,
        short_flag: &str,
        long_flag: &str,
        value: ValueTypes,
    ) {
        self.root_command
            .add_option(name, description, short_flag, long_flag, value);
    }
}

impl Fli {
    pub fn new(name: &str, version: &str, description: &str) -> Self {
        Self {
            name: name.to_owned(),
            version: version.to_owned(),
            description: description.to_owned(),
            root_command: FliCommand::new("", description),
        }
    }

    pub fn run(&mut self) {
        let args: Vec<String> = std::env::args().collect();

        // Skip the program name
        let command_args = if args.len() > 1 {
            args[1..].to_vec()
        } else {
            Vec::new()
        };

        let parser =
            InputArgsParser::new(self.root_command.get_name().to_string(), command_args);

        match self.root_command.run(parser) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}
