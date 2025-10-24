// option_parser.rs
use std::collections::HashMap;

use crate::command::FliCommand;

#[derive(Debug, Clone)]
pub enum Value {
    Str(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}
#[derive(Debug, Clone)]
pub enum ValueTypes {
    RequiredSingle(Value),
    OptionalSingle(Option<Value>),
    RequiredMultiple(Vec<Value>, Option<usize>), // optional expected value count
    OptionalMultiple(Option<Vec<Value>>, Option<usize>), // optional expected value count
    None,
}

#[derive(Debug, Clone)]
pub struct SingleOption {
    pub name: String,
    pub description: String,
    pub short_flag: String, // with -
    pub long_flag: String,  // with --
    pub value: ValueTypes,
}

// contains all options for a command
#[derive(Debug, Clone)]
pub struct CommandOptionsParser {
    pub options: Vec<SingleOption>,
    short_option_map: HashMap<String, usize>, // map short flag to index in options vec
    long_option_map: HashMap<String, usize>,  // map long
}

#[derive(Debug, Clone)]
pub enum InputType {
    Command(String),
    Option(ValueTypes),
    Argument(Vec<String>),
}

#[derive(Debug, Clone)]
pub enum CommandChain {
    SubCommand(String),
    Option(String, ValueTypes),
    Argument(String),
}

#[derive(Debug, Clone)]
pub struct InputArgsParser {
    pub command: String,   // this is the command that was called the first arg
    pub args: Vec<String>, // the args with the command removed
    pub command_chain: Vec<CommandChain>, // the chain of commands , options and arguments
}

#[derive(Debug, Clone)]
pub enum ParseState {
    Start,
    InCommand,
    InOption,
    AcceptingValue(String, ValueTypes), // option name , expected value type
    // a state to represent breaking using -- to stop option parsing
    Breaking,
    InArgument,
    End,
}

impl ParseState {
    pub fn set_next_mode(&mut self, next: ParseState) -> Result<&mut Self, String> {
        if self.can_go_to_next(&next) {
            *self = next;
            Ok(self)
        } else {
            Err(format!("Cannot transition from {:?} to {:?}", self, next))
        }
    }

    pub fn can_go_to_next(&self, next: &ParseState) -> bool {
        match next {
            ParseState::Start => false, // cannot go back to start
            
            // can only go to command from start or another command (subcommands)
            ParseState::InCommand => matches!(self, ParseState::Start | ParseState::InCommand),
            
            // can go to option from: start, command, after accepting value, or another option
            ParseState::InOption => matches!(
                self, 
                ParseState::Start | ParseState::InCommand | ParseState::InArgument | ParseState::InOption | ParseState::AcceptingValue(_, _)
            ),
            
            // can only go to accepting value from option
            ParseState::AcceptingValue(_, _) => matches!(self, ParseState::InOption),
            
            // can go to argument from: start, command, breaking, or another argument
            ParseState::InArgument => matches!(
                self, 
                ParseState::Start | ParseState::InCommand | ParseState::Breaking | ParseState::InArgument
            ),
            
            // can go to breaking from: option or accepting value
            ParseState::Breaking => matches!(self, ParseState::InOption | ParseState::AcceptingValue(_, _)),
            
            // can go to end from any state except Start
            ParseState::End => !matches!(self, ParseState::Start),
        }
    }
}

impl InputArgsParser {
    pub fn new(command: String, args: Vec<String>) -> Self {
        // let command = args.get(0).cloned().unwrap_or_default();
        // let args = if args.len() > 1 {
        //     args[1..].to_vec()
        // } else {
        //     Vec::new()
        // };

        Self {
            command,
            args,
            command_chain: Vec::new(),
        }
    }

    pub fn get_parsed_commands_chain(&self) -> &Vec<CommandChain> {
        &self.command_chain
    }

