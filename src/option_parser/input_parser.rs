use std::default;

use super::parse_state::ParseState;
use super::value_types::{Value, ValueTypes};
use crate::command::FliCommand;
use crate::error::{FliError, Result};

/// Represents elements in the parsed command chain.
///
/// Each element describes what was encountered during parsing:
/// - A subcommand to execute
/// - An option with its value
/// - A positional argument
/// - A preserved option that triggers immediate callback
#[derive(Debug, Clone)]
pub enum CommandChain {
    /// A subcommand was encountered
    SubCommand(String),
    /// An option with its parsed value
    Option(String, ValueTypes),
    /// A positional argument
    Argument(String),
    /// A preserved option that should trigger immediate callback
    IsPreservedOption(String),
}

/// Parses command-line arguments into a structured command chain.
///
/// This is the core parsing engine that:
/// 1. Tokenizes raw arguments
/// 2. Identifies commands, options, and values
/// 3. Validates against expected option types
/// 4. Builds a chain representing the parse tree
///
/// # Examples
///
/// ```rust
/// let args = vec!["copy".to_string(), "-s".to_string(),
///                 "file1.txt".to_string(), "file2.txt".to_string()];
/// let mut parser = InputArgsParser::new("copy".to_string(), args);
/// parser.prepare(&mut command)?;
/// ```
#[derive(Debug, Clone)]
pub struct InputArgsParser {
    pub command: String,
    pub args: Vec<String>,
    pub command_chain: Vec<CommandChain>,
    is_prepared: bool,
}

impl InputArgsParser {
    /// Creates a new argument parser.
    ///
    /// # Arguments
    ///
    /// * `command` - The command name being parsed
    /// * `args` - The raw arguments (without program name)
    ///
    /// # Returns
    ///
    /// An unprepared parser (call `prepare()` before use)
    pub fn new(command: String, args: Vec<String>) -> Self {
        Self {
            command,
            args,
            command_chain: Vec::new(),
            is_prepared: false,
        }
    }

    /// Returns the parsed command chain.
    ///
    /// # Returns
    ///
    /// Reference to the vector of parsed `CommandChain` elements
    ///
    /// # Note
    ///
    /// Only valid after `prepare()` has been called.
    pub fn get_parsed_commands_chain(&self) -> &Vec<CommandChain> {
        &self.command_chain
    }

