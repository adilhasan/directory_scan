# directory_scan
This program essentially wraps the WalkDir crate in order to scan a directory, or optionally recursively scan a directory and list the files in each directory in reverse chronological order. The list contains the file-path, size and created and modified times.

The purpose of the program is to identify old files with a view to cleaning up the disk space used.

## Install

- Make sure you have a new version of rust and cargo installed (the easiest is to install and use the [rustup](https://rustup.rs) command).

- Download the package and run the command `cargo build` to build the program.

- Edit the `config/scan.toml` file inserting into the `exclude` list the list of directories you want to exclude from a scan.

- To run the program:
```
cargo run -- -c <config file> <root-directory-path-to-scan>
```

The program will output a list of files in the form:
```
file-path, size, created-time, modified time
```

