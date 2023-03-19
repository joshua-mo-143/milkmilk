use std::io;
use std::io::Write;

pub struct Setup;

impl Setup {
    pub fn get_name() -> InitArgs {
        print!("What's the name of your project? > ");
        io::stdout().flush().ok();

        let mut project_name = String::new();

        io::stdin().read_line(&mut project_name).unwrap();

        let workdir = format!("./{}", project_name.trim());

        InitArgs {
            project_name,
            workdir,
        }
    }
}

pub struct InitArgs {
    pub project_name: String,
    pub workdir: String,
}
