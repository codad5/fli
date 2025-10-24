// command.rs
use std::collections::HashMap;

use crate::option_parser::{
    CommandChain, CommandOptionsParser, CommandOptionsParserBuilder, InputArgsParser, ValueTypes,
};

/// Context data passed to command callbacks containing parsed arguments and options.
///
/// This struct provides a convenient API for accessing parsed command-line data
/// without dealing with raw argument vectors.
///
/// # Examples
///
/// ```rust
/// fn my_command(data: &FliCallbackData) {
///     let name = data.get_option_value("name")
///         .and_then(|v| v.as_str())
///         .unwrap_or("World");
///     println!("Hello, {}!", name);
/// }
/// ```
#[derive(Debug, Clone)]
pub struct FliCallbackData {
    pub command: FliCommand,
    pub option_parser: CommandOptionsParser,
    pub arguments: Vec<String>,
    pub arg_parser: InputArgsParser,
}

impl FliCallbackData {
    /// Creates a new callback data context.
    ///
    /// # Arguments
    ///
    /// * `command` - The command being executed
    /// * `option_parser` - Parser containing all parsed options
    /// * `arguments` - Positional arguments passed to the command
    /// * `arg_parser` - The argument parser with full parse state
    ///
    /// # Note
    ///
    /// This is typically created internally by the framework. Users rarely need
    /// to construct this manually.
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

    /// Retrieves the parsed value for a given option name.
    ///
    /// Supports multiple lookup formats:
    /// - With dashes: "-v", "--verbose"
    /// - Without dashes: "v", "verbose"
    ///
    /// # Arguments
    ///
    /// * `name` - The option name (with or without dashes)
    ///
    /// # Returns
    ///
    /// * `Some(&ValueTypes)` - The parsed value if the option was provided
    /// * `None` - If the option wasn't provided or doesn't exist
    ///
    /// # Examples
    ///
    /// ```rust
    /// // All of these work:
    /// data.get_option_value("name");
    /// data.get_option_value("-n");
    /// data.get_option_value("--name");
    /// ````
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

    /// Retrieves a positional argument by index.
    ///
    /// # Arguments
    ///
    /// * `index` - Zero-based index of the argument
    ///
    /// # Returns
    ///
    /// * `Some(&String)` - The argument at the given index
    /// * `None` - If index is out of bounds
    ///
    /// # Examples
    ///
    /// ```rust
    /// // For command: myapp copy file1.txt file2.txt
    /// let source = data.get_argument_at(0);  // Some("file1.txt")
    /// let dest = data.get_argument_at(1);    // Some("file2.txt")
    /// ```
    pub fn get_argument_at(&self, index: usize) -> Option<&String> {
        self.arguments.get(index)
    }

    /// Returns all positional arguments as a vector.
    ///
    /// # Returns
    ///
    /// A reference to the vector of all positional arguments
    ///
    /// # Examples
    ///
    /// ```rust
    /// for arg in data.get_arguments() {
    ///     println!("Argument: {}", arg);
    /// }
    /// ```
    pub fn get_arguments(&self) -> &Vec<String> {
        &self.arguments
    }

    /// Returns a reference to the command being executed.
    ///
    /// # Returns
    ///
    /// A reference to the `FliCommand` that matched this execution
    pub fn get_command(&self) -> &FliCommand {
        &self.command
    }

    /// Returns a reference to the argument parser.
    ///
    /// Provides access to low-level parsing details if needed.
    ///
    /// # Returns
    ///
    /// A reference to the `InputArgsParser` used for parsing
    pub fn get_arg_parser(&self) -> &InputArgsParser {
        &self.arg_parser
    }
}

/// Metadata for options that have custom callbacks.
///
/// Preserved options trigger their callback immediately when encountered during
/// parsing (e.g., --help exits early rather than continuing execution).
#[derive(Debug, Clone)]
pub struct PreservedOption {
    pub long_flag: String,
    pub short_flag: String,
    pub value_type: ValueTypes,
    pub callback: fn(&FliCallbackData),
}

