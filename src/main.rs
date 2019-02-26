use regex::Regex;

use std::env;
use std::path::{Path, PathBuf};

// 再帰関数ではなく、mpscによるマルチスレッド処理でも良さそう。

enum Target<'a> {
    Simple(&'a str),
    SimpleExt(&'a str),
    WithWild(Regex),
    WithWildExt(Regex),
}
impl<'a> Target<'a> {
    fn from_file(target: &'a str) -> Self {
        let with_extension = {
            if !target.contains('.') {
                false
            } else if target.starts_with('.') {
                target.matches('.').count() >= 2
            } else {
                true
            }
        };
        let with_wildcard = target.contains('*');
        match (with_extension, with_wildcard) {
            ( true,  true) => {
                let target = String::from("^") + &target.replace("*", ".*") + "$";
                Target::WithWildExt(Regex::new(&target).unwrap())
            }
            ( true, false) => { Target::SimpleExt(target) }
            (false,  true) => {
                let target = String::from("^") + &target.replace("*", ".*") + "$";
                Target::WithWild(Regex::new(&target).unwrap())
            }
            (false, false) => { Target::Simple(target) }
        }
    }
    fn from_dir(target: &'a str) -> Self {
        if target.contains('*') {
            let target = String::from("^") + &target.replace("*", ".*") + "$";
            let re = Regex::new(&target).unwrap();
            Target::WithWild(re)
        } else {
            Target::Simple(target)
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
                    println!("target: \"{}\"\n", file_name);
                    let result = search_file(times, Path::new("./"), &Target::from_file(&file_name));
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
                    let result = search_dir(times, Path::new("./"), &Target::from_dir(&dir_name));
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
                println!("{}", s);
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
                match target {
                    Target::Simple(target) => {
                        if let Some(file_stem) = Path::new(file_name).file_stem() {
                            if cmp_osstr_refstr(file_stem, target) {
                                vec.push(path);
                            }
                        }
                    }
                    Target::SimpleExt(target) => {
                        if cmp_osstr_refstr(file_name, target) {
                            vec.push(path);
                        }
                    }
                    Target::WithWild(re) => {
                        if let Some(file_stem) = Path::new(file_name).file_stem() {
                            if let Some(name) = file_stem.to_str() {
                                if re.is_match(name) {
                                    vec.push(path);
                                }
                            }
                        }
                    }
                    Target::WithWildExt(re) => {
                        if let Some(name) = file_name.to_str() {
                            if re.is_match(name) {
                                vec.push(path)
                            }
                        }
                    }
                }
            }
        } else if n > 0 {
            let mut result = search_file(n - 1, path.as_path(), target);
            vec.append(&mut result);
        }
    }
    vec
}

fn search_dir(n: u32, path: &Path, target: &Target) -> Vec<PathBuf> {
    let mut vec = vec![];
    let mut entries = path.read_dir().expect(&format!("Failed to read list. At {:?}", path));
    while let Some(Ok(entry)) = entries.next() {
        let path = entry.path();
        if path.is_dir() {
            if let Some(dir_name) = path.file_name() {
                match target {
                    Target::Simple(target) => {
                        if cmp_osstr_refstr(dir_name, target) {
                            vec.push(path.clone());
                        }
                    }
                    Target::WithWild(re) => {
                        if let Some(name) = dir_name.to_str() {
                            if re.is_match(name) {
                                vec.push(path.clone());
                            }
                        }
                    }
                    _ => {}
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

use std::ffi::OsStr;
fn cmp_osstr_refstr(osstr: &OsStr, refstr: &str) -> bool {
    if let Some(s) = osstr.to_str() {
        s == refstr
    } else { false }
}
