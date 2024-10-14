<!-- chanagelog.md -->

# CHANGELOG

## 0.1.0
- Added new `init_fli_from_toml` macro to allow initializing the app from a toml file
    - `init_fli_from_toml!` macro will read the toml file and initialize the app with the values
- Deprecated `init_from_toml` method
    - `init_from_toml` method is deprecated and will be removed in the next major release

## 0.0.10
- Added auto version option to hrlp print version using `--version` or `-v`
## 0.0.9
- Fix env var issue 
    - used `env!` macro to get the toml details from the `Cargo.toml` file instead of `std::env::var` to allow compile time env instead of runtime

## 0.0.8
- Fix issue on removing index out of bounds when getting values

## 0.0.7
- Added support to set app version
    - `app.set_version("0.0.5")`
    - if using `init_from_toml` the version will be set from the toml file
- Properly handle the help message
    - Printing list of similar command if a command is not found
    - Printing the help message if the help command is found