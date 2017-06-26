#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process;


#[derive(Deserialize)]
struct Config {
    initial_heap_size: Option<u32>,
    maximum_heap_size: Option<u32>,
    thread_stack_size: Option<u32>,
}


const WS_JAR: &str = "wurstscript.jar";


fn fetch_paths_from_environment(possible_paths: &mut Vec<String>) {
    match env::var("JAVA_HOME") {
        Ok(path) => possible_paths.push(path),
        _ => (),
    };

    match env::var("PATH") {
        Ok(vals) => {
            let _ = vals.split(";")
                        .filter(|&p| p.to_lowercase().contains("java"))
                        .map(|p| possible_paths.push(p.to_owned()))
                        .count();
        },
        _ => (),
    };
}

fn get_java(paths: &Vec<String>) -> String {
    for path in paths {
        let opt = format!("{}\\java.exe", path);
        if Path::new(&opt).exists() {
            return opt;
        }

        let opt2 = format!("{}\\bin\\java.exe", path);
        if Path::new(&opt2).exists() {
            return opt2;
        }
    }

    println!("Failed to locate java.exe");
    process::exit(1);
}

fn main() {
    let mut java_args = String::new();
    let _ = match File::open("wrapper_config.toml") {
        Ok(mut file) => {
            let mut conts = String::new();
            file.read_to_string(&mut conts).unwrap_or(0);
            let c: Config = match toml::from_str(&conts) {
                Ok(f) => f,
                _ => {
                    println!("Found wrapper config file but unable to parse.");
                    process::exit(1);
                }
            };

            if let Some(init) = c.initial_heap_size {
                java_args.push_str(&format!("-Xms{}m ", init));
            }

            if let Some(max) = c.maximum_heap_size {
                java_args.push_str(&format!("-Xmx{}m ", max));
            }

            if let Some(stack) = c.thread_stack_size {
                java_args.push_str(&format!("-Xss{}m ", stack));
            }
        },
        _ => (),
    };

    println!("Using args {}", java_args);

    if !Path::new(WS_JAR).exists() {
        println!("Failed to locate {}.", WS_JAR);
        process::exit(1);
    }

    let mut possible_paths: Vec<String> = vec!();

    fetch_paths_from_environment(&mut possible_paths);

    let args = env::args().skip(1).collect::<Vec<String>>().join(" ");
    println!("{}", args);

    let java_path = get_java(&possible_paths);

    let subproc = process::Command::new(java_path).arg("-jar")
                                                  .arg(WS_JAR)
                                                  .arg(args)
                                                  .output()
                                                  .unwrap();
    println!("{}\n\n{}",
             String::from_utf8(subproc.stdout).unwrap(),
             String::from_utf8(subproc.stderr).unwrap());
}
