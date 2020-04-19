use crate::change::{create, release, repo};
use cli::Command;
use std::path::Path;

mod change;
mod cli;

fn main() {
    let opt = cli::start();
    let base_path = Path::new(".");
    match repo::open(base_path) {
        Ok(repo) => match opt.cmd {
            Command::Create {
                log_type,
                summary,
                audience,
            } => {
                create::handle(
                    create::Args::new(log_type.to_string(), summary, audience),
                    repo,
                )
                .unwrap();
            }
            Command::Release { release_name } => {
                release::handle(release::Args::new(release_name), repo).unwrap();
            }
            Command::Aggregate { release_name: _ } => {
                println!("release!");
            }
        },
        Err(err) => println!("Failed Collecting Git Repository Information: {:?}", err),
    };
}
