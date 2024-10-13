<!-- chanagelog.md -->

# CHANGELOG

## 0.0.7
- Fix env var issue 
    - used `env!` macro to get the toml details from the `Cargo.toml` file instead of `std::env::var` to allow compile time env instead of runtime

## 0.0.6
- Fix issue on removing index out of bounds when getting values

## 0.0.5
- Added support to set app version
    - `app.set_version("0.0.5")`
    - if using `init_from_toml` the version will be set from the toml file
- Properly handle the help message
    - Printing list of similar command if a command is not found
    - Printing the help message if the help command is found