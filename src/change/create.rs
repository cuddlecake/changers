use std::fs::DirBuilder;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::change::log::Log;
use crate::change::{repo, Context};

#[derive(Debug)]
enum Error {
    CouldNotWrite,
    CouldNotCreateFile,
    CouldNotSerializeLog,
    CouldNotCreateDir,
    RepoError(repo::Error),
}

impl From<repo::Error> for Error {
    fn from(err: repo::Error) -> Error {
        Error::RepoError(err)
    }
}

#[derive(Debug)]
pub struct Args {
    log_type: String,
    summary: String,
}

impl Args {
    pub fn new(log_type: String, summary: String) -> Args {
        Args { log_type, summary }
    }
}

pub fn handle(args: Args, ctx: Context) {
    let log: Log = Log::new(args.log_type, args.summary);
    match do_it(log, ctx) {
        Ok(_) => println!("Done"),
        Err(err) => println!("{:?}", err),
    };
}

fn do_it(mut log: Log, ctx: Context) -> Result<(), Error> {
    let unreleased_path = ctx.changelogs_directory.join("unreleased").as_path();

    DirBuilder::new()
        .recursive(true)
        .create(&unreleased_path)
        .map_err(|_| Error::CouldNotCreateDir)?;

    log.set_author(ctx.author);
    log.set_branch_name(ctx.branch_name);

    let log_item_yaml = log.to_yaml_str().map_err(|_| Error::CouldNotSerializeLog)?;

    let path = &unreleased_path.join("funk").with_extension("yml");
    write_log(&path, log_item_yaml)
}

fn write_log(path: &Path, log_string: String) -> Result<(), Error> {
    let mut file = File::create(path).map_err(|_| Error::CouldNotCreateFile)?;
    file.write_all(log_string.as_bytes())
        .map_err(|_| Error::CouldNotWrite)
}
