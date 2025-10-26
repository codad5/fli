use crate::{
    command::{FliCallbackData, FliCommand},
    display,
    error::{FliError, Result},
    option_parser::{InputArgsParser, Value, ValueTypes},
};

/// The main application struct for building CLI applications.
///
/// `Fli` serves as the entry point for defining commands, options, and running
/// the command-line parser. It wraps a root command that handles all subcommands.
///
/// # Examples
///
/// ```rust
/// use fli::option_parser::ValueTypes;
/// use fli::Fli;
///
/// let mut app = Fli::new("myapp", "1.0.0", "A sample CLI application");
/// app.add_option("verbose", "Enable verbose output", "-v", "--verbose",
///                ValueTypes::None);
/// app.run();
/// ```
pub struct Fli {
    pub name: String,
    pub version: String,
    pub description: String,
    pub root_command: FliCommand, // this is like a normal command but the command is an empty string
}

impl Fli {
    /// Creates or retrieves a subcommand.
    ///
    /// If a command with the given name doesn't exist, it will be created and added
    /// to the root command. Returns a mutable reference to the command for chaining.
    ///
    /// # Arguments
    ///
    /// * `name` - The command name (used to invoke it from CLI)
    /// * `description` - Help text describing what the command does
    ///
    /// # Returns
    ///
    /// A mutable reference to the `FliCommand` instance.
    ///
    /// # Panics
    ///
    /// Panics if the command cannot be retrieved after insertion (should not happen).
    ///
    /// # Examples
    ///
    /// ```rust
    /// app.command("serve", "Start the web server")
    ///    .add_option("port", "Port to bind to", "-p", "--port",
    ///                ValueTypes::RequiredSingle(Value::Int(8080)));
    /// ```
    pub fn command(&mut self, name: &str, description: &str) -> Result<&mut FliCommand> {
        // Use subcommand() to automatically inherit options
        Ok(self.root_command.subcommand(name, description))
    }

    /// Adds a pre-configured command to the application.
    ///
    /// Use this when you've constructed a `FliCommand` separately and want to
    /// add it to the application.
    ///
    /// # Arguments
    ///
    /// * `command` - A fully configured `FliCommand` instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut serve_cmd = FliCommand::new("serve", "Start server");
    /// serve_cmd.add_option(/* ... */);
    /// app.add_command(serve_cmd);
    /// ```
    pub fn add_command(&mut self, command: FliCommand) {
        self.root_command.add_sub_command(command);
    }

    /// Sets a callback function to execute when the root command is invoked.
    ///
    /// This is useful for applications with a default action or for handling
    /// cases where no subcommand is specified.
    ///
    /// # Arguments
    ///
    /// * `callback` - Function to execute, receives `FliCallbackData` with parsed args
    ///
    /// # Examples
    ///
    /// ```rust
    /// app.set_callback(|data| {
    ///     println!("Running default action");
    /// });
    /// ```
    pub fn set_callback(&mut self, callback: fn(&FliCallbackData)) {
        self.root_command.set_callback(callback);
    }

    /// Adds an option to the root command.
    ///
    /// Options are flags or parameters that can be passed to the application.
    /// They are accessible to all subcommands unless overridden.
    ///
    /// # Arguments
    ///
    /// * `name` - Internal identifier for the option
    /// * `description` - Help text shown to users
    /// * `short_flag` - Short form (e.g., "-v")
    /// * `long_flag` - Long form (e.g., "--verbose")
    /// * `value` - The type and default value for this option
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fli::option_parser::ValueTypes;
    /// app.add_option("config", "Config file path", "-c", "--config",
    ///                ValueTypes::OptionalSingle(None));
    /// ```
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

    /// Marks a single option as inheritable to all subcommands.
    ///
    /// Inheritable options are automatically propagated to all subcommands created
    /// after marking. This is useful for common options like verbose, quiet, or
    /// color flags that should be available across all commands.
    ///
    /// # Arguments
    ///
    /// * `flag` - The short or long flag of the option to mark as inheritable (e.g., "-v" or "--verbose")
    ///
    /// # Returns
    ///
    /// `Ok(())` if the option was successfully marked as inheritable, or an error if the flag doesn't exist
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fli::Fli;
    /// use fli::option_parser::ValueTypes;
    ///
    /// let mut app = Fli::new("myapp", "1.0.0", "My application");
    /// app.add_option("verbose", "Enable verbose output", "-v", "--verbose", ValueTypes::None);
    /// app.mark_inheritable("-v").unwrap();
    ///
    /// // All subcommands will now have the -v/--verbose option
    /// app.command("start", "Start the service").unwrap();
    /// ```
    ///
    /// # Notes
    ///
    /// - Options must be added before marking them as inheritable
    /// - Only subcommands created after marking will inherit the option
    /// - Each subcommand gets its own copy of inherited options
    pub fn mark_inheritable(&mut self, flag: &str) -> Result<()> {
        self.root_command.get_option_parser().mark_inheritable(flag)
    }

