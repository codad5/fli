// command.rs
use std::collections::HashMap;

use crate::option_parser::{
    CommandOptionsParser, CommandOptionsParserBuilder, InputArgsParser, ValueTypes,
};

pub struct FliCallbackData {
    pub command: FliCommand,
    pub option_parser: CommandOptionsParser,
    pub arguments: Vec<String>,
    pub arg_parser: InputArgsParser,
}


impl FliCallbackData {
    pub fn new(
        command: FliCommand,
        option_parser: CommandOptionsParser,
        arguments: Vec<String>,
        arg_parser: InputArgsParser,
    ) -> Self {
        Self {
            command,
            option_parser,
            arguments,
            arg_parser,
        }
    }

    pub fn get_option(&self, name: &str) -> Option<&ValueTypes> {
        self.option_parser.get_option_expected_value_type(name)
    }

    pub fn get_argument_at(&self, index: usize) -> Option<&String> {
        self.arguments.get(index)
    }

    pub fn get_arguments(&self) -> &Vec<String> {
        &self.arguments
    }

    pub fn get_command(&self) -> &FliCommand {
        &self.command
    }

    pub fn get_arg_parser(&self) -> &InputArgsParser {
        &self.arg_parser
    }
}

pub struct FliCommand {
    pub name: String,
    pub description: String,
    // pub arg_parser: InputArgsParser,
    pub option_parser_builder: CommandOptionsParserBuilder,
    pub sub_commands: HashMap<String, FliCommand>,
    pub callback: Option<fn(&FliCallbackData)>,
}

impl FliCommand {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_owned(),
            description: description.to_owned(),
            // arg_parser: InputArgsParser::new(name.to_string(), Vec::new()),
            sub_commands: HashMap::new(),
            callback: None,
            option_parser_builder: CommandOptionsParserBuilder::new(),
        }
    }

    pub fn set_callback(&mut self, callback: fn(&FliCallbackData)) {
        self.callback = Some(callback);
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_description(&self) -> &String {
        &self.description
    }


    pub fn get_option_parser_builder(&self) -> &CommandOptionsParserBuilder {
        &self.option_parser_builder
    }

    pub fn get_option_parser(&self) -> CommandOptionsParser {
        self.option_parser_builder.build()
    }

    pub fn get_sub_commands(&self) -> &HashMap<String, FliCommand> {
        &self.sub_commands
    }

    pub fn has_sub_commands(&self) -> bool {
        !self.sub_commands.is_empty()
    }

    pub fn get_sub_command(&self, name: &str) -> Option<&FliCommand> {
        self.sub_commands.get(name)
    }

    pub fn has_sub_command(&self, name: &str) -> bool {
        self.sub_commands.contains_key(name)
    }

    pub fn add_option(
        &mut self,
        name: &str,
        description: &str,
        short_flag: &str,
        long_flag: &str,
        value: ValueTypes,
    ) -> &mut Self {
        self.option_parser_builder
            .add_option(name, description, short_flag, long_flag, value);
        self
    }

    pub fn add_sub_command(&mut self, command: FliCommand) {
        self.sub_commands
            .insert(command.get_name().to_owned(), command);
    }

    pub fn get_callback(&self) -> Option<fn(&FliCallbackData)> {
        self.callback
    }

    pub fn run(&self, arg_parser: &InputArgsParser) {
        // need something like this

        for command_chain in arg_parser.get_parsed_commands_chain() {
            // if the first chain is command then it is a sub command get it if it exists call the run method recursively passing the arg_parser but remove the first chain, that means it has to be iter so we can maybe call something like `arg_parser.get_remaining_commands_chain()` or we can clone and remove the first element
        }
    }
}
