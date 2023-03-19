use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::process::{Command, ExitStatus};

pub fn run_command(cmd: String, workdir: Option<&str>) -> Result<ExitStatus, String> {
    let mut cmd = cmd.split_whitespace();

    let first_part = cmd.next().unwrap();

    let cmd = match workdir {
        Some(workdir) => Command::new(first_part)
            .args(cmd.collect::<Vec<&str>>())
            .current_dir(workdir)
            .status(),
        None => Command::new(first_part)
            .args(cmd.collect::<Vec<&str>>())
            .status(),
    };

    match cmd {
        Ok(res) => Ok(res),
        Err(err) => Err(format!("Error: {err}")),
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct PackageJson {
    name: String,
    version: String,
    private: bool,
    pub scripts: HashMap<String, String>,
    dependencies: HashMap<String, String>,
    devDependencies: HashMap<String, String>,
}

impl PackageJson {
    pub fn from_json(workdir: &str) -> Self {
        let packagejson_string =
            fs::read_to_string(workdir).expect("Couldn't read package.json, does it exist?");

        let data: PackageJson = serde_json::from_str(&packagejson_string).unwrap();

        data
    }

    pub fn add_data(mut self) -> Self {
        self.scripts.insert(
            "build".to_string(),
            "next build -o ./backend/static && cargo-build --manifest-path ./backend/Cargo.toml"
                .to_string(),
        );
        self.scripts.insert("dev".to_string(), "npm run build && concurrently --names \"next, cargo\" \"next dev\" \"cargo run --working-directory ./backend".to_string());

        self
    }

    pub fn write_to_file(&self, workdir: &str) -> serde_json::Result<()> {
        let f = fs::File::create(workdir)
            .expect("Couldn't create package.json, do you have enough space?");

        // let data = serde_json::to_string_pretty(&self)?;

        serde_json::to_writer_pretty(f, &self)?;

        Ok(())
    }
}
