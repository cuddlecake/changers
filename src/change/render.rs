use crate::change::log::Log;
use crate::change::render::Error::IOError;
use crate::change::repo::Repo;
use crate::cli::version::SemanticVersion;
use std::collections::HashMap;
use std::{fs, io};

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
pub enum Error {
    ReleaseNotAccessible,
    IOError,
}

impl From<io::Error> for Error {
    fn from(_: io::Error) -> Self {
        IOError
    }
}

pub fn handle(args: Args, repo: Repo) -> Result<(), Error> {
    let path = repo.changelogs_dir().join(args.version.to_string());
    if !path.is_dir() {
        Err(Error::ReleaseNotAccessible)
    } else {
        let list = fs::read_dir(&path)?
            .filter_map(|res| {
                res.map(|res| res.path())
                    .map(fs::read_to_string)
                    .map(|str| Log::from_yaml_str(str.unwrap().parse().unwrap()).unwrap())
                    .ok()
            })
            .collect::<Vec<Log>>()
            .into_iter();

        let mut map: HashMap<String, Vec<Log>> = HashMap::new();
        list.for_each(|entry| {
            let key = entry.log_type();
            map.entry(key).or_default().push(entry)
        });

        println!("## {}", args.version.to_string());

        for (key, display) in vec![
            ("feature", "Features"),
            ("bugfix", "Bugfix"),
            ("misc", "Misc"),
            ("technical", "Technical"),
        ] {
            match map.get(key) {
                Some(list) => {
                    println!("### {}", display);
                    for log in list {
                        println!("- {}", log.summary());
                    }
                }
                None => (),
            }
        }

        Ok(())
    }
}
