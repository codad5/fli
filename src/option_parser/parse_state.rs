use super::value_types::ValueTypes;

#[derive(Debug, Clone)]
pub enum ParseState {
    Start,
    InCommand,
    InOption,
    AcceptingValue(String, ValueTypes),
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