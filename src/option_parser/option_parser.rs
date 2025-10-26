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
    inheritable_flags: Vec<usize>,
}

impl CommandOptionsParser {
    /// Creates a new empty option parser.
    pub fn new() -> Self {
        Self {
            options: Vec::new(),
            short_option_map: HashMap::new(),
            long_option_map: HashMap::new(),
            inheritable_flags: Vec::new(),
        }
    }

    /// Marks a single option as inheritable by its flag.
    ///
    /// Inheritable options are automatically passed down to subcommands, eliminating
    /// the need to redefine common options (like `--verbose`, `--quiet`, `--color`)
    /// for every subcommand.
    ///
    /// # Arguments
    ///
    /// * `flag` - The short or long flag of the option to mark as inheritable (e.g., "-v" or "--verbose")
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the flag was found and successfully marked as inheritable
    /// * `Err(FliError::OptionNotFound)` - If the flag doesn't correspond to any registered option
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fli::option_parser::{CommandOptionsParser, ValueTypes};
    ///
    /// let mut parser = CommandOptionsParser::new();
    /// parser.add_option("verbose", "Enable verbose output", "-v", "--verbose", ValueTypes::None);
    /// parser.mark_inheritable("-v").unwrap();
    ///
    /// // Now the -v flag will be automatically available to all subcommands
    /// ```
    ///
    /// # Notes
    ///
    /// - If an option is already marked as inheritable, calling this again has no effect
    /// - The option must exist before it can be marked as inheritable
    /// - Only the flag is needed, not the option name
    pub fn mark_inheritable(&mut self, flag: &str) -> Result<()> {
        if let Some(index) = self.get_option_position(flag) {
            if !self.inheritable_flags.contains(&index) {
                self.inheritable_flags.push(index);
            }
            Ok(())
        } else {
            Err(FliError::OptionNotFound(flag.to_string()))
        }
    }

    /// Marks multiple options as inheritable in a single call.
    ///
    /// This is a convenience method for marking several options as inheritable at once.
    /// It accepts any iterable of items that can be referenced as strings (short or long flags).
    ///
    /// # Arguments
    ///
    /// * `flags` - An iterable of flag strings (e.g., `&["-v", "--quiet", "--color"]`)
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If all flags were found and successfully marked as inheritable
    /// * `Err(FliError::OptionNotFound)` - If any flag is not found (processing stops at first error)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fli::option_parser::{CommandOptionsParser, ValueTypes};
    ///
    /// let mut parser = CommandOptionsParser::new();
    /// parser.add_option("verbose", "Enable verbose output", "-v", "--verbose", ValueTypes::None);
    /// parser.add_option("quiet", "Suppress output", "-q", "--quiet", ValueTypes::None);
    /// parser.add_option("color", "Enable colors", "-c", "--color", ValueTypes::None);
    ///
    /// // Mark all three as inheritable at once
    /// parser.mark_inheritable_many(&["-v", "-q", "-c"]).unwrap();
    /// ```
    ///
    /// # Notes
    ///
    /// - Processing stops at the first error encountered
    /// - No partial marking occurs - if one flag fails, previously processed flags remain marked
    /// - You can mix short and long flags in the same call
    pub fn mark_inheritable_many<I, S>(&mut self, flags: I) -> Result<()>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for f in flags {
            self.mark_inheritable(f.as_ref())?;
        }
        Ok(())
    }

    /// Creates a builder containing only the options marked as inheritable.
    ///
    /// This method is primarily used internally to propagate inheritable options to subcommands.
    /// It returns a `CommandOptionsParserBuilder` pre-populated with clones of all options
    /// that have been marked as inheritable.
    ///
    /// # Returns
    ///
    /// A `CommandOptionsParserBuilder` containing clones of all inheritable options
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fli::option_parser::{CommandOptionsParser, ValueTypes};
    ///
    /// let mut parser = CommandOptionsParser::new();
    /// parser.add_option("verbose", "Enable verbose output", "-v", "--verbose", ValueTypes::None);
    /// parser.add_option("quiet", "Suppress output", "-q", "--quiet", ValueTypes::None);
    /// parser.mark_inheritable("-v").unwrap();
    ///
    /// // Get a builder with only the inheritable options
    /// let builder = parser.inheritable_options_builder();
    /// // This builder contains only the --verbose option
    /// ```
    ///
    /// # Notes
    ///
    /// - The returned builder is independent - modifying it doesn't affect the parent parser
    /// - Options are cloned, so changes to the parent won't affect already-created builders
    /// - If no options are marked as inheritable, returns an empty builder
    /// - This is typically used when creating subcommands to inherit parent options
    pub fn inheritable_options_builder(&self) -> CommandOptionsParserBuilder {
        let mut builder = CommandOptionsParserBuilder::new();
        for &idx in &self.inheritable_flags {
            if let Some(opt) = self.options.get(idx) {
                // add_option clones the provided data where necessary
                builder.add_option(
                    &opt.name,
                    &opt.description,
                    &opt.short_flag,
                    &opt.long_flag,
                    opt.value.clone(),
                );
            }
        }
        builder
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
