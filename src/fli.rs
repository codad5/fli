
use std::{env, collections::HashMap};

pub struct Fli {
    name:String,
    args : Vec<String>,
    pub args_hash_table: HashMap<String, fn(app : &Self)>,
    short_hash_table : HashMap<String, String>,
    cammands_hash_tables : HashMap<String, Fli>
}

enum FliError{
    InvalidArg(String),
    EmptyParam(String)
}

impl Fli {
    pub fn init(name : String) -> Self {
        println!("{:?}", env::args());
        Self {
            name,
            args: env::args().collect(),
            args_hash_table: HashMap::new(),
            short_hash_table: HashMap::new(),
            cammands_hash_tables: HashMap::new(),
        }
    }
    pub fn command(&mut self, name: String) -> &mut Fli
    {
        let mut args = self.args.clone();
        args.remove(0);
        println!("args: {:?}", args);
        let new_fli = Self {
            name:name.clone(),
            args: args,
            args_hash_table: HashMap::new(),
            short_hash_table: HashMap::new(),
            cammands_hash_tables: HashMap::new(),
        };
        self.cammands_hash_tables.insert(name.clone(), new_fli);
        return self.cammands_hash_tables.get_mut(&name).unwrap();
    }

    pub fn option(&mut self, key: String, value: fn(app : &Self)) -> &Fli{
        let args : Vec<&str> = key.split(",").collect();
        let mut options = String::new();
        if let Some(opts)  = args.get(0){
            options = String::from(opts.to_owned());
        }
        let broken_args : Vec<_> = options.split(" ").collect();
        let short = broken_args[0].trim();
        let long = broken_args[1].trim();
        // for i in options.split(" ") {
        self.short_hash_table.insert(short.to_string(), long.to_string());
        let mut param_type = String::new();
        if let Some(param_d) = args.get(1){
            param_type = String::from(param_d.to_owned());
        }
        if args.len() > 1 && ["<>", "[]", "<...>", "[...]"].contains(&param_type.trim()) == false {
            panic!("Error : unknown param type {}", param_type)
        }
        let option : String = long.trim().to_owned()+" "+param_type.trim();
        self.args_hash_table.insert(option.trim().to_owned(), value);
        // }
        return self
    }
    pub fn get_params_callback(&mut self, key : String) -> Option<&for<'a> fn(&'a Fli)>
    {
        if let Some(callback) = self.args_hash_table.get(&key){
                return Some(callback)
        }
        return None;
    }
    pub fn run(&self)  {
        let mut callbacks: Vec<for<'a> fn(&'a Fli)> = vec![];
        let mut counter: usize = 0;
        for mut arg in self.args.clone()
        {
            // a value based param must start with a - either -- or -
            if arg.starts_with("-") {
                arg = self.get_callable_name(arg);
                for optional_template in ["", "[]", "[...]"]
                {
                    // check if it need a required param
                    let find = &String::from(format!("{arg} {optional_template}"));
                    if let Some(callback) = self.args_hash_table.get(find.trim()){
                        callbacks.push(*callback);
                        // continue;
                    }
                }
                for required_template in ["<>", "<...>"]
                {
                    // check if it need a required param
                    let find = &String::from(format!("{arg} {required_template}"));
                    if let Some(callback) = self.args_hash_table.get(find.trim()){
                        // make sure a value is passed in else it should show error/help
                        if !self.has_a_value(arg.trim().to_string())
                        {
                            panic!("{}", format!("Why Invalid syntax : {arg}  does not have a value"));
                        }
                        callbacks.push(*callback);
                    }
                }
            }
            else{
                if let Some(command_struct) = self.cammands_hash_tables.get(arg.trim()){
                    command_struct.run();
                    break;
                }
            }
            counter += 1;
        }
        self.run_callbacks(callbacks);
    }
    fn has_a_value(&self, arg_name : String) -> bool
    {
        let mut counter = 0;
        let binding = self.get_callable_name(arg_name);
        let arg_full_name = binding.trim();
        for arg in &self.args{
            if  self.get_callable_name(arg.to_string()) == arg_full_name{
                if let Some(value) = self.args.get(counter+1)
                {
                    if !value.contains("-") {
                        return true;
                    }
                }
            }
            counter+=1;
        }
        return false;
    }
    fn run_callbacks(&self, callbacks : Vec<for<'a> fn(&'a Fli)>)
    {
        for callback in callbacks.clone()
        {
            callback(self)
        }
    }
    /**
     * Gets the Long name for a short arg
     */
    fn get_callable_name(&self, arg: String) -> String
    {
        let mut arg_template: String = String::from(format!("{}", arg));
        if let Some(long_name) = self.short_hash_table.get(&arg){
            arg_template = long_name.to_string();
        }
        if !arg_template.contains("--"){
            arg_template = String::from(format!("--{}", arg));
        }
        return arg_template;
    }
    pub fn get_values(&self, arg: String) -> Result<Option<Vec<String>>, &str>
    {
        let mut values : Vec<String> = vec![];
        let mut arg_name: String = self.get_callable_name(arg);
        // if the argument does not need a param then dont return none
        if let Some(_) = self.args_hash_table.get(&arg_name){
            return Err("Does not expect a value");
        }
        let mut counter = 1;
        for mut i in self.args.clone()
        {
          i = self.get_callable_name(i);
          if i != arg_name{
            counter+=1;
            continue;
          }
            let binding = &String::from(format!("{} []", arg_name));
            if let Some(_) = self.args_hash_table.get(binding){
                if let Some(v) = self.args.get(counter) {
                    if v.starts_with("-") {
                        return Err("No value passed");
                    }
                    values.push(v.to_string());
                    break;
                }
            }
            let binding = &String::from(format!("{} <>", arg_name));
            if let Some(_) = self.args_hash_table.get(binding){
                if let Some(v) = self.args.get(counter) {
                    if v.starts_with("-") {
                        return Err("No value Passed 2")
                    }
                    values.push(v.to_string());
                    break;
                }
            }
            let binding = &String::from(format!("{} [...]", arg_name));
            if let Some(_) = self.args_hash_table.get(binding){
                if let Some(params) = self.args.get((counter)..self.args.len())
                {
                    for i in params
                    {
                        if i.starts_with(&"-".to_string()) {
                            break;
                        }
                        values.push(i.to_string());
                    }
                    
                }
            }
            let binding = &String::from(format!("{} <...>", arg_name));
            if let Some(_) = self.args_hash_table.get(binding){
                if let Some(params) = self.args.get((counter)..self.args.len())
                {
                    for i in params
                    {
                        if i.starts_with(&"-".to_string()) {
                            break;
                        }
                        values.push(i.to_string());
                    }
                    
                }
            }
          counter += 1;
        }
        if values.len() > 0{
            return Ok(Some(values));
        }
        return Ok(None);
    }
    pub fn is_passed(&self, param : String) -> bool
    {
        return self.args.contains(&param);
    }
}

