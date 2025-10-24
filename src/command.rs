// command.rs
use std::collections::HashMap;

use crate::option_parser::{
    CommandChain, CommandOptionsParser, CommandOptionsParserBuilder, InputArgsParser, ValueTypes,
};

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

    pub fn get_option_value(&self, name: &str) -> Option<&ValueTypes> {
        if name.starts_with("-") {
            return self.option_parser.get_option_expected_value_type(name);
        }

        // try single-dash prefix
        let short = format!("-{}", name);
        if let Some(val) = self.option_parser.get_option_expected_value_type(&short) {
            return Some(val);
        }

        // try double-dash prefix
        let long = format!("--{}", name);
        if let Some(val) = self.option_parser.get_option_expected_value_type(&long) {
            return Some(val);
        }

        // fallback to raw name
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
pub struct PreservedOption {
    pub long_flag: String,
    pub short_flag: String,
    pub value_type: ValueTypes,
    pub callback: fn(&FliCallbackData),
}

#[derive(Debug, Clone)]
pub struct FliCommand {
    pub name: String,
    pub description: String,
    // pub arg_parser: InputArgsParser,
    pub option_parser_builder: CommandOptionsParserBuilder,
    pub sub_commands: HashMap<String, FliCommand>,
    pub callback: Option<fn(&FliCallbackData)>,
    pub preserved_options: Vec<PreservedOption>,
    pub preserved_short_flags: HashMap<String, usize>, // map short flag to index in preserved_options
    pub preserved_long_flags: HashMap<String, usize>, // map long flag to index in preserved_options
}

impl FliCommand {
    pub fn new(name: &str, description: &str) -> Self {
        let mut x = Self {
            name: name.to_owned(),
            description: description.to_owned(),
            // arg_parser: InputArgsParser::new(name.to_string(), Vec::new()),
            sub_commands: HashMap::new(),
            callback: None,
            option_parser_builder: CommandOptionsParserBuilder::new(),
            preserved_options: Vec::new(),
            preserved_short_flags: HashMap::new(),
            preserved_long_flags: HashMap::new(),
        };
        x.setup_help_flag();
        x
    }

    pub fn setup_help_flag(&mut self) {
        self.add_option_with_callback(
            "help",
            "Show help information",
            "-h",
            "--help",
            ValueTypes::None,
            |data| {
                println!("Help for command: {}", data.get_command().get_name());
                println!("{}", data.get_command().get_description());
                println!("Options:");
                for x in data.option_parser.get_options() {
                    println!(
                        "  {} / {} : {}",
                        x.short_flag, x.long_flag, x.description
                    );
                }
                if data.get_command().has_sub_commands() {
                    println!("Subcommands:");
                    for (sub_name, sub_cmd) in data.get_command().get_sub_commands() {
                        println!("  {} : {}", sub_name, sub_cmd.get_description());
                    }
                }
            },
        );
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

    pub fn get_option_parser(&mut self) -> &mut CommandOptionsParser {
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

    pub fn get_sub_command_mut(&mut self, name: &str) -> Option<&mut FliCommand> {
        self.sub_commands.get_mut(name)
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

    pub fn add_option_with_callback(
        &mut self,
        name: &str,
        description: &str,
        short_flag: &str,
        long_flag: &str,
        value: ValueTypes,
        callback: fn(&FliCallbackData),
    ) -> &mut Self {
        // register option with the normal option parser builder (clone value for the builder)
        self.option_parser_builder.add_option(
            name,
            description,
            short_flag,
            long_flag,
            value.clone(),
        );

        // create preserved option that will trigger the provided callback when encountered
        let preserved = PreservedOption {
            long_flag: long_flag.to_string(),
            short_flag: short_flag.to_string(),
            value_type: value,
            callback,
        };

        // record index and maps for quick lookup
        let idx = self.preserved_options.len();
        if !preserved.short_flag.is_empty() {
            self.preserved_short_flags
                .insert(preserved.short_flag.clone(), idx);
        }
        if !preserved.long_flag.is_empty() {
            self.preserved_long_flags
                .insert(preserved.long_flag.clone(), idx);
        }

        self.preserved_options.push(preserved);
        self
    }

    pub fn get_preserved_option(&self, name: &str) -> Option<&PreservedOption> {
        // try exact as provided (direct lookups on self to ensure correct lifetimes)
        if let Some(idx) = self.preserved_short_flags.get(name) {
            return self.preserved_options.get(*idx);
        }
        if let Some(idx) = self.preserved_long_flags.get(name) {
            return self.preserved_options.get(*idx);
        }

        // normalize by trimming existing dashes and try common variants
        let trimmed = name.trim_start_matches('-');
        let variants = [
            format!("-{}", trimmed),
            format!("--{}", trimmed),
            trimmed.to_string(),
        ];

        for v in &variants {
            if let Some(idx) = self.preserved_short_flags.get(v.as_str()) {
                return self.preserved_options.get(*idx);
            }
            if let Some(idx) = self.preserved_long_flags.get(v.as_str()) {
                return self.preserved_options.get(*idx);
            }
        }

        None
    }

    pub fn has_preserved_option(&self, name: &str) -> bool {
        self.get_preserved_option(name).is_some()
    }

    pub fn subcommand(&mut self, name: &str, description: &str) -> &mut FliCommand {
        let command = FliCommand::new(name, description);
        self.add_sub_command(command);
        self.sub_commands.get_mut(name).unwrap()
    }

    pub fn add_sub_command(&mut self, command: FliCommand) {
        self.sub_commands
            .insert(command.get_name().to_owned(), command);
    }

    pub fn get_callback(&self) -> Option<fn(&FliCallbackData)> {
        self.callback
    }

    pub fn run(&mut self, mut arg_parser: InputArgsParser) -> Result<(), String> {
        // Prepare the parser with this command's options
        arg_parser.prepare(self)?;

        println!("Parsed arguments: {:?}", arg_parser.get_command_chain());

        let chain = arg_parser.get_parsed_commands_chain().clone();
        let mut chain_iter = chain.iter();

        // Collect arguments and check for subcommands
        let mut arguments = Vec::new();
        let mut next_subcommand: Option<(&String, Vec<CommandChain>, usize)> = None;
        let mut preserved_option: Option<&String> = None;

        for (idx, item) in chain.iter().enumerate() {
            match item {
                CommandChain::SubCommand(sub_name) => {
                    // Found a subcommand, collect remaining chain items
                    let remaining: Vec<CommandChain> = chain[idx + 1..].to_vec();
                    next_subcommand = Some((sub_name, remaining, idx));
                    break;
                }
                CommandChain::Argument(arg) => {
                    arguments.push(arg.clone());
                }
                CommandChain::Option(_, _) => {
                    // Options are already processed, just skip
                }
                CommandChain::IsPreservedOption(s) => {
                    // Preserved options are already processed, just skip
                    preserved_option = Some(s);
                }
            }
        }

        // If there's a subcommand, handle it recursively
        if let Some((sub_name, remaining_chain, idx)) = next_subcommand {
            if let Some(sub_command) = self.get_sub_command_mut(sub_name) {
                // Create a new parser for the subcommand
                let mut sub_parser = arg_parser.with_remaining_chain(idx);
                sub_parser.command_chain = remaining_chain;

                return sub_command.run(sub_parser);
            } else {
                return Err(format!("Unknown subcommand: {}", sub_name));
            }
        }

        let mut callback: Option<fn(&FliCallbackData)> = None;
        let callback_data = FliCallbackData::new(
            self.clone(),
            self.get_option_parser().clone(),
            arguments,
            arg_parser,
        );

        // No subcommand, execute this command's callback
        if let Some(_callback) = self.get_callback() {
            callback = Some(_callback);
        }

        if let Some(preserved_name) = preserved_option {
            if let Some(preserved) = self.get_preserved_option(preserved_name) {
                // Execute the preserved option's callback
                callback = Some(preserved.callback);
            }
        }

        if let Some(cb) = callback {
            cb(&callback_data);
        }

        Ok(())
    }
}
