use clap::{Parser, Subcommand};

use crate::backend::axum::Axum;
use crate::docker::Dockerfile;
use crate::frontend::nextjs::Nextjs;
use crate::setup::Setup;
use crate::utils::{PackageJson, Utils};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Cmd,
}

#[derive(Subcommand)]
pub enum Cmd {
    /// Initiate a full-stack Next.js + Axum fullstack project complete with Dockerfile ready to deploy.
    Start {
        #[arg(short, long)]
        /// Initiate your backend to
        shuttle: bool,
    },
    /// Make a dockerfile that you can use to deploy your Rust web services on.
    Dockerfile,
    /// Initiate a premade Axum backend project with basic CRUD routes.
    Backend {
        #[arg(short, long)]
        shuttle: bool,
    },
    /// Add scripts to your package.json to make it auto-build into your Rust backend
    Packagejson,
}

pub fn parse_commands() -> Result<(), String> {
    let cli = Cli::parse();

    match cli.command {
        Cmd::Dockerfile => {
            Dockerfile::generate_dockerfile(None);
        }
        Cmd::Start { shuttle } => {
            let mut init_args = Setup::get_name(shuttle);

            Nextjs::bootstrap(init_args.clone());

            SetupCmd::CargoInit.run(Some("backend"), Some(&init_args.workdir));

            init_args.workdir.push_str("/backend");

            if shuttle {
                SetupCmd::CargoAdd.run(None, Some(&init_args.workdir));
            } else {
                SetupCmd::CargoAddShuttle.run(None, Some(&init_args.workdir));
            }
            Dockerfile::generate_ci_files(Some(&init_args.workdir));
            Axum::bootstrap(init_args.workdir, init_args.deploy_on).expect("Failed to write files");
        }
        Cmd::Backend { shuttle } => {
            let init_args = Setup::get_name(shuttle);

            SetupCmd::CargoInit.run(Some(&init_args.project_name), None);
            SetupCmd::CargoAdd.run(None, Some(&init_args.workdir));
            Dockerfile::generate_ci_files(Some(&init_args.workdir));
            Axum::bootstrap(init_args.workdir, init_args.deploy_on).expect("Failed to write files");
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
    CargoAddShuttle,
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
            SetupCmd::CargoAddShuttle => "cargo add shuttle_runtime shuttle_axum shuttle_secrets tokio axum serde sqlx --features 
            serde/derive,sqlx/runtime-tokio-native-tls,sqlx/postgres".into()
        };

        println!("Trying to run {cmd}");

        Utils::run_command(cmd, workdir).expect("Looks like bootstrapping has broken :(");
    }
}
