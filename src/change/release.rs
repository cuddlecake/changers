use crate::change::repo::Repo;
use crate::cli::version;
use std::fs::rename;

#[derive(Debug)]
pub struct Args {
    version: version::SemanticVersion,
}

#[derive(Debug)]
pub enum Error {
    CouldNotRename,
}

impl Args {
    pub fn new(version: version::SemanticVersion) -> Args {
        Args { version }
    }
}

pub fn handle(args: Args, repo: Repo) -> Result<(), Error> {
    let unreleased_path = &repo.unreleased_dir();
    let target_path = repo.changelogs_dir().join(args.version.to_string());

    rename(unreleased_path, target_path).map_err(|_| Error::CouldNotRename)
}
