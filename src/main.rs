mod backend;
mod commands;
mod docker;
mod frontend;
mod setup;
mod utils;
use commands::parse_commands;

fn main() {
    match parse_commands() {
        Ok(_) => println!(r#"Milkmilk was successful!"#),
        Err(_) => {
            println!("Looks like an error happened :( feel free to report the error on GitHub!")
        }
    }
}
