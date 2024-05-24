use colored::Colorize;
use std::{collections::HashMap, env, process};

use crate::{fli_default_callback, levenshtein_distance};

// This is the main struct that holds all the data
pub struct Fli {
    name: String,
    description: String,
    args: Vec<String>,
    pub args_hash_table: HashMap<String, fn(app: &Self)>,
    short_hash_table: HashMap<String, String>,
    cammands_hash_tables: HashMap<String, Fli>,
    help_hash_table: HashMap<String, String>,
    default_callback: fn(app: &Self),
    allow_duplicate_callback: bool,
    allow_inital_no_param_values: bool,
}

impl Fli {
    pub fn init(name: &str, description: &str) -> Self {
        let mut app = Self {
            name: name.to_string(),
            description: description.to_string(),
            args: env::args().collect(),
            args_hash_table: HashMap::new(),
            short_hash_table: HashMap::new(),
            cammands_hash_tables: HashMap::new(),
            help_hash_table: HashMap::new(),
            default_callback: fli_default_callback,
            allow_duplicate_callback: false,
            allow_inital_no_param_values: false,
        };
        app.add_help_option();
        return app;
    }
    pub fn command(&mut self, name: &str, description: &str) -> &mut Fli {
        let mut args = self.args.clone();
        args.remove(0);
        let mut new_fli = Self {
            name: name.to_string(),
            description: description.to_string(),
            args: args,
            args_hash_table: HashMap::new(),
            short_hash_table: HashMap::new(),
            cammands_hash_tables: HashMap::new(),
            help_hash_table: HashMap::new(),
            default_callback: fli_default_callback,
            allow_duplicate_callback: self.allow_duplicate_callback,
            allow_inital_no_param_values: self.allow_inital_no_param_values,
        };
        new_fli.add_help_option();
        self.cammands_hash_tables.insert(name.to_string(), new_fli);
        self.help_hash_table
            .insert(name.to_string(), description.to_string());
        return self
            .cammands_hash_tables
            .get_mut(&name.to_string())
            .unwrap();
    }

    pub fn allow_duplicate_callback(&mut self, data: bool) -> &mut Self {
        self.allow_duplicate_callback = data;
        self
    }

    pub fn allow_inital_no_param_values(&mut self, data: bool) -> &mut Self {
        self.allow_inital_no_param_values = data;
        self
    }

    fn add_help_option(&mut self) {
        self.option(
            "-h --help",
            &format!("print help screen for {}", self.name),
            |x| x.default_help(),
        );
    }
    pub fn print_help(&self, message: &str) {
        println!(
            "{0: <1} {1}",
            "",
            "ERROR================================".bold().red()
        );
        println!("{0: <5} {1}", "", message.bright_red());
        println!(
            "{0: <1} {1}",
            "",
            "================================".bold().red()
        );
        self.default_help();
        process::exit(0);
    }
    fn default_help(&self) {
        println!("{0: <1} {1}: {2}", "", "Name".bold().green(), self.name);
        println!(
            "{0: <1} {1}: {2}",
            "",
            "Description".bold().blue(),
            self.description
        );
        println!(
            "{0: <1} {1}: {2} [options|commands]",
            "",
            "Usage".bold().yellow(),
            self.name
        );
        self.print_options();
        self.print_commands();
        process::exit(0);
    }

    pub fn print_most_similar_commands(&self, command: &str) {
        let similar_commands = self.get_most_similar_commands(command);
        if similar_commands.len() > 0 {
            println!("{0: <1} {1}", "", "Did you mean:".bold().red());
            for i in similar_commands {
                //  give about 2 tap space then a bullet point before showing the similar command 
                println!("{0: <4} {1} {2}", "   ", "•".bold().red(), i.bold());
            }
        }
    }

    fn get_most_similar_commands(&self, command: &str) -> Vec<String> {
        //  get commands with distances less than 3
        let mut similar_commands: Vec<String> = vec![];
        for key in self.help_hash_table.keys() {
            let distance = levenshtein_distance(&command, key);
            if distance < 3 {
                similar_commands.push(key.to_string());
            }
        }
        return similar_commands;
    }

