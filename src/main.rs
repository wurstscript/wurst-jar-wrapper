
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::process;

// This file is build by the makefile using cargo-make.
const WS_JAR: &str = include!("jar.tmp");

fn fetch_paths_from_environment(possible_paths: &mut Vec<String>) {
    if let Ok(path) = env::var("JAVA_HOME") {
        possible_paths.push(path);
    };

    if let Ok(vals) = env::var("PATH") {
        let _ = vals
            .split(';')
            .filter(|&p| p.to_lowercase().contains("java"))
            .map(|p| possible_paths.push(p.to_owned()))
            .count();
    };
}

fn get_java(paths: &[String]) -> Result<String, ()> {
    for path in paths {
        let opt = format!("{}\\javaw.exe", path);
        if Path::new(&opt).exists() {
            return Ok(opt);
        }

        let opt2 = format!("{}\\bin\\javaw.exe", path);
        if Path::new(&opt2).exists() {
            return Ok(opt2);
        }
    }

    Err(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let jar_path = format!(
        "{}{}{}",
        dirs::home_dir().expect("Failed to get user home").display(),
        "\\.wurst\\",
        WS_JAR
    );

    {
        // Test the file can be opened, then let it close.
        File::open(&jar_path).expect(
            &format!("{} could not be found!", WS_JAR)
        );
    }

    let java_path = {
        let mut possible_paths: Vec<String> = vec![];
        fetch_paths_from_environment(&mut possible_paths);
        get_java(&possible_paths).expect("Failed to locate javaw.exe")
    };

    let args = env::args().skip(1).collect::<Vec<String>>();

    let mut subproc = process::Command::new(java_path)
        .arg("-jar")
        .arg(jar_path)
        .args(args)
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to open a subprocess");

    if let Some(out) = subproc.stdout.take() {
        std::io::BufReader::new(out)
            .lines()
            .into_iter()
            .for_each(|line| {
                println!("{}", line.unwrap_or("".into()));
            });
    }

    if let Some(err) = subproc.stderr.take() {
        std::io::BufReader::new(err)
            .lines()
            .into_iter()
            .for_each(|line| {
                println!("{}", line.unwrap_or("".into()));
            });
    }

    process::exit(
        subproc
            .wait()
            .expect("failed subprocess")
            .code()
            .unwrap_or(-1),
    );
}