    /// Parses arguments and validates them against the command definition.
    ///
    /// This is the main parsing method that:
    /// 1. Validates command name matches
    /// 2. Iterates through arguments
    /// 3. Handles state transitions (option -> value -> next option)
    /// 4. Validates required values are provided
    /// 5. Handles special cases like "--" and preserved options
    ///
    /// # Arguments
    ///
    /// * `command` - The command definition with expected options
    ///
    /// # Returns
    ///
    /// * `Ok(&mut Self)` - If parsing succeeded
    /// * `Err(String)` - If parsing failed with error description
    ///
    /// # Errors
    ///
    /// Returns errors for:
    /// - Command name mismatch
    /// - Missing required option values
    /// - Unexpected "--" placement
    /// - Invalid option-value combinations
    /// - Unknown options or subcommands
    ///
    /// # State Machine
    ///
    /// Uses `ParseState` to track current parsing context:
    /// - `Start` -> `InCommand`: After verifying command name
    /// - `InCommand` -> `InOption`: When encountering an option flag
    /// - `InOption` -> `AcceptingValue`: For options that need values
    /// - `AcceptingValue` -> `InOption`: After consuming value(s)
    /// - Any -> `Breaking`: When encountering "--"
    /// - Any -> `End`: When parsing completes
    pub fn prepare(&mut self, command: &mut FliCommand) -> Result<&mut Self> {
        if self.is_prepared {
            println!("Parser is already prepared");
            return Ok(self);
        }
        let mut state = ParseState::Start;

        // Verify command name matches
        if self.command != *command.get_name() {
            return Err(FliError::command_mismatch(
                command.get_name(),
                &self.command,
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
                        return Err(FliError::UnexpectedToken {
                            token: "--".to_string(),
                            position: i,
                        });
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
                    ValueTypes::RequiredSingle(default) => {
                        // Check if next arg is another option (error case)
                        if command.get_option_parser().has_option(arg) {
                            return Err(FliError::missing_value(option_name));
                        }

                        let value = default.clone().replace_with_expected_value(arg)?;
                        // Assign the value
                        self.command_chain.push(CommandChain::Option(
                            option_name.clone(),
                            ValueTypes::RequiredSingle(value.clone()),
                        ));
                        command.get_option_parser().update_option_value(
                            option_name,
                            ValueTypes::RequiredSingle(value),
                        )?;
                        state.set_next_mode(ParseState::InOption)?;
                        i += 1;
                        continue;
                    }
                    ValueTypes::OptionalSingle(default) => {
                        // If next arg is an option, don't consume it as value
                        if command.get_option_parser().has_option(arg) {
                            self.command_chain.push(CommandChain::Option(
                                option_name.clone(),
                                ValueTypes::OptionalSingle(None),
                            ));
                            state.set_next_mode(ParseState::InOption)?;
                            continue; // Don't increment i, process this arg as option
                        }

                        let value = default
                            .clone()
                            .unwrap_or(Value::Str(String::new()))
                            .replace_with_expected_value(arg)?;

                        // Otherwise, consume as value
                        self.command_chain.push(CommandChain::Option(
                            option_name.clone(),
                            ValueTypes::OptionalSingle(Some(value.clone())),
                        ));
                        command.get_option_parser().update_option_value(
                            option_name,
                            ValueTypes::OptionalSingle(Some(value)),
                        )?;
                        state.set_next_mode(ParseState::InOption)?;
                        i += 1;
                        continue;
                    }
                    ValueTypes::RequiredMultiple(default, expected_count) => {
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

                            let current_value = default
                                .get(values.len())
                                .cloned()
                                .unwrap_or(Value::Str(String::new()))
                                .replace_with_expected_value(current_arg)?;

                            values.push(current_value);
                            i += 1;
                        }

                        // Validate we got at least one value
                        if values.is_empty() {
                            return Err(FliError::missing_value(option_name));
                        }

                        // Validate expected count if specified
                        if let Some(expected) = expected_count {
                            if values.len() != *expected {
                                return Err(FliError::missing_value(option_name));
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
                    ValueTypes::OptionalMultiple(default, expected_count) => {
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

                            let current_value = if default.is_some() {
                                default
                                    .as_ref()
                                    .unwrap()
                                    .get(values.len())
                                    .cloned()
                                    .unwrap_or(Value::Str(String::new()))
                                    .replace_with_expected_value(current_arg)?
                            } else {
                                Value::Str(String::new())
                                    .replace_with_expected_value(current_arg)?
                            };

                            values.push(current_value);
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
                        return Err(FliError::Internal(
                            "AcceptingValue state with None value type".to_string(),
                        ));
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
                    .ok_or_else(|| FliError::OptionNotFound(arg.to_string()))?;

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

            if command.get_expected_positional_args() <= 0
                && (matches!(state, ParseState::Start | ParseState::InCommand)
                    || matches!(self.command_chain.last(), Some(CommandChain::SubCommand(_))))
            {
                let available: Vec<String> = command.get_sub_commands().keys().cloned().collect();
                return Err(FliError::UnknownCommand(arg.to_string(), available));
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
                    return Err(FliError::missing_value(option_name));
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

    /// Creates a parser from an existing command chain.
    ///
    /// Used internally for subcommand handling.
    ///
    /// # Arguments
    ///
    /// * `command` - Command name
    /// * `chain` - Pre-built command chain
    ///
    /// # Returns
    ///
    /// A parser marked as already prepared
    pub fn from_chain(command: String, chain: Vec<CommandChain>) -> Self {
        Self {
            command,
            args: Vec::new(),
            command_chain: chain,
            is_prepared: true,
        }
    }

    /// Creates a new parser with remaining chain elements after an index.
    ///
    /// Used for passing control to subcommands.
    ///
    /// # Arguments
    ///
    /// * `start_idx` - Index to start slicing from
    ///
    /// # Returns
    ///
    /// New parser with remaining chain elements
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

    /// Returns the command name being parsed.
    pub fn get_command(&self) -> &String {
        &self.command
    }

    /// Returns the full command chain.
    ///
    /// Alias for `get_parsed_commands_chain()` for convenience.
    pub fn get_command_chain(&self) -> &Vec<CommandChain> {
        &self.command_chain
    }
}
