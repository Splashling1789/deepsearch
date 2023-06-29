use std::fs::{metadata, read_dir};
use std::path::PathBuf;
macro_rules! help {
    () => {println!("Usage: deepsearch (file) [parameters]\n-p, --path (path): specify a start path.\n-i, --ignore-hidden: ignores hidden directories such as \".local\"\n-v, --verbose: prints the process\n-c, --show-count: shows the number of files read")};
}

fn deep_search(dir: &PathBuf,  file: &String, ignore_hidden: bool, verbose: bool, show_count: bool, count: &mut u32) ->  Option<PathBuf> {
    match read_dir(dir) {
        Ok(p) => {
            for obj in p {
                match obj {
                    Ok(o) => {
                        match metadata(o.path()) {
                            Ok(meta) => {
                                if verbose { println!("Reading: {}", o.path().to_str().expect("NaN"));}
                                if show_count {*count += 1}
                                if meta.is_file() && o.file_name().to_str().expect("NaN") == (*file).as_str() {
                                    return Some(o.path());
                                }
                                else if meta.is_dir() {
                                    if ignore_hidden && o.file_name().to_str().expect("NaN").starts_with(".") {
                                        continue
                                    }
                                    else{
                                        match deep_search(& o.path(), file, ignore_hidden, verbose, show_count, count) {
                                            Some(f) => return Some(f),
                                            None => continue,
                                        }
                                    }
                                }
                                else {
                                    continue;
                                }
                            }
                            Err(e) => println!("Unable to access metadata of file \"{}\"\nError: {}", o.path().to_str().expect("NaN"), e)
                        }
                    }
                    Err(e) => println!("Unable to access the entries of directory\nError: {}", e)
                }
            }
        }
        Err(e) => println!("Unable to read the directory \"{}\"\nError: {}", dir.to_str().expect("NaN"), e)
    }
    None
}

fn main() {
    let mut path = PathBuf::from("/");
    let mut filename = String::new();

    let args: Vec<String> = std::env::args().collect();
    if let Some(f) = args.get(1) {
        filename = String::from(f);
    }
    else {
        help!();
        std::process::exit(0);
    }
    /*if let Some(p) = args.get(2) {
        path = PathBuf::from(p);
    }*/
    let ignore_hidden = args.contains(& String::from("--ignore-hidden")) || args.contains(& String::from("-i"));
    let verbose = args.contains(&String::from("--verbose")) || args.contains(&String::from("-v"));
    let show_count = args.contains(&String::from("-c")) || args.contains(&String::from("--show-count"));
    let mut count:u32 =0;
    if args.contains(&String::from("--path")) || args.contains(&String::from("-p")) {
        for i in 1..args.len() {
            match args.get(i) {
                Some(a) => {
                    if  String::cmp(a, &String::from("-p")).is_eq() || String::cmp(a, &String::from("--path")).is_eq()  {
                        path = PathBuf::from(args.get(i+1).expect("Argument -p or --path was given incorrectly").to_string().as_str());
                    }
                }
                None => println!("Unable to read")
            }

        }
    }
    println!("Searching: {} in {}", filename, path.to_str().expect("NaN"));
    match deep_search(&path, &filename, ignore_hidden, verbose, show_count, &mut count) {
        Some(result) => {
            println!("Found {} in {}", filename, result.to_str().expect("NaN"));
            if show_count {println!("{} files counted", count);}
        }
        None => {
            println!("File {} could not found", filename);
        }
    }
}
