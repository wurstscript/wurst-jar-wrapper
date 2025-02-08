use std::env;
use std::fs::File;
use std::io::BufRead;
use std::path::PathBuf;
use std::process;

use eyre::{eyre, Context};
use which::which;

// This file is built by the makefile using cargo-make.
const WS_JAR: &str = include!("jar.tmp");

fn fetch_java_paths_from_environment() -> Vec<PathBuf> {
    env::var("JAVA_HOME")
        .into_iter()
        .map(PathBuf::from)
        .chain(
            which("java")
                .into_iter()
                .flat_map(|dir| dir.parent().map(|path| path.to_path_buf())),
        )
        .collect()
}

fn get_java_exe_from_java_dirs(paths: &[PathBuf]) -> eyre::Result<PathBuf> {
    paths
        .iter()
        .flat_map(|path| [path.join("javaw.exe"), path.join("bin").join("javaw.exe")].into_iter())
        .find(|path| path.exists())
        .ok_or(eyre!(
            "Couldn't find a javaw.exe in JAVA_HOME or in the directory obtained with `which java`"
        ))
}

fn main() -> eyre::Result<()> {
    let jar_path = dirs::home_dir()
        .expect("Failed to get user home")
        .join(".wurst")
        .join(WS_JAR);

    {
        // Test the file can be opened, then let it close.
        File::open(&jar_path).with_context(|| {
            format!(
                "Tried to open the configured jar path ({:?}) but it failed!",
                jar_path
            )
        })?;
    }

    let java_path = {
        let possible_paths = fetch_java_paths_from_environment();
        get_java_exe_from_java_dirs(&possible_paths)?
    };

    let args = env::args().skip(1).collect::<Vec<String>>();

    let mut subproc = process::Command::new(java_path)
        .arg("-jar")
        .arg(jar_path)
        .args(args)
        .stdout(std::process::Stdio::piped())
        .spawn()?;

    if let Some(out) = subproc.stdout.take() {
        std::io::BufReader::new(out).lines().for_each(|line| {
            println!("{}", line.unwrap_or_else(|err| err.to_string()));
        });
    }

    if let Some(err) = subproc.stderr.take() {
        std::io::BufReader::new(err).lines().for_each(|line| {
            println!("{}", line.unwrap_or_else(|err| err.to_string()));
        });
    }

    process::exit(subproc.wait()?.code().ok_or_else(|| {
        eyre!("Failed to read the exit code of subprocess - was it masked by a signal handler?")
    })?)
}
