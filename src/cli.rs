pub mod log_type;

use log_type::LogType;
use structopt::StructOpt;

pub fn start() -> CLI {
    return CLI::from_args();
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "changers",
    about = "Utility for creating and aggregating Changelog Artifacts"
)]

pub struct CLI {
    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(name = "create")]
    Create {
        /// The type of the logged change
        log_type: LogType,
        /// The summary of the logged change
        summary: String,
    },
    #[structopt(name = "aggregate")]
    Aggregate {
        /// The release to aggregate
        release_name: String,
    },
    #[structopt(name = "release")]
    Release {
        /// Name of the Release and the resulting directory
        release_name: String,
    },
}
