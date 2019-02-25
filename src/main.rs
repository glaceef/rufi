use regex::Regex;

use std::env;
use std::path::{Path, PathBuf};

// 再帰関数ではなく、mpscによるマルチスレッド処理でも良さそう。

enum Target<'a> {
    Simple(&'a str),
    SimpleExt(&'a str),
    WithWild(&'a str, Regex),
    WithWildExt(&'a str, Regex),
}
impl Target {
    fn from(target: &str) -> Self {
        let with_extension = {
            if !pattern.contains('.') {
                false
            } else if pattern.starts_with('.') {
                pattern.matches('.').len()
            }
            !pattern.starts_with('.') || pattern.starts_with('.') && ;
        };
        let with_wildcard = pattern.contains('*');
        match (is_file, with_wildcard) {
        }
    }
}

fn main(){
    let mut args = env::args();
    let _ = args.next();

    let mut times = 5;
    while let Some(command) = args.next() {
        match command.as_str() {
            "-n" => {
                if let Some(n) = args.next() {
                    times = n.parse().unwrap();
                } else {
                    error();
                }
            }
            "-f" | "--file" => {
                if let Some(file_name) = args.next() {
                    let target = if file_name.contains('*') { file_name.replace("*", ".*") } else { file_name };
                    println!("target: \"{}\"\n", target);
                    let result = search_file(times, Path::new("./"), &target);
                    print_result(&result);
                    match result.len() {
                        n if n == 1 => { println!("{} file was found.", n);   }
                        n if n > 1  => { println!("{} files were found.", n); }
                        _ => {}
                    }
                } else {
                    error();
                }
            }
            "-d" | "--dir" => {
                if let Some(dir_name) = args.next() {
                    println!("target: \"{}\"\n", dir_name);
                    let result = search_dir(times, Path::new("./"), &dir_name);
                    print_result(&result);
                    match result.len() {
                        n if n == 1 => { println!("{} directory was found.", n);    }
                        n if n > 1  => { println!("{} directories were found.", n); }
                        _  => {}
                    }
                } else {
                    error();
                }
            }
            "-h" | "--help" => {
                usage();
            }
            _ => {
                error();
            }
        }
    }
}

fn usage() {
    let usage = r"
    Usage:
        find [-n num] <-f | --file> name
        find [-n num] <-d | --dir>  name
        find [-h | --help]

      -n: Optional. Times of recursion. Set 5 as default.
      -f: Search files. Give one file name.
      -d: Search directories. Give one directory name.
      -h: Show usage.
    ";
    println!("{}", usage);
}

fn error() {
    println!("Invalid args. Give '-h' or '--help' command to show helps.");
}

fn print_result(result: &Vec<PathBuf>) {
    if result.is_empty() {
        println!("Not found");
    } else {
        for path in result {
            if let Some(s) = path.as_path().to_str() {
                println!("{}", to_os_string(s));
            }
        }
        println!("");
    }
}

fn search_file(n: u32, path: &Path, target: &Target) -> Vec<PathBuf> {
    let mut vec = vec![];
    let mut entries = path.read_dir().expect(&format!("Failed to read list. At {:?}", path));
    while let Some(Ok(entry)) = entries.next() {
        let path = entry.path();
        if path.is_file() {
            if let Some(file_name) = path.file_name() {
                target.
                // match Target {
                //     Target::Simple(target) => {
                //         if file_name == target {
                //             vec.push(path);
                //         }
                //     }
                //     Target::WithWild(target) => {
                //         target.is_match(target)
                //     }
                // }
            }
        } else if n > 0 {
            let mut result = search_file(n - 1, path.as_path(), target);
            vec.append(&mut result);
        }
    }
    vec
}

fn search_dir(n: u32, path: &Path, target: &impl AsRef<Path>) -> Vec<PathBuf> {
    let mut vec = vec![];
    let mut entries = path.read_dir().expect(&format!("Failed to read list. At {:?}", path));
    while let Some(Ok(entry)) = entries.next() {
        let path = entry.path();
        if path.is_dir() {
            if let Some(dir_name) = path.file_name() {
                if dir_name == target.as_ref() {
                    vec.push(path.clone());
                }
            }
            if n > 0 {
                let mut result = search_dir(n - 1, path.as_path(), target);
                vec.append(&mut result);
            }
        }
    }
    vec
}

#[cfg(target_os = "windows")]
fn func() {
    println!("Windows");
}

#[cfg(target_os = "macos")]
fn func() {
    println!("MacOS");
}
