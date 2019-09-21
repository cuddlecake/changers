use cli::Command;

mod change;
mod cli;
mod create;

fn main() {
    let default_project_root = ".".to_string();
    let opt = cli::start();
    match opt.cmd {
        Command::Create { log_type, summary } => create::handle(create::Args::new(
            log_type.to_string(),
            summary,
            default_project_root,
        )),
        Command::Aggregate { release_name: _ } => println!("aggregate!"),
        Command::Release { release_name: _ } => println!("release!"),
    }
}
