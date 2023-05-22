# FLI

A library for creating commnad line apps in rust

```rust
fn main(){
    let mut app : &Fli = Fli::init("my app");
    app.option("-n --name <>", |x : &Fli| {});
    app.option("-g --greet", |x : &Fli| {
        match x.get_values("--name".to_owned()){
            Ok(option) => {
                if let Some(v) = option{ println!("Good day {?}", v[0]);}
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
coming soon
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
    let mut app = Fli::init("app-name");
}
```
### Adding Options

```rust
fn main(){
    let mut app = Fli::init("app-name");
    app.option("-g --greet", greet);
    app.option("-n --name, <>" |x|{});
}

// callback for the greet param
fn greet(x: &Fli){
    match x.get_values("--name".to_owned()){
        Ok(option) => {
            if let Some(v) = option{ println!("Good day {?}", v[0]);}
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
    let mut app = Fli::init("app-name");
    let moveCommand = app.command("move");
    moveCommand.option("-d --dir, <...>", move_file);
    app.option("-g --greet", greet);
    app.option("-n --name, <>" |x|{});
    app.run();
}
fn move_file(x){ /*code to move file"*/ }
fn greet(x){ /*code to greet "*/ }
```
Then you would run the command like this
```shell
$ cargo run -- move -d .
file moved successfully in 0.8s
```

### Doing it in a procedual way
Like commander.js you can also do it in a procedual way

```rust
fn main(){
    //  doing it procedual way
    let mut app = Fli::init("app-name");
    let moveCommand = app.command("move");
    // the [...] means accept optional multiple
    moveCommand.option("-d --dir, [...]", |x){}; // pass in an empty callback
    app.option("-g --greet", |x|{});
    app.option("-n --name, <>" |x|{});
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
  - First `arg_and_data` : This is a format template of how the avaliable arguments for a command would be being in a format `-a --arg` or `-a --arg, data`
