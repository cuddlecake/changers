use cli::Command;

mod cli;
mod create;

fn main() {
    let opt = cli::start();
    match opt.cmd {
        Command::Create { log_type, summary } => {
            create::handle(create::Args::new(log_type.to_string(), summary))
        }
        Command::Aggregate => println!("aggregate"),
    }
}