    /// Marks multiple options as inheritable to all subcommands.
    ///
    /// This is a convenience method for marking several options at once, equivalent
    /// to calling `mark_inheritable()` multiple times.
    ///
    /// # Arguments
    ///
    /// * `flags` - An iterator of flag strings (short or long) to mark as inheritable
    ///
    /// # Returns
    ///
    /// `Ok(())` if all options were successfully marked, or an error if any flag doesn't exist
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fli::Fli;
    /// use fli::option_parser::ValueTypes;
    ///
    /// let mut app = Fli::new("myapp", "1.0.0", "My application");
    /// app.add_option("verbose", "Enable verbose", "-v", "--verbose", ValueTypes::None);
    /// app.add_option("quiet", "Suppress output", "-q", "--quiet", ValueTypes::None);
    /// app.add_option("color", "Enable colors", "-c", "--color", ValueTypes::None);
    ///
    /// // Mark all three options as inheritable at once
    /// app.mark_inheritable_many(&["-v", "-q", "-c"]).unwrap();
    ///
    /// // All subcommands will now have -v, -q, and -c options
    /// ```
    ///
    /// # Notes
    ///
    /// - All options must exist before calling this method
    /// - If any flag is invalid, the entire operation fails
    /// - Subcommands created after this call will inherit all marked options
    pub fn mark_inheritable_many<I, S>(&mut self, flags: I) -> Result<()>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.root_command
            .get_option_parser()
            .mark_inheritable_many(flags)
    }
}

impl Fli {
    /// Creates a new CLI application.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the application (used in help text and error messages)
    /// * `version` - The version string (e.g., "1.0.0")
    /// * `description` - A brief description of what the application does
    ///
    /// # Returns
    ///
    /// A new `Fli` instance with an empty root command
    ///
    /// # Examples
    ///
    /// ```rust
    ///  use fli::Fli;
    /// let app = Fli::new("git", "2.0.0", "Distributed version control system");
    /// ```
    pub fn new(name: &str, version: &str, description: &str) -> Self {
        Self {
            name: name.to_owned(),
            version: version.to_owned(),
            description: description.to_owned(),
            root_command: FliCommand::new("", description),
        }
    }

    /// Parses command-line arguments and executes the appropriate command.
    ///
    /// This method:
    /// 1. Collects arguments from `std::env::args()`
    /// 2. Parses them into a command chain
    /// 3. Executes the matched command's callback
    /// 4. Exits with error code 1 if parsing fails
    ///
    /// # Panics
    ///
    /// Exits the process with code 1 if argument parsing fails or an invalid
    /// command is specified.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut app = Fli::new("myapp", "1.0.0", "Description");
    /// // ... configure app ...
    /// app.run();  // Never returns on error
    /// ```
    ///
    /// # Note
    ///
    /// This method calls `std::process::exit()` on errors. For library usage,
    /// consider using a `run_with_args()` variant that returns `Result`.
    pub fn run(&mut self) {
        let args: Vec<String> = std::env::args().collect();

        display::debug_print("App", &format!("Running {} v{}", self.name, self.version));
        display::debug_struct("Arguments", &args);

        // Skip the program name
        let command_args = if args.len() > 1 {
            args[1..].to_vec()
        } else {
            Vec::new()
        };

        let parser = InputArgsParser::new(self.root_command.get_name().to_string(), command_args);

        match self.root_command.run(parser) {
            Ok(_) => {
                display::debug_print("App", "Execution completed successfully");
            }
            Err(e) => {
                display::print_error_detailed(
                    "Command Execution Failed",
                    &e.to_string(),
                    Some("Run with --help for usage information"),
                );

                if let FliError::UnknownCommand(cmd, available) = e {
                    display::print_did_you_mean(&cmd, &available);
                }
                std::process::exit(1);
            }
        }
    }
}

impl Fli {
    /// Enable debug mode for the application
    pub fn with_debug(mut self) -> Self {
        display::enable_debug();
        self
    }

    /// Add a debug flag to root command
    pub fn add_debug_option(&mut self) {
        self.root_command.add_option_with_callback(
            "debug",
            "Enable debug output",
            "-D",
            "--debug",
            ValueTypes::OptionalSingle(Some(Value::Bool(false))),
            false,
            |data| {
                display::enable_debug();
            },
        );

        self.mark_inheritable("--debug").unwrap();

        // Check if debug flag is present in args
        if std::env::args().any(|arg| arg == "-D" || arg == "--debug") {
            display::enable_debug();
        }
    }
}
