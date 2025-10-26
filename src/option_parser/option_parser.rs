use super::value_types::ValueTypes;
use std::collections::HashMap;
use crate::error::{FliError, Result};
/// Represents a single command-line option with its configuration.
///
/// This stores both the definition (flags, description) and the parsed value.
#[derive(Debug, Clone)]
pub struct SingleOption {
    pub name: String,
    pub description: String,
    pub short_flag: String,
    pub long_flag: String,
    pub value: ValueTypes,
}

/// Parser for command options that maps flags to their configurations.
///
/// Maintains internal HashMaps for O(1) lookups by short or long flags.
///
/// # Examples
///
/// ```rust
/// let mut parser = CommandOptionsParser::new();
/// parser.add_option(SingleOption {
///     name: "verbose".to_string(),
///     short_flag: "-v".to_string(),
///     long_flag: "--verbose".to_string(),
///     value: ValueTypes::None,
///     description: "Enable verbose output".to_string(),
/// });
/// ```
#[derive(Debug, Clone)]
pub struct CommandOptionsParser {
    pub options: Vec<SingleOption>,
    short_option_map: HashMap<String, usize>,
    long_option_map: HashMap<String, usize>,
}

impl CommandOptionsParser {
    /// Creates a new empty option parser.
    pub fn new() -> Self {
        Self {
            options: Vec::new(),
            short_option_map: HashMap::new(),
            long_option_map: HashMap::new(),
        }
    }

    /// Finds the position of an option by flag.
    ///
    /// # Arguments
    ///
    /// * `flag` - Either short ("-v") or long ("--verbose") flag
    ///
    /// # Returns
    ///
    /// * `Some(usize)` - The index in the options vector
    /// * `None` - If the flag doesn't exist
    fn get_option_position(&self, flag: &str) -> Option<usize> {
        if let Some(&index) = self.short_option_map.get(flag) {
            Some(index)
        } else if let Some(&index) = self.long_option_map.get(flag) {
            Some(index)
        } else {
            None
        }
    }

    /// Updates the value of an existing option.
    ///
    /// # Arguments
    ///
    /// * `flag` - The flag identifying the option
    /// * `value` - The new value to set
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If update succeeded
    /// * `Err(String)` - If option not found
    ///
    /// # Errors
    ///
    /// Returns an error if the flag doesn't match any registered option.
    pub fn update_option_value(&mut self, flag: &str, value: ValueTypes) -> Result<()> {
        if let Some(index) = self.get_option_position(flag) {
            if let Some(option) = self.options.get_mut(index) {
                option.value = value;
                Ok(())
            } else {
                Err(FliError::OptionNotFound(flag.to_string()))
            }
        } else {
            Err(FliError::OptionNotFound(flag.to_string()))
        }
    }

    /// Registers a new option with the parser.
    ///
    /// # Arguments
    ///
    /// * `option` - The option to add
    ///
    /// # Note
    ///
    /// Overwrites existing options with the same flags without warning.
    pub fn add_option(&mut self, option: SingleOption) {
        let index = self.options.len();
        self.short_option_map
            .insert(option.short_flag.clone(), index);
        self.long_option_map.insert(option.long_flag.clone(), index);
        self.options.push(option);
    }

    /// Retrieves an option by its short flag.
    ///
    /// # Arguments
    ///
    /// * `flag` - The short flag (e.g., "-v")
    ///
    /// # Returns
    ///
    /// * `Some(&SingleOption)` - If found
    /// * `None` - If not found
    pub fn get_option_by_short_flag(&self, flag: &str) -> Option<&SingleOption> {
        self.short_option_map
            .get(flag)
            .and_then(|&index| self.options.get(index))
    }

    /// Retrieves an option by its long flag.
    ///
    /// # Arguments
    ///
    /// * `flag` - The long flag (e.g., "--verbose")
    ///
    /// # Returns
    ///
    /// * `Some(&SingleOption)` - If found
    /// * `None` - If not found
    pub fn get_option_by_long_flag(&self, flag: &str) -> Option<&SingleOption> {
        self.long_option_map
            .get(flag)
            .and_then(|&index| self.options.get(index))
    }

    /// Checks if an option with the given flag exists.
    ///
    /// # Arguments
    ///
    /// * `flag` - Either short or long flag
    ///
    /// # Returns
    ///
    /// `true` if the option exists, `false` otherwise
    pub fn has_option(&self, flag: &str) -> bool {
        self.short_option_map.contains_key(flag) || self.long_option_map.contains_key(flag)
    }

    /// Returns all registered options.
    pub fn get_options(&self) -> &Vec<SingleOption> {
        &self.options
    }

    /// Gets the expected value type for an option.
    ///
    /// Useful for knowing whether an option expects values before parsing.
    ///
    /// # Arguments
    ///
    /// * `flag` - Either short or long flag
    ///
    /// # Returns
    ///
    /// * `Some(&ValueTypes)` - The expected value type
    /// * `None` - If option doesn't exist
    pub fn get_option_expected_value_type(&self, flag: &str) -> Option<&ValueTypes> {
        self.get_option_by_short_flag(flag)
            .or_else(|| self.get_option_by_long_flag(flag))
            .map(|opt| &opt.value)
    }
}

/// Builder for constructing a `CommandOptionsParser`.
///
/// Provides a fluent API for adding options before finalizing the parser.
///
/// # Examples
///
/// ```rust
/// let parser = CommandOptionsParserBuilder::new()
///     .add_option("verbose", "Enable verbose", "-v", "--verbose", ValueTypes::None)
///     .add_option("output", "Output file", "-o", "--output",
///                 ValueTypes::RequiredSingle(Value::Str(String::new())))
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct CommandOptionsParserBuilder {
    option_parser: CommandOptionsParser,
}

impl CommandOptionsParserBuilder {
    /// Creates a new builder with an empty parser.
    pub fn new() -> Self {
        Self {
            option_parser: CommandOptionsParser::new(),
        }
    }

    /// Adds an option to the builder.
    ///
    /// # Arguments
    ///
    /// * `name` - Internal identifier
    /// * `description` - Help text
    /// * `short_flag` - Short form
    /// * `long_flag` - Long form
    /// * `value` - Type and default
    ///
    /// # Returns
    ///
    /// `&mut self` for method chaining
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

    /// Finalizes and returns the built parser.
    ///
    /// # Returns
    ///
    /// Mutable reference to the constructed `CommandOptionsParser`
    pub fn build(&mut self) -> &mut CommandOptionsParser {
        &mut self.option_parser
    }
}
