<!-- chanagelog.md -->

# CHANGELOG
## 0.0.5
- Added support to set app version
    - `app.set_version("0.0.5")`
    - if using `init_from_toml` the version will be set from the toml file
- Properly handle the help message
    - Printing list of similar command if a command is not found
    - Printing the help message if the help command is found