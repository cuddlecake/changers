use crate::change::repo;
use crate::change::repo::Repo;
use crate::cli::version::Version;

#[derive(Debug)]
pub struct Args {
    audience: String,
    version: Version,
}

impl Args {
    pub fn new(version: Version, audience: String) -> Args {
        Args { version, audience }
    }
}

#[derive(Debug)]
pub enum Error {
    RepoError(repo::Error),
}

impl From<repo::Error> for Error {
    fn from(err: repo::Error) -> Self {
        Error::RepoError(err)
    }
}

pub fn handle(args: Args, repo: Repo) -> Result<(), Error> {
    let path = repo
        .find_repo_root()?
        .join("changelog")
        .join(args.version.to_string());

    println!("{:?}", path);
    println!("{:?}", args);
    Ok(())
}
