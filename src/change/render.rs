use crate::change::repo::Repo;
use crate::cli::version::SemanticVersion;

#[derive(Debug)]
pub struct Args {
    audience: String,
    version: SemanticVersion,
}

impl Args {
    pub fn new(version: SemanticVersion, audience: String) -> Args {
        Args { version, audience }
    }
}

#[derive(Debug)]
pub enum Error {}

pub fn handle(args: Args, repo: Repo) -> Result<(), Error> {
    let path = repo.changelogs_dir().join(args.version.to_string());

    println!("{:?}", path);
    println!("{:?}", args);
    Ok(())
}
