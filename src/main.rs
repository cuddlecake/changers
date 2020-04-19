use crate::change::{create, release, render, repo};
use cli::Command;
use std::path::Path;

mod change;
mod cli;

fn main() {
    let opt = cli::start();
    let base_path = Path::new(".");
    let repo = repo::open(base_path);

    match opt.cmd {
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
        Command::Render {
            release_name,
            audience,
        } => {
            render::handle(render::Args::new(release_name, audience), repo).unwrap();
        }
    }
}
