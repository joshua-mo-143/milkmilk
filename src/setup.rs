use std::io;
use std::io::Write;

pub struct Setup;

impl Setup {
    pub fn get_name(shuttle: bool) -> InitArgs {
        print!("What's the name of your project? > ");
        io::stdout().flush().ok();

        let mut project_name = String::new();

        io::stdin().read_line(&mut project_name).unwrap();

        let workdir = format!("./{}", project_name.trim());

        InitArgs {
            project_name,
            workdir,
            deploy_on: if shuttle {
                DeployOn::Shuttle
            } else {
                DeployOn::DockerImage
            },
        }
    }
}

#[derive(Clone)]
pub struct InitArgs {
    pub project_name: String,
    pub workdir: String,
    pub deploy_on: DeployOn,
}

pub enum DeployOn {
    DockerImage,
    Shuttle,
}
