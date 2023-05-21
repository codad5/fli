
use std::{env, collections::HashMap};

pub struct Fli {
    args : Vec<String>,
    pub args_hash_table: HashMap<String, fn(app : &Self)>,
}

enum FliError{
    InvalidArg(String),
    EmptyParam(String)
}

impl Fli {
    pub fn init() -> Self {
        println!("{:?}", env::args());
        Self {
            args: env::args().collect(),
            args_hash_table: HashMap::new(),
        }
    }

    pub fn option(&mut self, key: String, value: fn(app : &Self)) -> &Fli{
        let args : Vec<&str> = key.split(",").collect();
        let mut options = String::new();
        if let Some(opts)  = args.get(0){
            options = String::from(opts.to_owned());
        }
        for i in options.split(" ") {
            let mut param_type = String::new();
            if let Some(param_d) = args.get(1){
                param_type = String::from(param_d.to_owned());
            }
            if args.len() > 1 && ["<>", "[]", "<...>", "[...]"].contains(&param_type.trim()) == false {
                panic!("Error : unknown param type {}", param_type)
            }
            let option : String = i.trim().to_owned()+" "+param_type.trim();
            self.args_hash_table.insert(option.trim().to_owned(), value);
        }
        return self
    }
    pub fn get_command(&mut self, key : String) -> Option<&for<'a> fn(&'a Fli)>
    {
         if let Some(callback) = self.args_hash_table.get(&key){
                return Some(callback)
        }
        return None;
    }
    pub fn run(&self)  {
        let mut counter: usize = 0;
        let mut callbacks: Vec<for<'a> fn(&'a Fli)> = vec![];
        for i in self.args.clone()
        {
            if i.starts_with("-") {
                let find = &i;
                if let Some(callback) = self.args_hash_table.get(find){
                    callbacks.push(*callback);
                }
                let find = &String::from(format!("{} []", i));
                if let Some(callback) = self.args_hash_table.get(find){
                    callbacks.push(*callback);
                }
                
                let find = &String::from(format!("{} <>", i));
                if let Some(callback) = self.args_hash_table.get(find){
                    if let Some(value) = self.args.get(counter+1)
                    {
                        if value.contains("-") {
                            panic!("Invalid syntax");
                        }
                        callbacks.push(*callback);
                    }
                    else{
                        panic!("Invalid syntax");
                    }
                }

                let find = &String::from(format!("{} [...]", i));
                if let Some(callback) = self.args_hash_table.get(find){
                    callbacks.push(*callback);
                }
                let find = &String::from(format!("{} <...>", i));
                if let Some(callback) = self.args_hash_table.get(find){
                    if let Some(value) = self.args.get(counter+1)
                    {
                        if value.contains("-") {
                            panic!("Invalid syntax");
                        }
                        callbacks.push(*callback);
                    }
                    else{
                        panic!("Invalid syntax");
                    }
                }
            }
            counter += 1;
        }
        self.run_callbacks(callbacks);
    }
    fn run_callbacks(&self, callbacks : Vec<for<'a> fn(&'a Fli)>)
    {
        for callback in callbacks.clone()
        {
            callback(self)
        }
    }

    pub fn get_values(&self, args: String) -> Result<Option<Vec<String>>, &str>
    {
        let mut values : Vec<String> = vec![];
        let find = &String::from(format!("{}", args));
        // if the argument does not need a param then dont return none
        if let Some(_) = self.args_hash_table.get(find){
            return Err("Does not expect a value");
        }
        let mut counter = 1;
        for i in self.args.clone()
        {
          if i != args{
            counter+=1;
            continue;
          }
          if i.starts_with("-")  {
            let find = &String::from(format!("{} []", args));
            if let Some(_) = self.args_hash_table.get(find){
                if let Some(v) = self.args.get(counter) {
                    if v.starts_with("-") {
                        return Err("No value passed 1");
                    }
                    values.push(v.to_string());
                    break;
                }
            }
            let find = &String::from(format!("{} <>", args));
            if let Some(_) = self.args_hash_table.get(find){
                if let Some(v) = self.args.get(counter) {
                    if v.starts_with("-") {
                        println!("{v}, {i}, {args}");
                        return Err("No value Passed 2")
                    }
                    values.push(v.to_string());
                    break;
                }
            }
            let find = &String::from(format!("{} [...]", args));
            if let Some(_) = self.args_hash_table.get(find){
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
            let find = &String::from(format!("{} <...>", args));
            if let Some(_) = self.args_hash_table.get(find){
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
          }
          counter += 1;
        }
        return Ok(Some(values));
    }
    pub fn is_passed(&self, param : String) -> bool
    {
        return self.args.contains(&param);
    }
}

