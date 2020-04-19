use git2::Repository;
use std::path::Path;

pub struct Repo {
    git_repository: Repository,
}

#[derive(Debug)]
pub enum Error {
    CouldNotOpenGitRepo,
    CouldNotFindGitRoot,
    CouldNotGetHEADName,
}

pub fn open(path: &Path) -> Result<Repo, Error> {
    match Repository::open(path) {
        Ok(git_repository) => Ok(Repo { git_repository }),
        Err(_err) => Err(Error::CouldNotOpenGitRepo),
    }
}

impl Repo {
    pub fn find_repo_root(&self) -> Result<&Path, Error> {
        self.git_repository
            .workdir()
            .ok_or(Error::CouldNotFindGitRoot)
    }

    pub fn current_branch_name(&self) -> Result<String, Error> {
        self.git_repository
            .resolve_reference_from_short_name("HEAD")
            .map(|reference| String::from(reference.shorthand().unwrap_or_default()))
            .map_err(|_| Error::CouldNotGetHEADName)
    }

    pub fn author_name(&self) -> String {
        self.git_repository
            .signature()
            .map_or(String::from(""), |sig| {
                String::from(sig.name().unwrap_or_default())
            })
    }
}