    fn print_options(&self) {
        println!("{0: <1} {1}", "", "Options:".bold().blue());
        println!(
            "{0: <2}  {1: <12} | {2: <10} | {3: <10} | {4: <10}",
            "",
            "Long".bold().blue(),
            "Short".bold().green(),
            "ParamType",
            "Description".bold().yellow()
        );
        for key in self.help_hash_table.keys() {
            // if a command skip
            if self.cammands_hash_tables.contains_key(key) {
                continue;
            }
            if let Some(description) = self.help_hash_table.get(key) {
                let mut short = String::new();
                if let Some(short_key) = key.split(" ").collect::<Vec<&str>>().get(0) {
                    short = short_key.to_string();
                }
                let mut param_type = String::new();
                if let Some(param_d) = key.split(" ").collect::<Vec<&str>>().get(2) {
                    param_type = match param_d.trim() {
                        "<>" => "Required",
                        "[]" => "Optional",
                        "<...>" => "Required Multiple",
                        "[...]" => "Optional Multiple",
                        _ => "None",
                    }
                    .to_string();
                }
                let mut long = String::new();
                if let Some(long_key) = key.split(" ").collect::<Vec<&str>>().get(1) {
                    long = String::from(long_key.to_owned());
                }
                println!(
                    "{0: <2}  {1: <12} | {2: <10} | {3: <10} | {4: <10}",
                    "",
                    long.blue(),
                    short.green(),
                    param_type,
                    description.yellow()
                );
            }
        }
    }
    fn print_commands(&self) {
        println!("{0: <1} {1}", "", "Commands:".bold().blue());
        println!(
            "{0: <2} {1: <12} | {2: <10}",
            "",
            "Name".bold().blue(),
            "Description".bold().yellow()
        );
        for key in self.help_hash_table.keys() {
            // if a command skip
            if !self.cammands_hash_tables.contains_key(key) {
                continue;
            }
            if let Some(description) = self.help_hash_table.get(key) {
                println!(
                    "{0: <2} {1: <12} | {2: <10}",
                    "",
                    key.blue(),
                    description.yellow()
                );
            }
        }
    }
    pub fn default(&mut self, callback: fn(app: &Self)) -> &mut Self {
        self.default_callback = callback;
        return self;
    }

