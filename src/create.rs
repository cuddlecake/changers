use git2::Repository;

pub fn handle(args: Args) {
    let name = branch_name();
    println!("{:?}", name);
    if let Ok(name) = branch_name() {
        println!("name is {:?}", name)
    }
}

#[derive(Debug)]
pub struct Args {
    log_type: String,
    summary: String,
}

impl Args {
    pub fn new(log_type: String, summary: String) -> Args {
        return Args {
            log_type,
            summary: summary,
        };
    }
}

fn branch_name() -> Result<String, git2::Error> {
    let repo = Repository::open(".")?;
    let reference = repo.resolve_reference_from_short_name("HEAD")?;
    if let Some(name) = reference.shorthand() {
        return Ok(name.to_string());
    } else {
        return Err(git2::Error::from_str(
            "HEAD could not be resolved to a name",
        ));
    }
}
