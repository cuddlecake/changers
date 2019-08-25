use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "changers",
    about = "Utility for creating and aggregating Changelog Artifacts"
)]
struct Changers {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
enum LogType {
    Bugfix,
    Feature,
    Technical,
    Misc,
}

#[derive(Debug, Clone)]
struct LogTypeParseError {
    value: String,
}

impl std::fmt::Display for LogTypeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Invalid Value provided: {:?}, should be one of bugfix|feature|technical|misc",
            self.value
        )
    }
}

// This is important for other errors to wrap this one.
impl std::error::Error for LogTypeParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

impl std::str::FromStr for LogType {
    type Err = LogTypeParseError;
    fn from_str(s: &str) -> Result<LogType, LogTypeParseError> {
        match s.trim().to_lowercase().as_ref() {
            "bugfix" => Ok(LogType::Bugfix),
            "feature" => Ok(LogType::Feature),
            "technical" => Ok(LogType::Technical),
            "misc" => Ok(LogType::Misc),
            _ => Err(LogTypeParseError {
                value: s.to_string(),
            }),
        }
    }
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "create")]
    Create {
        /// The type of the logged change
        log_type: LogType,
        /// The summary of the logged change
        summary: String,
    },
    #[structopt(name = "fetch")]
    Aggregate,
}

struct CreateVal {
    summary: String,
}

fn main() {
    let opt = Changers::from_args();
    println!("{:?}", opt);
}
