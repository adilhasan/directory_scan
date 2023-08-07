use std::{path::{Path, PathBuf}, time::SystemTime};
use clap::Parser;
use walkdir::WalkDir;
use serde::Deserialize;
use toml;
use std::fs;
use chrono::prelude::{DateTime, Utc};

#[derive(Debug, Default, Parser)]
struct ScanParams {
    /// Root path to scan
    path: String,
    /// Flag to indicate whether to recurse directories or not
    #[clap(short, long)]
    recursive: bool,
    /// The configuration file for the directory scanner
    #[clap(short, long)]
    config: String,
}

#[derive(Deserialize, Debug)]
struct ScanConfig {
    directories: Directories,
}

#[derive(Deserialize, Debug)]
struct Directories {
    exclude: Vec<String>,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct FileInfo {
    name: PathBuf,
    size: u64,
    created: SystemTime,
    modified: SystemTime,
}

impl FileInfo {
    pub fn new(name: PathBuf, size: u64,
            created: SystemTime, modified: SystemTime) -> Self {
                FileInfo {
                    name,
                    size,
                    created,
                    modified,
                }
    }
}

fn extract_metadata(path: &Path) -> FileInfo {
    let file_info = path.metadata().unwrap();
    FileInfo::new(path.to_path_buf(),
        file_info.len(),
        file_info.created().unwrap(),
        file_info.modified().unwrap()
    )
}

fn filter_folder(entry: &walkdir::DirEntry, config: &String) -> bool {
    let toml_str = fs::read_to_string(config)
        .expect("Failed to read config file");
    let scan_toml: ScanConfig = toml::from_str(&toml_str)
        .expect("Failed to deserialize the config toml file");
    
    let path_string = entry.path().to_path_buf().into_os_string().into_string().unwrap();
    if scan_toml.directories.exclude.contains(&path_string) {
        true
    } else {
        false
    }
}

fn iso8601(st: &std::time::SystemTime) -> String {
    let dt: DateTime<Utc> = st.clone().into();
    format!("{}", dt.format("%+"))
}

fn main() {

    let args = ScanParams::parse();
    let mut max_depth = 1000;
    let mut file_list: Vec<FileInfo> = Vec::new();

    if ! args.recursive {
        max_depth = 1;
    }
    for entry in WalkDir::new(args.path)
        .max_depth(max_depth).into_iter().filter_entry(|e| !filter_folder(e, &args.config)) {
        match entry {
            Ok(ent) => {
                if ent.path().is_file() {
                    let file_info = extract_metadata(ent.path());
                    file_list.push(file_info);
                }
            },
            Err(er) => println!("error {}", er)
        };
    }
    file_list.sort_by(|a, b| b.modified.cmp(&a.modified));
    for a_file in file_list.iter().rev() {
        println!("name {}, size {}, created {}, modified {}",
        a_file.name.display(), a_file.size,
        iso8601(&a_file.created), iso8601(&a_file.modified));
    }
}
