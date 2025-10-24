use super::value_types::ValueTypes;

/// Represents the current state during argument parsing.
///
/// This enum implements a state machine that enforces valid transitions
/// during parsing, preventing invalid parse sequences.
///
/// # State Transitions
///
/// ```text
/// Start → InCommand → InOption ⇄ AcceptingValue
///   ↓         ↓           ↓
///   ↓         ↓       Breaking
///   └────→ InArgument ←──┘
///              ↓
///            End
/// ```
///
/// # Examples
///
/// ```rust
/// let mut state = ParseState::Start;
/// state.set_next_mode(ParseState::InCommand)?;
/// state.set_next_mode(ParseState::InOption)?;
/// ```
#[derive(Debug, Clone)]
pub enum ParseState {
    /// Initial state before parsing begins
    Start,
    /// Currently inside a command context (after command name verified)
    InCommand,
    /// Currently processing an option flag
    InOption,
    /// Waiting to consume value(s) for an option
    /// - First field: option name
    /// - Second field: expected value type
    AcceptingValue(String, ValueTypes),
    /// After "--" separator, all remaining args are positional
    Breaking,
    /// Processing a positional argument
    InArgument,
    /// Parsing completed successfully
    End,
}

impl ParseState {

    /// Attempts to transition to the next state.
    ///
    /// Validates the transition is legal before applying it.
    ///
    /// # Arguments
    ///
    /// * `next` - The desired next state
    ///
    /// # Returns
    ///
    /// * `Ok(&mut Self)` - If transition is valid
    /// * `Err(String)` - If transition is invalid with error message
    ///
    /// # Errors
    ///
    /// Returns an error if the transition violates state machine rules.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut state = ParseState::Start;
    /// state.set_next_mode(ParseState::InCommand)?;  // OK
    /// state.set_next_mode(ParseState::Start)?;       // Error: can't go back to Start
    /// ```
    pub fn set_next_mode(&mut self, next: ParseState) -> Result<&mut Self, String> {
        if self.can_go_to_next(&next) {
            *self = next;
            Ok(self)
        } else {
            Err(format!("Cannot transition from {:?} to {:?}", self, next))
        }
    }

    /// Checks if a transition to the given state is valid.
    ///
    /// # Arguments
    ///
    /// * `next` - The state to check transition validity for
    ///
    /// # Returns
    ///
    /// `true` if transition is allowed, `false` otherwise
    ///
    /// # Validation Rules
    ///
    /// - Cannot return to `Start`
    /// - Can only enter `InCommand` from `Start` or another `InCommand` (subcommands)
    /// - Can enter `InOption` from most states except `Start`
    /// - Can only enter `AcceptingValue` from `InOption`
    /// - Can enter `InArgument` from command, breaking, or another argument
    /// - Can enter `Breaking` from option or accepting value states
    /// - Can enter `End` from any state except `Start`

    pub fn can_go_to_next(&self, next: &ParseState) -> bool {
        match next {
            ParseState::Start => false, // cannot go back to start

            // can only go to command from start or another command (subcommands)
            ParseState::InCommand => matches!(self, ParseState::Start | ParseState::InCommand),

            // can go to option from: start, command, after accepting value, or another option
            ParseState::InOption => matches!(
                self,
                ParseState::Start
                    | ParseState::InCommand
                    | ParseState::InArgument
                    | ParseState::InOption
                    | ParseState::AcceptingValue(_, _)
            ),

            // can only go to accepting value from option
            ParseState::AcceptingValue(_, _) => matches!(self, ParseState::InOption),

            // can go to argument from: start, command, breaking, or another argument
            ParseState::InArgument => matches!(
                self,
                ParseState::Start
                    | ParseState::InCommand
                    | ParseState::Breaking
                    | ParseState::InArgument
            ),

            // can go to breaking from: option or accepting value
            ParseState::Breaking => matches!(
                self,
                ParseState::InOption | ParseState::AcceptingValue(_, _)
            ),

            // can go to end from any state except Start
            ParseState::End => !matches!(self, ParseState::Start),
        }
    }
}
