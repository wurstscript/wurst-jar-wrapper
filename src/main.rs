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
    java_path:         Option<String>,
}


const WS_JAR: &str = "wurstscript.jar";


fn fetch_paths_from_environment(possible_paths: &mut Vec<String>) {
    if let Ok(path) = env::var("JAVA_HOME") {
        possible_paths.push(path);
    };

    if let Ok(vals) = env::var("PATH") {
        let _ = vals.split(';')
                    .filter(|&p| p.to_lowercase().contains("java"))
                    .map(|p| possible_paths.push(p.to_owned()))
                    .count();
    };
}

fn get_java(paths: &[String]) -> String {
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
    let mut java_args = vec!();
    let mut java_path = String::new();

    if let Ok(mut file) = File::open("wrapper_config.toml") {
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
            java_args.push(format!("-Xms{}m ", init));
        }

        if let Some(max) = c.maximum_heap_size {
            java_args.push(format!("-Xmx{}m ", max));
        }

        if let Some(stack) = c.thread_stack_size {
            java_args.push(format!("-Xss{}m ", stack));
        }

        if let Some(path) = c.java_path {
            if File::open(&path).is_err() {
                println!("Found configured java path but file not found.  \
                          Consider commenting out or deleting java_path from \
                          your wrapper_config.toml to automatically detect \
                          a java path.");
                process::exit(1);
            }

            java_path = path;
        }
    };

    if !Path::new(WS_JAR).exists() {
        println!("Failed to locate {}.", WS_JAR);
        process::exit(1);
    }

    if java_path.is_empty() {
        let mut possible_paths: Vec<String> = vec!();
        fetch_paths_from_environment(&mut possible_paths);
        java_path = get_java(&possible_paths);
    }

    let args = env::args().skip(1).collect::<Vec<String>>();
    println!("Forwarded run arguments: {:?}", args);

    let subproc = process::Command::new(java_path).arg("-jar")
                                                  .arg(WS_JAR)
                                                  .args(args)
                                                  .args(java_args)
                                                  .output()
                                                  .unwrap();
    println!("{}\n\n{}",
             String::from_utf8(subproc.stdout).unwrap(),
             String::from_utf8(subproc.stderr).unwrap());

    process::exit(subproc.status.code().unwrap_or(-1));
}
