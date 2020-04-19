use crate::change::repo::Repo;
use serde_yaml::to_string;
use std::path::Path;

pub mod create;
pub mod log;
pub mod release;
pub mod repo;

pub struct Context<'a> {
    repo: Repo,
    changelogs_directory: &'a Path,
    author: String,
    branch_name: String,
}

impl Context<'_> {
    pub fn new(base_path: &Path) -> Result<Context, repo::Error> {
        let repo = repo::open(base_path)?;
        let repo_root = repo.find_repo_root()?;
        let author = repo.author_name();
        let branch_name = repo.current_branch_name()?;
        Ok(Context {
            repo,
            changelogs_directory: changelogs_directory(repo_root),
            author,
            branch_name,
        })
    }
}

fn changelogs_directory(base_directory: &Path) -> &Path {
    base_directory.join("changelogs").as_path()
}
