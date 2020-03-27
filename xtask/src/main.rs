use std::{
    env, io,
    path::{Path, PathBuf},
    process::Command,
};

fn main() -> io::Result<()> {
    match env::args().nth(1).as_deref() {
        Some(command) => match command {
            "dev" => dev()?,
            other => eprintln!("Missing task '{}'", other),
        }
        _ => eprintln!("Expected task name, eg: cargo xtask dev")
    }

    Ok(())
}

fn dev() -> io::Result<()> {
    Command::new("systemfd")
        .current_dir(project_root())
        .args(&[
            "--no-pid",
            "-s",
            &format!("http::{}", server_port()),
            "--",
            "cargo",
            "watch",
            "-x",
            "run",
            "-i",
            "xtask",
            "-i",
            &env!("CARGO_MANIFEST_DIR"),
            "-i",
            ".cargo",
            "-i",
            ".gitignore",
            "-i",
            "Procfile",
        ])
        .status()?;
    Ok(())
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

fn server_port() -> u16 {
    env::var("PORT")
        .map(|port| port.parse().expect("PORT must be a number"))
        .unwrap_or(3000)
}
