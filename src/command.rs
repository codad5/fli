// command.rs
use std::collections::HashMap;

use crate::option_parser::{CommandChain, CommandOptionsParser, CommandOptionsParserBuilder, InputArgsParser, ValueTypes};

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

    pub fn run(&self, mut arg_parser: InputArgsParser) -> Result<(), String> {
        // Prepare the parser with this command's options
        arg_parser.prepare(self)?;
        
        let chain = arg_parser.get_parsed_commands_chain().clone();
        let mut chain_iter = chain.iter();
        
        // Collect arguments and check for subcommands
        let mut arguments = Vec::new();
        let mut next_subcommand: Option<(&String, Vec<CommandChain>)> = None;
        
        for (idx, item) in chain.iter().enumerate() {
            match item {
                CommandChain::SubCommand(sub_name) => {
                    // Found a subcommand, collect remaining chain items
                    let remaining: Vec<CommandChain> = chain[idx+1..].to_vec();
                    next_subcommand = Some((sub_name, remaining));
                    break;
                }
                CommandChain::Argument(arg) => {
                    arguments.push(arg.clone());
                }
                CommandChain::Option(_, _) => {
                    // Options are already processed, just skip
                }
            }
        }
        
        // If there's a subcommand, handle it recursively
        if let Some((sub_name, remaining_chain)) = next_subcommand {
            if let Some(sub_command) = self.get_sub_command(sub_name) {
                // Create a new parser for the subcommand
                let mut sub_parser = InputArgsParser::new(
                    sub_name.clone(),
                    Vec::new() // We'll reconstruct args from chain
                );
                sub_parser.command_chain = remaining_chain;
                
                return sub_command.run(sub_parser);
            } else {
                return Err(format!("Unknown subcommand: {}", sub_name));
            }
        }
        
        // No subcommand, execute this command's callback
        if let Some(callback) = self.get_callback() {
            let callback_data = FliCallbackData::new(
                self.clone(),
                self.get_option_parser(),
                arguments,
                arg_parser,
            );
            callback(&callback_data);
        }
        
        Ok(())
    }
}