    pub fn prepare(&mut self, command: &FliCommand) -> Result<&mut Self, String> {
    let mut state = ParseState::Start;
    
    // Verify command name matches
    if self.command != *command.get_name() {
        return Err(format!("Command name mismatch: expected '{}', got '{}'", 
            command.get_name(), self.command));
    }
    
    // Move to InCommand state after verifying command
    state.set_next_mode(ParseState::InCommand)?;
    
    let mut i = 0;
    while i < self.args.len() {
        let arg = &self.args[i];
        
        // Handle the break symbol "--"
        if arg == "--" {
            match state {
                ParseState::InOption | ParseState::AcceptingValue(_, _) => {
                    state.set_next_mode(ParseState::Breaking)?;
                    i += 1;
                    continue;
                }
                _ => {
                    return Err(format!("Unexpected '--' at position {}", i));
                }
            }
        }
        
        // If we're in Breaking state, everything after is an argument
        if matches!(state, ParseState::Breaking) {
            self.command_chain.push(CommandChain::Argument(arg.to_string()));
            state.set_next_mode(ParseState::InArgument)?;
            i += 1;
            continue;
        }
        
        // Handle AcceptingValue state
        if let ParseState::AcceptingValue(option_name, expected_value_type) = &state {
            match expected_value_type {
                ValueTypes::RequiredSingle(_) => {
                    // Check if next arg is another option (error case)
                    if command.get_option_parser().has_option(arg) {
                        return Err(format!(
                            "Expected value for option '{}', but found another option: '{}'",
                            option_name, arg
                        ));
                    }
                    
                    // Assign the value
                    self.command_chain.push(CommandChain::Option(
                        option_name.clone(),
                        ValueTypes::RequiredSingle(Value::Str(arg.to_string())),
                    ));
                    state.set_next_mode(ParseState::InOption)?;
                    i += 1;
                    continue;
                }
                ValueTypes::OptionalSingle(_) => {
                    // If next arg is an option, don't consume it as value
                    if command.get_option_parser().has_option(arg) {
                        self.command_chain.push(CommandChain::Option(
                            option_name.clone(),
                            ValueTypes::OptionalSingle(None),
                        ));
                        state.set_next_mode(ParseState::InOption)?;
                        continue; // Don't increment i, process this arg as option
                    }
                    
                    // Otherwise, consume as value
                    self.command_chain.push(CommandChain::Option(
                        option_name.clone(),
                        ValueTypes::OptionalSingle(Some(Value::Str(arg.to_string()))),
                    ));
                    state.set_next_mode(ParseState::InOption)?;
                    i += 1;
                    continue;
                }
                ValueTypes::RequiredMultiple(_, expected_count) => {
                    let mut values = Vec::new();
                    let max_count = expected_count.unwrap_or(usize::MAX);
                    
                    // Collect values until we hit an option or reach max count
                    while i < self.args.len() && values.len() < max_count {
                        let current_arg = &self.args[i];
                        
                        if command.get_option_parser().has_option(current_arg) {
                            break;
                        }
                        
                        if current_arg == "--" {
                            break;
                        }
                        
                        values.push(Value::Str(current_arg.to_string()));
                        i += 1;
                    }
                    
                    // Validate we got at least one value
                    if values.is_empty() {
                        return Err(format!(
                            "Option '{}' requires at least one value",
                            option_name
                        ));
                    }
                    
                    // Validate expected count if specified
                    if let Some(expected) = expected_count {
                        if values.len() != *expected {
                            return Err(format!(
                                "Option '{}' expected {} value(s), got {}",
                                option_name, expected, values.len()
                            ));
                        }
                    }
                    
                    self.command_chain.push(CommandChain::Option(
                        option_name.clone(),
                        ValueTypes::RequiredMultiple(values, *expected_count),
                    ));
                    state.set_next_mode(ParseState::InOption)?;
                    continue; // Don't increment i again, it's already advanced
                }
                ValueTypes::OptionalMultiple(_, expected_count) => {
                    let mut values = Vec::new();
                    let max_count = expected_count.unwrap_or(usize::MAX);
                    
                    // Collect values until we hit an option or reach max count
                    while i < self.args.len() && values.len() < max_count {
                        let current_arg = &self.args[i];
                        
                        if command.get_option_parser().has_option(current_arg) {
                            break;
                        }
                        
                        if current_arg == "--" {
                            break;
                        }
                        
                        values.push(Value::Str(current_arg.to_string()));
                        i += 1;
                    }
                    
                    let option_value = if values.is_empty() {
                        ValueTypes::OptionalMultiple(None, *expected_count)
                    } else {
                        ValueTypes::OptionalMultiple(Some(values), *expected_count)
                    };
                    
                    self.command_chain.push(CommandChain::Option(
                        option_name.clone(),
                        option_value,
                    ));
                    state.set_next_mode(ParseState::InOption)?;
                    continue; // Don't increment i again
                }
                ValueTypes::None => {
                    // This shouldn't happen as None options don't accept values
                    return Err("Internal error: AcceptingValue state with None value type".to_string());
                }
            }
        }
        
        // Check if current arg is an option
        let option_parser = command.get_option_parser();
        if option_parser.has_option(arg) {
            let expected_value_type = option_parser
                .get_option_expected_value_type(arg)
                .ok_or_else(|| format!("Failed to get value type for option: {}", arg))?;
            
            match expected_value_type {
                ValueTypes::None => {
                    // Flag option, no value needed
                    self.command_chain.push(CommandChain::Option(
                        arg.to_string(),
                        ValueTypes::None,
                    ));
                    state.set_next_mode(ParseState::InOption)?;
                }
                _ => {
                    // Option requires value(s), transition to AcceptingValue
                    state.set_next_mode(ParseState::InOption)?;
                    state.set_next_mode(ParseState::AcceptingValue(
                        arg.to_string(),
                        expected_value_type.clone(),
                    ))?;
                }
            }
            i += 1;
            continue;
        }
        
        // Check if it's a subcommand
        if command.has_sub_command(arg) {
            self.command_chain.push(CommandChain::SubCommand(arg.to_string()));
            state.set_next_mode(ParseState::InCommand)?;
            i += 1;
            continue;
        }
        
        // Otherwise, it's an argument
        self.command_chain.push(CommandChain::Argument(arg.to_string()));
        state.set_next_mode(ParseState::InArgument)?;
        i += 1;
    }
    
    // Final validation - check if we're expecting a required value
    if let ParseState::AcceptingValue(option_name, value_type) = &state {
        match value_type {
            ValueTypes::RequiredSingle(_) | ValueTypes::RequiredMultiple(_, _) => {
                return Err(format!(
                    "Missing required value for option: '{}'",
                    option_name
                ));
            }
            ValueTypes::OptionalSingle(_) | ValueTypes::OptionalMultiple(_, _) => {
                // It's optional, add it with None
                self.command_chain.push(CommandChain::Option(
                    option_name.clone(),
                    match value_type {
                        ValueTypes::OptionalSingle(_) => ValueTypes::OptionalSingle(None),
                        ValueTypes::OptionalMultiple(_, count) => ValueTypes::OptionalMultiple(None, *count),
                        _ => unreachable!(),
                    },
                ));
            }
            _ => {}
        }
    }
    
    state.set_next_mode(ParseState::End)?;
    Ok(self)
}

    
    pub fn get_command(&self) -> &String {
        &self.command
    }
}

impl CommandOptionsParser {
    pub fn new() -> Self {
        Self {
            options: Vec::new(),
            short_option_map: HashMap::new(),
            long_option_map: HashMap::new(),
        }
    }
    pub fn add_option(&mut self, option: SingleOption) {
        let index = self.options.len();
        self.short_option_map
            .insert(option.short_flag.clone(), index);
        self.long_option_map.insert(option.long_flag.clone(), index);
        self.options.push(option);
    }

