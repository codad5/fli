# FLI

A library for creating command line apps in rust inspired by the like of [commander.js](https://github.com/tj/commander.js)

## Changes
For stable changes check the [CHANGELOG.md](https://github.com/codad5/fil/blob/master/CHANGELOG.md) file

For unstable changes check the [CHANGELOG.md](https://github.com/codad5/fil/blob/dev/CHANGELOG.md) file

```rust
fn main(){
    let mut app : Fli = Fli::init("my app", "my app description");
    app.option("-n --name, <>", "Name to call you", |x : &Fli| {});
    app.option("-g --greet", "greeting", |x : &Fli| {
        match x.get_values("name".to_owned() /* passing (--name, -n or n) would work*/){
            Ok(value) => {
                println!("Good day {?}", value[0])
            },
            Err(_) => {},
        }
    });
    // required for the app  to work 
    app.run();
}
```

then run it like this
```bash
$ cargo run -- -g -n james
Good day james
```

## Getting started
### Installtion
```bash
cargo add fli
```
OR
```toml
[dependencies]
fli = "0.0.1"
```

### Import
```rust
extern crate fli;
use fli::Fli;
fn main(){
    println!("Happy Coding");
}
```
### Create an App Instance 
```rust
fn main(){
    let mut app = Fli::init("app-name", "app description");
}
```
### Adding Options

```rust
fn main(){
    let mut app = Fli::init("app-name");
    app.option("-g --greet", "to make a greeting", greet);
    app.option("-n --name, <>", "to set your name", |x|{});
}

// callback for the greet param
fn greet(x: &Fli){
    match x.get_values("--name".to_owned()){
        Ok(option) => {
            println!("Good day {?}", v[0]);
        },
        Err(_) => {},
    }
}
```

### Run your app
```rust
fn main(){
    //... other code
    app.run();
}
```

### Adding a new Command Set
You can also add a new command set using the command method
```rust
fn main(){
    let mut app = Fli::init("app-name", "app-description");
    let moveCommand = app.command("move", "move files");
    moveCommand.option("-p --path, <...>", "path to files to be moved", move_file);
    app.option("-g --greet",  "to make a greeting", greet);
    app.option("-n --name, <>", "to set your name", |x|{});
    app.run();
}
fn move_file(x){ /*code to move file"*/ }
fn greet(x){ /*code to greet "*/ }
```
Then you would run the command like this
```shell
$ cargo run -- move -d .
file(s) moved successfully in 0.8s
```

### Doing it in a procedual way
Like commander.js you can also do it in a procedual way

```rust
fn main(){
    //  doing it procedual way
    let mut app = Fli::init("app-name", "app descripton");
    let moveCommand = app.command("move", "move files");
    // the [...] means accept optional multiple
    moveCommand.option("-p --path, <...>", "path to files to be moved", move_file);
    app.option("-g --greet",  "to make a greeting", |x|{});
    app.option("-n --name, <>", "to set your name", |x|{});
    app.run();

    if app.is_passed("--greet"){
        if app.has_a_value("-n"){/* greet person with name*/}
        else { /* freet without name*/ }
    }
}
```

## Explaining all `app : Fli` methods
**Note:**
> All `app : Fli` methods are avaliable as `app : &Fli` methods
- `app.option(arg_and_data, callback)` : 
This method takes in 2 param 
  - First `arg_and_data` : This is a format template of how the avaliable arguments for a command would be being in a format `-a --arg` or `-a --arg, data` where `-a` is the short  form of the argument and `--arg` is the long form of the argument. `--data` is the acceptable data type and it is seperated by a **comma** `,` , if not passed then the arg does not need a data type
  
  | symbol | meaning |
  |:---:|:---|
  | [] | This means it needs one optional data|
  | <> | This means it needs one required data |
  | [...] | This means it can take in many optional data |
  | <...> | This means it needs at least one data, can take more |


- `app.commad(command_name)` : 
This is to create a new command with its own option and param like this
Get the app general help option
```shell
$ my-app --help
```
Get the move command help option
```shell
$ my-app move --help # a new command called
```
The app command returns a new refrence instance of Fli

