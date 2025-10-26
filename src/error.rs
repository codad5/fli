use thiserror::Error;

use crate::command::FliCommand;

/// Errors that can occur during CLI parsing and execution
#[derive(Debug, Error)]
pub enum FliError {
    
    /// Command name doesn't match expected command
    #[error("Command mismatch: expected '{expected}', got '{actual}'")]
    CommandMismatch {
        expected: String,
        actual: String,
    },

    /// Unknown subcommand was specified
    #[error("Unknown command: '{0}'. Run with --help to see available commands")]
    UnknownCommand(String, Vec<String>),

    /// Unknown option flag was provided
    #[error("Unknown option: '{0}'. Run with --help to see available options")]
    UnknownOption(String),

    // ==================== Value Errors ====================
    
    /// Required option value is missing
    #[error("Missing required value for option '{option}'")]
    MissingValue {
        option: String,
    },

    /// Option expects no value but one was provided
    #[error("Option '{option}' does not accept values, but '{value}' was provided")]
    UnexpectedValue {
        option: String,
        value: String,
    },

    /// Wrong number of values provided
    #[error("Option '{option}' expected {expected} value(s), got {actual}")]
    ValueCountMismatch {
        option: String,
        expected: usize,
        actual: usize,
    },

    /// Value parsing failed (e.g., string to int)
    #[error("Invalid value '{value}' for option '{option}': {reason}")]
    InvalidValue {
        option: String,
        value: String,
        reason: String,
    },

    // ==================== State Errors ====================
    
    /// Invalid state transition during parsing
    #[error("Invalid parse state transition from {from:?} to {to:?}")]
    InvalidStateTransition {
        from: String,
        to: String,
    },

    /// Unexpected token at position
    #[error("Unexpected '{token}' at position {position}")]
    UnexpectedToken {
        token: String,
        position: usize,
    },

    // ==================== Configuration Errors ====================
    
    /// Option configuration is invalid
    #[error("Invalid option configuration for '{option}': {reason}")]
    InvalidOptionConfig {
        option: String,
        reason: String,
    },

    /// Command configuration is invalid
    #[error("Invalid command configuration: {0}")]
    InvalidCommandConfig(String),

    /// Option flag format is invalid
    #[error("Invalid flag format: '{flag}'. Flags must start with '-' or '--'")]
    InvalidFlagFormat {
        flag: String,
    },

    // ==================== Runtime Errors ====================
    
    /// Option was not found in parser
    #[error("Option '{0}' not found in parser")]
    OptionNotFound(String),

    /// Parser not prepared before use
    #[error("Parser must be prepared before use. Call prepare() first")]
    ParserNotPrepared,

    // ==================== Generic Errors ====================
    
    /// Internal error (shouldn't happen in normal usage)
    #[error("Internal error: {0}")]
    Internal(String),
    /// Invalid command usage (e.g., wrong flags or operands)
    #[error("Invalid usage: {0}. Run with --help to see correct usage")]
    InvalidUsage (String),
}

impl FliError {
    /// Creates a command mismatch error
    pub fn command_mismatch(expected: impl Into<String>, actual: impl Into<String>) -> Self {
        Self::CommandMismatch {
            expected: expected.into(),
            actual: actual.into(),
        }
    }

    /// Creates a missing value error
    pub fn missing_value(option: impl Into<String>) -> Self {
        Self::MissingValue {
            option: option.into(),
        }
    }

    /// Creates an invalid value error
    pub fn invalid_value(
        option: impl Into<String>,
        value: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self::InvalidValue {
            option: option.into(),
            value: value.into(),
            reason: reason.into(),
        }
    }

    /// Creates a value count mismatch error
    pub fn value_count_mismatch(
        option: impl Into<String>,
        expected: usize,
        actual: usize,
    ) -> Self {
        Self::ValueCountMismatch {
            option: option.into(),
            expected,
            actual,
        }
    }
}

/// Type alias for Results using FliError
pub type Result<T> = std::result::Result<T, FliError>;