    pub fn get_option_by_short_flag(&self, flag: &str) -> Option<&SingleOption> {
        match self.short_option_map.get(flag) {
            Some(&index) => self.options.get(index),
            None => None,
        }
    }

    pub fn get_option_by_long_flag(&self, flag: &str) -> Option<&SingleOption> {
        match self.long_option_map.get(flag) {
            Some(&index) => self.options.get(index),
            None => None,
        }
    }

    pub fn has_option(&self, flag: &str) -> bool {
        self.short_option_map.contains_key(flag) || self.long_option_map.contains_key(flag)
    }

    pub fn get_options(&self) -> &Vec<SingleOption> {
        &self.options
    }

    pub fn get_option_expected_value_type(&self, flag: &str) -> Option<&ValueTypes> {
        if let Some(option) = self.get_option_by_short_flag(flag) {
            return Some(&option.value);
        }
        if let Some(option) = self.get_option_by_long_flag(flag) {
            return Some(&option.value);
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct CommandOptionsParserBuilder {
    option_parser: CommandOptionsParser,
}

impl CommandOptionsParserBuilder {
    pub fn new() -> Self {
        Self {
            option_parser: CommandOptionsParser::new(),
        }
    }

    pub fn add_option(
        &mut self,
        name: &str,
        description: &str,
        short_flag: &str,
        long_flag: &str,
        value: ValueTypes,
    ) -> &mut Self {
        let option = SingleOption {
            name: name.to_owned(),
            description: description.to_owned(),
            short_flag: short_flag.to_owned(),
            long_flag: long_flag.to_owned(),
            value,
        };
        self.option_parser.add_option(option);
        self
    }

    pub fn build(&self) -> CommandOptionsParser {
        self.option_parser.clone()
    }
}