/// Represents a CLI command with options, subcommands, and execution logic.
///
/// Commands form a tree structure where each command can have subcommands,
/// creating a hierarchical CLI interface (like `git commit` or `docker run`).
///
/// # Examples
///
/// ```rust
/// let mut cmd = FliCommand::new("serve", "Start the server");
/// cmd.add_option("port", "Port to bind", "-p", "--port",
///                ValueTypes::OptionalSingle(Some(Value::Int(8080))));
/// cmd.set_callback(|data| {
///     // Server logic here
/// });
/// ```
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
    /// Creates a new command.
    ///
    /// # Arguments
    ///
    /// * `name` - The command name (used to invoke it)
    /// * `description` - Help text describing the command
    ///
    /// # Returns
    ///
    /// A new `FliCommand` with auto-generated help flag
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

    /// Adds a standard --help/-h flag to the command.
    ///
    /// This is called automatically in `new()`. The help flag displays:
    /// - Command description
    /// - Available options
    /// - Subcommands
    ///
    /// # Note
    ///
    /// This is a preserved option, meaning it executes immediately and
    /// exits the program.
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
                    println!("  {} / {} : {}", x.short_flag, x.long_flag, x.description);
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

    /// Sets the callback function for this command.
    ///
    /// The callback is invoked when this command is matched during parsing.
    ///
    /// # Arguments
    ///
    /// * `callback` - Function that receives `FliCallbackData` with parsed values
    pub fn set_callback(&mut self, callback: fn(&FliCallbackData)) {
        self.callback = Some(callback);
    }

    /// Returns the command name
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Returns the command description.
    pub fn get_description(&self) -> &String {
        &self.description
    }

    /// Returns a reference to the option parser builder.
    ///
    /// Useful for inspecting configured options before parsing.
    pub fn get_option_parser_builder(&self) -> &CommandOptionsParserBuilder {
        &self.option_parser_builder
    }

    /// Returns a mutable reference to the built option parser.
    ///
    /// This builds the parser if it hasn't been built yet.
    pub fn get_option_parser(&mut self) -> &mut CommandOptionsParser {
        self.option_parser_builder.build()
    }

    /// Returns all subcommands as a HashMap.
    pub fn get_sub_commands(&self) -> &HashMap<String, FliCommand> {
        &self.sub_commands
    }

    /// Checks if this command has any subcommands
    pub fn has_sub_commands(&self) -> bool {
        !self.sub_commands.is_empty()
    }

    /// Retrieves a specific subcommand by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The subcommand name
    ///
    /// # Returns
    ///
    /// * `Some(&FliCommand)` - If the subcommand exists
    /// * `None` - If no subcommand with that name exists
    pub fn get_sub_command(&self, name: &str) -> Option<&FliCommand> {
        self.sub_commands.get(name)
    }

    /// Retrieves a mutable reference to a specific subcommand.
    ///
    /// # Arguments
    ///
    /// * `name` - The subcommand name
    ///
    /// # Returns
    ///
    /// * `Some(&mut FliCommand)` - If the subcommand exists
    /// * `None` - If no subcommand with that name exists
    pub fn get_sub_command_mut(&mut self, name: &str) -> Option<&mut FliCommand> {
        self.sub_commands.get_mut(name)
    }

    /// Checks if a subcommand with the given name exists.
    pub fn has_sub_command(&self, name: &str) -> bool {
        self.sub_commands.contains_key(name)
    }

    /// Adds an option to this command.
    ///
    /// # Arguments
    ///
    /// * `name` - Internal identifier
    /// * `description` - Help text
    /// * `short_flag` - Short form (e.g., "-p")
    /// * `long_flag` - Long form (e.g., "--port")
    /// * `value` - Type and default value
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
        self.option_parser_builder
            .add_option(name, description, short_flag, long_flag, value);
        self
    }

    /// Adds an option with a custom callback.
    ///
    /// The callback executes immediately when this option is encountered,
    /// useful for flags like --help or --version that should exit early.
    ///
    /// # Arguments
    ///
    /// * `name` - Internal identifier
    /// * `description` - Help text
    /// * `short_flag` - Short form
    /// * `long_flag` - Long form
    /// * `value` - Type and default
    /// * `callback` - Function to execute when option is found
    ///
    /// # Returns
    ///
    /// `&mut self` for method chaining
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

    /// Retrieves a preserved option by flag name.
    ///
    /// Supports lookup with or without dashes.
    ///
    /// # Arguments
    ///
    /// * `name` - The flag (e.g., "-h", "--help", "help")
    ///
    /// # Returns
    ///
    /// * `Some(&PreservedOption)` - If found
    /// * `None` - If not found
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

    /// Checks if a preserved option exists.
    pub fn has_preserved_option(&self, name: &str) -> bool {
        self.get_preserved_option(name).is_some()
    }

    /// Creates and adds a new subcommand, returning a mutable reference.
    ///
    /// This is the fluent API for building command trees.
    ///
    /// # Arguments
    ///
    /// * `name` - Subcommand name
    /// * `description` - Subcommand description
    ///
    /// # Returns
    ///
    /// Mutable reference to the newly created subcommand for chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// cmd.subcommand("start", "Start service")
    ///    .add_option("daemon", "Run as daemon", "-d", "--daemon", ValueTypes::None);
    /// ```
    pub fn subcommand(&mut self, name: &str, description: &str) -> &mut FliCommand {
        let command = FliCommand::new(name, description);
        self.add_sub_command(command);
        self.sub_commands.get_mut(name).unwrap()
    }

    /// Adds a pre-configured subcommand.
    ///
    /// # Arguments
    ///
    /// * `command` - A fully configured `FliCommand`
    pub fn add_sub_command(&mut self, command: FliCommand) {
        self.sub_commands
            .insert(command.get_name().to_owned(), command);
    }

    /// Returns the callback function if one is set.
    pub fn get_callback(&self) -> Option<fn(&FliCallbackData)> {
        self.callback
    }

    /// Executes this command with the given argument parser.
    ///
    /// This method:
    /// 1. Parses arguments using the provided parser
    /// 2. Handles subcommand delegation recursively
    /// 3. Executes preserved option callbacks (like --help)
    /// 4. Executes the command's main callback
    ///
    /// # Arguments
    ///
    /// * `arg_parser` - Parser initialized with command-line arguments
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If execution succeeded
    /// * `Err(String)` - If parsing failed or unknown command/option encountered
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Required option values are missing
    /// - Unknown subcommands are specified
    /// - Option parsing fails
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