    pub fn option(&mut self, key: &str, description: &str, value: fn(app: &Self)) -> &mut Self {
        let args: Vec<&str> = key.split(",").collect();
        let mut options = String::new();
        if let Some(opts) = args.get(0) {
            options = String::from(opts.to_owned());
        }
        let broken_args: Vec<_> = options.split(" ").collect();
        let short = broken_args[0].trim();
        let mut long = broken_args[0].trim();
        if broken_args.len() > 1 {
            long = broken_args[1].trim();
            self.short_hash_table
                .insert(short.to_string(), long.to_string());
        }
        // for i in options.split(" ") {
        let mut param_type = String::new();
        if let Some(param_d) = args.get(1) {
            param_type = String::from(param_d.to_owned());
        }
        if args.len() > 1 && ["<>", "[]", "<...>", "[...]"].contains(&param_type.trim()) == false {
            self.print_help(&format!("Error : unknown param type {param_type}"));
        }
        let option: String = long.trim().to_owned() + " " + param_type.trim();
        self.args_hash_table.insert(option.trim().to_owned(), value);
        self.help_hash_table.insert(
            short.to_string() + " " + option.trim(),
            description.to_string(),
        );
        // }
        return self;
    }
    pub fn get_params_callback(&mut self, key: String) -> Option<&for<'a> fn(&'a Fli)> {
        if let Some(callback) = self.args_hash_table.get(&self.get_callable_name(key)) {
            return Some(callback);
        }
        return None;
    }
    pub fn run(&self) -> &Fli {
        let mut callbacks: Vec<for<'a> fn(&'a Fli)> = vec![];
        let mut init_arg = self.args.clone();
        init_arg.remove(0); // remove the app runner / command
        let default_callback: fn(&Fli) = fli_default_callback;
        for _arg in init_arg {
            let mut arg = _arg;
            let mut current_callback = default_callback;

            if !arg.starts_with("-") {
                if let Some(command_struct) = self.cammands_hash_tables.get(arg.trim()) {
                    return command_struct.run();
                }
                continue;
            }
            arg = self.get_callable_name(arg);
            for optional_template in ["", "[]", "[...]"] {
                // check if it need a required param
                let find = &String::from(format!("{arg} {optional_template}"));
                let callback_find = self.args_hash_table.get(find.trim());
                if callback_find.is_none() {
                    continue;
                }
                current_callback = *callback_find.unwrap();
            }
            for required_template in ["<>", "<...>"] {
                // check if it need a required param
                let find = &String::from(format!("{arg} {required_template}"));
                let callback_find = self.args_hash_table.get(find.trim());
                if callback_find.is_none() {
                    continue;
                }
                // make sure a value is passed in else it should show error/help
                if !self.has_a_value(arg.trim().to_string()) {
                    self.print_help(&format!("Invalid syntax : {arg}  does not have a value"));
                    return self;
                }
                current_callback = *(callback_find.unwrap());
            }

            if current_callback == default_callback {
                callbacks = Vec::new();
                // break;
            }

            if !callbacks.contains(&current_callback) || self.allow_duplicate_callback {
                callbacks.push(current_callback)
            }
        }
        if callbacks.len() == 0 {
            callbacks.push(self.default_callback);
        }
        self.run_callbacks(callbacks)
    }

    pub fn has_a_value(&self, arg_name: String) -> bool {
        let mut counter = 0;
        let binding = self.get_callable_name(arg_name);
        let arg_full_name = binding.trim();
        for arg in &self.args {
            if self.get_callable_name(arg.to_string()) == arg_full_name {
                if let Some(value) = self.args.get(counter + 1) {
                    if !value.starts_with("-") {
                        return true;
                    }
                }
            }
            counter += 1;
        }
        return false;
    }

    fn run_callbacks(&self, callbacks: Vec<for<'a> fn(&'a Fli)>) -> &Self {
        for callback in callbacks.clone() {
            callback(self)
        }
        self
    }
    /**
     * Gets the Long name for a short arg
     */
    pub fn get_callable_name(&self, arg: String) -> String {
        let mut arg_template: String = String::from(format!("{}", arg));
        if !arg_template.starts_with("-") {
            arg_template = String::from(format!("-{}", arg));
        }
        if let Some(long_name) = self.short_hash_table.get(&arg_template) {
            arg_template = long_name.to_string();
        }
        if !arg_template.starts_with("--") {
            arg_template = String::from(format!("--{}", arg));
        }
        return arg_template;
    }
    pub fn get_values(&self, arg: String) -> Result<Vec<String>, &str> {
        let mut values: Vec<String> = vec![];
        let arg_name: String = self.get_callable_name(arg);
        // if the argument does not need a param then dont return none
        if let Some(_) = self.args_hash_table.get(&arg_name) {
            return Err("Does not expect a value");
        }
        let mut counter = 1;
        for mut i in self.args.clone() {
            i = self.get_callable_name(i);
            if i != arg_name {
                counter += 1;
                continue;
            }
            let binding = &String::from(format!("{} []", arg_name));
            if let Some(_) = self.args_hash_table.get(binding) {
                if let Some(v) = self.args.get(counter) {
                    if v.starts_with("-") {
                        return Err("No value passed");
                    }
                    values.push(v.to_string());
                    break;
                }
            }
            let binding = &String::from(format!("{} <>", arg_name));
            if let Some(_) = self.args_hash_table.get(binding) {
                if let Some(v) = self.args.get(counter) {
                    if v.starts_with("-") {
                        return Err("No value Passed");
                    }
                    values.push(v.to_string());
                    break;
                }
            }
            let binding = &String::from(format!("{} [...]", arg_name));
            if let Some(_) = self.args_hash_table.get(binding) {
                if let Some(params) = self.args.get((counter)..self.args.len()) {
                    for i in params {
                        if i.starts_with(&"-".to_string()) {
                            break;
                        }
                        values.push(i.to_string());
                    }
                }
            }
            let binding = &String::from(format!("{} <...>", arg_name));
            if let Some(_) = self.args_hash_table.get(binding) {
                if let Some(params) = self.args.get((counter)..self.args.len()) {
                    for i in params {
                        if i.starts_with(&"-".to_string()) {
                            break;
                        }
                        values.push(i.to_string());
                    }
                }
            }
            counter += 1;
        }
        if values.len() > 0 {
            return Ok(values);
        }
        return Err("No value passed");
    }
    pub fn is_passed(&self, param: String) -> bool {
        for i in self.args.clone() {
            if self.get_callable_name(i) == self.get_callable_name(param.clone()) {
                return true;
            }
        }
        return false;
    }
    pub fn get_arg_at(&self, index: u8) -> Option<String> {
        if let Some(arg) = self.args.get(index as usize) {
            return Some(arg.to_string());
        }
        return None;
    }
}
