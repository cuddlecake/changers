use crate::change::repo;
use crate::change::repo::Repo;
use crate::cli::version;
use std::fs::rename;

#[derive(Debug)]
pub struct Args {
    version: version::Version,
}

#[derive(Debug)]
pub enum Error {
    RepoError(repo::Error),
    CouldNotRename,
}

impl From<repo::Error> for Error {
    fn from(err: repo::Error) -> Error {
        Error::RepoError(err)
    }
}

impl Args {
    pub fn new(version: version::Version) -> Args {
        Args { version }
    }
}

pub fn handle(args: Args, repo: Repo) -> Result<(), Error> {
    let unreleased_path = &repo.find_repo_root()?.join("changelogs/unreleased");
    let target_path = &repo
        .find_repo_root()
        .unwrap()
        .join("changelogs")
        .join(args.version.to_string());

    rename(unreleased_path, target_path).map_err(|_| Error::CouldNotRename)
}
