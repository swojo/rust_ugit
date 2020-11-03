use clap::{App, SubCommand};
use std::fs;

fn main() {
    let matches = App::new("ugit")
        .version("1.0")
        .about("ugit in Rust")
        .subcommand(SubCommand::with_name("init"))
        .get_matches();

    match matches.subcommand() {
        ("init", _) => {
            match init() {
                Ok(_) => println!("initialized empty ugit repository"),
                Err(_) => println!("Error initializing ugit repo")
            }
        },
        _ => ()
    }
}

fn init() -> std::io::Result<()> {
    fs::create_dir(".ugit")?;
    Ok(())
}
