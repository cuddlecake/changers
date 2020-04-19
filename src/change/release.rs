use crate::change::Context;
use crate::cli::version;
use std::fs::rename;
use std::path::Path;

#[derive(Debug)]
pub struct Args {
    version: version::Version,
}

impl Args {
    pub fn new(version: version::Version) -> Args {
        Args { version }
    }
}

pub fn handle(args: Args, ctx: Context) {
    match rename(
        Path::new(&ctx.changelogs_directory).join("unreleased"),
        args.version.to_string(),
    ) {
        Ok(_) => {
            println!("ok");
        }
        Err(_) => {
            println!("not ok");
        }
    }
}
