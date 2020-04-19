pub mod log_type;
pub mod version;

use log_type::LogType;
use structopt::StructOpt;
use version::Version;

pub fn start() -> CLI {
    return CLI::from_args();
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "changers",
    about = "Utility for creating and rendering Changelog Artifacts"
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

        /// Audience for which to render exclusively
        #[structopt(default_value = "user", long = "for", short = "f")]
        audience: String,
    },
    #[structopt(name = "render")]
    Render {
        /// The release to render
        release_name: Version,

        /// Audience for which to render exclusively
        #[structopt(default_value = "user", long = "for", short = "f")]
        audience: String,
    },
    #[structopt(name = "release")]
    Release {
        /// Name of the Release and the resulting directory
        release_name: Version,
    },
}
