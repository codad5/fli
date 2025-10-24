use super::parse_state::ParseState;
use super::value_types::{Value, ValueTypes};
use crate::command::FliCommand;

#[derive(Debug, Clone)]
pub enum CommandChain {
    SubCommand(String),
    Option(String, ValueTypes),
    Argument(String),
    IsPreservedOption(String),
}

#[derive(Debug, Clone)]
pub struct InputArgsParser {
    pub command: String,
    pub args: Vec<String>,
    pub command_chain: Vec<CommandChain>,
    is_prepared: bool,
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
            is_prepared: false,
        }
    }

    pub fn get_parsed_commands_chain(&self) -> &Vec<CommandChain> {
        &self.command_chain
    }

    pub fn prepare(&mut self, command: &mut FliCommand) -> Result<&mut Self, String> {
        if self.is_prepared {
            println!("Parser is already prepared");
            return Ok(self);
        }
        let mut state = ParseState::Start;

        // Verify command name matches
        if self.command != *command.get_name() {
            return Err(format!(
                "Command name mismatch: expected '{}', got '{}'",
                command.get_name(),
                self.command
            ));
        }

        // Move to InCommand state after verifying command
        state.set_next_mode(ParseState::InCommand)?;

        let mut i = 0;
        while i < self.args.len() {
            let arg = &self.args[i].clone();

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
                self.command_chain
                    .push(CommandChain::Argument(arg.to_string()));
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
                        command.get_option_parser().update_option_value(
                            option_name,
                            ValueTypes::RequiredSingle(Value::Str(arg.to_string())),
                        )?;
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
                        command.get_option_parser().update_option_value(
                            option_name,
                            ValueTypes::OptionalSingle(Some(Value::Str(arg.to_string()))),
                        )?;
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
                                    option_name,
                                    expected,
                                    values.len()
                                ));
                            }
                        }

                        self.command_chain.push(CommandChain::Option(
                            option_name.clone(),
                            ValueTypes::RequiredMultiple(values.clone(), *expected_count),
                        ));
                        command.get_option_parser().update_option_value(
                            option_name,
                            ValueTypes::RequiredMultiple(values, *expected_count),
                        )?;
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
                            option_value.clone(),
                        ));
                        command
                            .get_option_parser()
                            .update_option_value(option_name, option_value)?;
                        state.set_next_mode(ParseState::InOption)?;
                        continue; // Don't increment i again
                    }
                    ValueTypes::None => {
                        // This shouldn't happen as None options don't accept values
                        return Err(
                            "Internal error: AcceptingValue state with None value type".to_string()
                        );
                    }
                }
            }

            if let Some(preserved_option) = command.get_preserved_option(arg) {
                // It's a preserved option
                self.command_chain
                    .push(CommandChain::IsPreservedOption(arg.to_string()));
                state.set_next_mode(ParseState::InOption)?;
                i += 1;
                continue;
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
                        self.command_chain
                            .push(CommandChain::Option(arg.to_string(), ValueTypes::None));
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
            if let Some(command) = command.get_sub_command_mut(arg) {
                // self.command_chain
                //     .push(CommandChain::SubCommand(arg.to_string()));
                // state.set_next_mode(ParseState::InCommand)?;
                // i += 1;
                // continue;
                // if a sub command update the self.args to be the remaining args, then return self.prepare and pass the sub command
                self.command_chain
                    .push(CommandChain::SubCommand(arg.to_string()));
                let remaining_args = self.args[i + 1..].to_vec();
                self.command = arg.to_string();
                self.args = remaining_args;
                return self.prepare(command);
            }

            // Otherwise, it's an argument
            self.command_chain
                .push(CommandChain::Argument(arg.to_string()));
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
                            ValueTypes::OptionalMultiple(_, count) => {
                                ValueTypes::OptionalMultiple(None, *count)
                            }
                            _ => unreachable!(),
                        },
                    ));
                }
                _ => {}
            }
        }

        state.set_next_mode(ParseState::End)?;
        self.is_prepared = true;
        Ok(self)
    }

    pub fn from_chain(command: String, chain: Vec<CommandChain>) -> Self {
        Self {
            command,
            args: Vec::new(),
            command_chain: chain,
            is_prepared: true,
        }
    }

    /// Clone parser with remaining chain after index
    pub fn with_remaining_chain(&self, start_idx: usize) -> Self {
        let remaining_chain = if start_idx < self.command_chain.len() {
            self.command_chain[start_idx..].to_vec()
        } else {
            Vec::new()
        };

        Self {
            command: self.command.clone(),
            args: Vec::new(),
            command_chain: remaining_chain,
            is_prepared: true,
        }
    }

    pub fn get_command(&self) -> &String {
        &self.command
    }
    
    pub fn get_command_chain(&self) -> &Vec<CommandChain> {
        &self.command_chain
    }
}
