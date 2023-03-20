use clap::{Parser, Subcommand};

use crate::backend::axum::Axum;
use crate::docker::Dockerfile;
use crate::frontend::nextjs::Nextjs;
use crate::setup::Setup;
use crate::utils::{Utils, PackageJson};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Cmd,
}

#[derive(Subcommand)]
pub enum Cmd {
    /// Initiate a full-stack Next.js + Axum fullstack project complete with Dockerfile ready to deploy.
    Start,
    /// Make a dockerfile that you can use to deploy your Rust web services on.
    Dockerfile,
    /// Initiate a premade Axum backend project with basic CRUD routes.
    Backend,
    /// Get packagejson stuff
    Packagejson,
}

pub fn parse_commands() -> Result<(), String> {
    let cli = Cli::parse();

    match cli.command {
        Cmd::Dockerfile => {
            Dockerfile::generate_dockerfile(None);
        }
        Cmd::Start => {
            let mut init_args = Setup::get_name();

            Nextjs::bootstrap(init_args.clone());

            SetupCmd::CargoInit.run(Some("backend"), Some(&init_args.workdir));

            init_args.workdir.push_str("/backend");

            SetupCmd::CargoAdd.run(None, Some(&init_args.workdir));
            Dockerfile::generate_ci_files(Some(&init_args.workdir));
            Axum::bootstrap(init_args.workdir).expect("Failed to write files");
        }
        Cmd::Backend => {
            let init_args = Setup::get_name();

            SetupCmd::CargoInit.run(Some(&init_args.project_name), None);
            SetupCmd::CargoAdd.run(None, Some(&init_args.workdir));
            Dockerfile::generate_ci_files(Some(&init_args.workdir));
            Axum::bootstrap(init_args.workdir).expect("Failed to write files");
        }
        Cmd::Packagejson => {
            let filepath = String::from("./package.json");

            PackageJson::from_json(&filepath)
                .add_data()
                .write_to_file(&filepath)
                .expect("Failed to write to package.json");
        }
    }

    Ok(())
}

pub enum SetupCmd {
    CreateNextApp,
    FrontendDeps,
    TailwindDeps,
    TailwindInit,
    CargoInit,
    CargoAdd,
}

impl SetupCmd {
    pub fn run(&self, project_name: Option<&str>, workdir: Option<&str>) {
        let cmd = match self {
            SetupCmd::CreateNextApp => {
                format!(
                    "npx create-next-app@latest {} --ts --tailwind",
                    project_name.unwrap()
                )
            },

            SetupCmd::FrontendDeps => "npm i zustand".into(),

            SetupCmd::TailwindDeps => "npm i -D tailwindcss@latest autoprefixer@latest postcss@latest".into(),

            SetupCmd::TailwindInit => "npx tailwindcss init -p".into(),

            SetupCmd::CargoInit => {
                format!("cargo init --bin {}", project_name.unwrap())
            }

            SetupCmd::CargoAdd => "cargo add tokio axum serde dotenvy sqlx --features serde/derive,sqlx/runtime-tokio-rustls,sqlx/postgres,tokio/macros".into(),
        };

        println!("Trying to run {cmd}");

        Utils::run_command(cmd, workdir).expect("Looks like bootstrapping has broken :(");
    }
}
