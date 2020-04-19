use std::fs::DirBuilder;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::change::log::Log;
use crate::change::repo;
use crate::change::repo::Repo;
use std::time::SystemTime;

#[derive(Debug)]
pub enum Error {
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

pub fn handle(args: Args, repo: Repo) -> Result<(), Error> {
    let mut log: Log = Log::new(args.log_type, args.summary);
    let unreleased_path = repo.find_repo_root().unwrap().join("changelogs/unreleased");
    create_unrelease_directory(unreleased_path.as_path())?;
    let current_branch_name = repo.current_branch_name()?;

    log.set_author(repo.author_name());
    log.set_branch_name(current_branch_name.to_string());

    let log_item_yaml = log.to_yaml_str().map_err(|_| Error::CouldNotSerializeLog)?;
    let file_name = format!(
        "{}_{}",
        current_timestamp(),
        current_branch_name.to_string()
    );

    let path = &unreleased_path.join(file_name).with_extension("yml");
    write_log(&path, log_item_yaml)
}

fn write_log(path: &Path, log_string: String) -> Result<(), Error> {
    let mut file = File::create(path).map_err(|_| Error::CouldNotCreateFile)?;
    file.write_all(log_string.as_bytes())
        .map_err(|_| Error::CouldNotWrite)
}

fn create_unrelease_directory(unreleased_path: &Path) -> Result<(), Error> {
    DirBuilder::new()
        .recursive(true)
        .create(&unreleased_path)
        .or(Err(Error::CouldNotCreateDir))
}

fn current_timestamp() -> String {
    format!(
        "{}",
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    )
}
