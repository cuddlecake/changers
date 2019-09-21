use git2::Repository;
use std::fs;
use std::fs::DirBuilder;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::change::log::Log;

pub fn handle(args: Args) {
    let project_root_path = Path::new(&args.project_root);
    let repo = open_repo(&project_root_path);
    let base_path = args.project_root + "/changelogs/unreleased";
    DirBuilder::new()
        .recursive(true)
        .create(&base_path)
        .unwrap();

    let lelog: Log = Log::new(args.log_type, args.summary);
    let log_item_yaml = match lelog.to_yaml_str() {
        Ok(item) => item,
        Err(err) => panic!("should never happen"),
    };

    let branch_name = branch_name(repo);
    let path_value = String::from(base_path) + "/" + &branch_name.to_string() + ".yml";
    let path = Path::new(&path_value);
    let file_result = File::create(path);
    if let Ok(mut file) = file_result {
        match file.write_all(log_item_yaml.as_bytes()) {
            Ok(_) => println!("successfully wrote"),
            Err(_err) => panic!("erjae"),
        }
    }
}

#[derive(Debug)]
pub struct Args {
    log_type: String,
    summary: String,
    project_root: String,
}

impl Args {
    pub fn new(log_type: String, summary: String, project_root: String) -> Args {
        return Args {
            log_type,
            summary,
            project_root,
        };
    }
}

fn open_repo(path: &Path) -> Repository {
    match Repository::open(path) {
        Ok(repo) => repo,
        Err(error) => {
            println!("Project Path={} is not a repository", path.display());
            std::process::exit(1);
        }
    }
}

fn branch_name(repo: Repository) -> String {
    let reference = repo
        .resolve_reference_from_short_name("HEAD")
        .map_err(|_| {
            eprintln!(
                "HEAD could not be resolved to a branch name, your git directory may be broken",
            );
            std::process::exit(1);
        })
        .expect("really weird error");
    let name = reference.shorthand().expect("ok");
    name.to_string()
}
