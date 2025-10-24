use super::value_types::ValueTypes;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SingleOption {
    pub name: String,
    pub description: String,
    pub short_flag: String,
    pub long_flag: String,
    pub value: ValueTypes,
}

#[derive(Debug, Clone)]
pub struct CommandOptionsParser {
    pub options: Vec<SingleOption>,
    short_option_map: HashMap<String, usize>,
    long_option_map: HashMap<String, usize>,
}

impl CommandOptionsParser {
    pub fn new() -> Self {
        Self {
            options: Vec::new(),
            short_option_map: HashMap::new(),
            long_option_map: HashMap::new(),
        }
    }

    fn get_option_position(&self, flag: &str) -> Option<usize> {
        if let Some(&index) = self.short_option_map.get(flag) {
            Some(index)
        } else if let Some(&index) = self.long_option_map.get(flag) {
            Some(index)
        } else {
            None
        }
    }

    pub fn update_option_value(&mut self, flag: &str, value: ValueTypes) -> Result<(), String> {
        if let Some(index) = self.get_option_position(flag) {
            if let Some(option) = self.options.get_mut(index) {
                option.value = value;
                Ok(())
            } else {
                Err(format!("Option at index {} not found", index))
            }
        } else {
            Err(format!("Option with flag '{}' not found", flag))
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
        self.short_option_map
            .get(flag)
            .and_then(|&index| self.options.get(index))
    }

    pub fn get_option_by_long_flag(&self, flag: &str) -> Option<&SingleOption> {
        self.long_option_map
            .get(flag)
            .and_then(|&index| self.options.get(index))
    }

    pub fn has_option(&self, flag: &str) -> bool {
        self.short_option_map.contains_key(flag) || self.long_option_map.contains_key(flag)
    }

    pub fn get_options(&self) -> &Vec<SingleOption> {
        &self.options
    }

    pub fn get_option_expected_value_type(&self, flag: &str) -> Option<&ValueTypes> {
        self.get_option_by_short_flag(flag)
            .or_else(|| self.get_option_by_long_flag(flag))
            .map(|opt| &opt.value)
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

    pub fn build(&mut self) -> &mut CommandOptionsParser {
        &mut self.option_parser
    }
}
