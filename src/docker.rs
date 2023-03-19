use std::fs;
use std::io::Write;
use std::path::PathBuf;

pub struct Dockerfile;

pub enum Filetype {
    Dockerfile,
    Dockerignore,
    Gitignore,
    Env,
}

impl Filetype {
    pub fn generate_filepath(&self, mut workdir: Option<&str>) -> PathBuf {
        if workdir.is_none() {
            workdir = Some(r#"."#);
        }

        let mut workdir = workdir.unwrap().to_string();

        match self {
            Filetype::Dockerfile => workdir.push_str("/Dockerfile"),
            Filetype::Dockerignore => workdir.push_str("/.dockerignore"),
            Filetype::Gitignore => workdir.push_str("/.gitignore"),
            Filetype::Env => workdir.push_str("/.env"),
        }

        PathBuf::from(workdir)
    }
}

impl Dockerfile {
    pub fn generate_ci_files(workdir: Option<&str>) {
        Dockerfile::generate_dockerfile(workdir);
        Dockerfile::generate_dockerignore(workdir);
        Dockerfile::generate_gitignore(workdir);
        Dockerfile::generate_env(workdir);
    }

    pub fn generate_dockerfile(workdir: Option<&str>) {
        let workdir = Filetype::Dockerfile.generate_filepath(workdir);

        let mut f = fs::File::create(&workdir).expect("Had an error trying to create a file :(");

        let str = r#"FROM rust:latest

WORKDIR /joshuamo/<your-app-here>
COPY . .
RUN cargo install --path .
EXPOSE 8000
CMD [<your-app-here>]"#;

        f.write_all(str.as_bytes())
            .expect("Had a problem writing to file :(");

        println!("Dockerfile written");
    }

    pub fn generate_dockerignore(workdir: Option<&str>) {
        let workdir = Filetype::Dockerignore.generate_filepath(workdir);

        fs::File::create(workdir).expect("Had an error trying to create a file :(");

        println!(".dockerignore file written");
    }

    pub fn generate_gitignore(workdir: Option<&str>) {
        let workdir = Filetype::Gitignore.generate_filepath(workdir);

        let mut f = fs::File::create(workdir).expect("Had an error trying to create a file :(");

        let str = r#".env
/target
"#;

        f.write_all(str.as_bytes())
            .expect("Had a problem writing to file :(");

        println!(".gitignore file written");
    }

    pub fn generate_env(workdir: Option<&str>) {
        let workdir = Filetype::Env.generate_filepath(workdir);

        let mut f = fs::File::create(workdir).expect("Had an error trying to create a file :(");

        let str = r#"DATABASE_URL="postgres://postgres:postgres@localhost:5432/postgres""#;

        f.write_all(str.as_bytes())
            .expect("Had a problem writing to file :(");

        println!(".env file written");
    }
}
