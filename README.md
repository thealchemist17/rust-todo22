# rust-todo22
*Step 1*
- Use [clap](https://github.com/clap-rs/clap) to parse command line arguments
- Add an argument `-f` to specify filename using clap library
- Now try to make tests work again by specifing a named temporary file using [tempfile](https://github.com/Stebalien/tempfile)
- Add tests for edit, remove and clear

*Step 2*
- Define a struct that represents a Todo entry
- Add handling of Priority and State
- Use serde to serialize and deserialize 
- Optionally use a config file to specify the path/name of the todo file

*Step 3*
- Adapt the list subcommand to allow sorting and filtering

*Step 4*
- Add TUI