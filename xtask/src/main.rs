use std::{
    env, fs,
    path::{Path, PathBuf},
    process::{self, Command},
};

fn main() {
    if let Err(err) = try_main() {
        eprintln!("{}", err);
        process::exit(-1);
    }
}

fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    let task = env::args().nth(1);
    match task.as_ref().map(|it| it.as_str()) {
        Some("test") => perform_test()?,
        _ => print_help(),
    };
    Ok(())
}

fn perform_test() -> Result<(), Box<dyn std::error::Error>> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let mut manifest_paths: Vec<PathBuf> = vec![PathBuf::from("Cargo.toml")];

    // looking for other crates in `crates` dir
    for entry in fs::read_dir("crates")? {
        let dir = entry?;
        if dir.file_type()?.is_dir() {
            let manifest = dir.path().join("Cargo.toml");
            if let Some(metadata) = fs::metadata(&manifest).ok() {
                if metadata.is_file() {
                    manifest_paths.push(manifest);
                }
            }
        }
    }

    for manifest_path in manifest_paths {
        println!("Performing test for {}", manifest_path.to_string_lossy());

        let is_nextest_enabled = env::var("DISABLE_NEXTEST")
            .map(|v| v != "1")
            .unwrap_or_else(|_| true);

        let status = Command::new(cargo.clone())
            .current_dir(project_root())
            .args(if is_nextest_enabled {
                vec![
                    "nextest",
                    "run",
                    "--manifest-path",
                    manifest_path.to_str().unwrap(),
                ]
            } else {
                vec!["test", "--manifest-path", manifest_path.to_str().unwrap()]
            })
            .status()?;

        if !status.success() {
            Err("cargo test failed")?;
        }
    }

    Ok(())
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}

fn print_help() {
    eprintln!("Tasks:");
    eprintln!("test\t\ttests all crates")
}
