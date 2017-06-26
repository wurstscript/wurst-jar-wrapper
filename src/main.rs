use std::env;
use std::path::Path;
use std::process;

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
