use git2::Repository;
use std::path::Path;

pub struct Repo {
    git_repository: Repository,
}

pub fn open(path: &Path) -> Repo {
    Repository::open(path)
        .map(|repo| Repo {
            git_repository: repo,
        })
        .expect("CLI expects to be called from within a git repository, but none was found")
}

impl Repo {
    pub fn find_repo_root(&self) -> &Path {
        self.git_repository
            .workdir()
            .expect("CLI expects a git workdir to exist, but none was found")
    }

    pub fn current_branch_name(&self) -> String {
        self.git_repository
            .resolve_reference_from_short_name("HEAD")
            .map(|reference| String::from(reference.shorthand().unwrap_or_default()))
            .expect("CLI expects 'HEAD' (an active branch) to exist, but none was found")
    }

    pub fn author_name(&self) -> String {
        self.git_repository
            .signature()
            .map_or(String::from(""), |sig| {
                String::from(sig.name().unwrap_or_default())
            })
    }
}
