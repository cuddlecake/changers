use serde::{Deserialize, Serialize};
use serde_yaml;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Log {
    summary: String,
    log_type: String,
    branch_name: Option<String>,
    author: Option<String>,
    audience: String,
}

impl Log {
    pub fn new(log_type: String, summary: String) -> Log {
        Log {
            log_type,
            summary,
            author: None,
            branch_name: None,
            audience: "".to_string(),
        }
    }

    /*
    pub fn from_yaml_str(s: &str) -> Result<Log, serde_yaml::Error> {
        return serde_yaml::from_str(s);
    }
    */

    pub fn set_author(&mut self, author: String) -> &mut Log {
        self.author = Some(author);
        self
    }

    pub fn set_branch_name(&mut self, branch_name: String) -> &mut Log {
        self.branch_name = Some(branch_name);
        self
    }

    pub fn set_audience(&mut self, audience: String) -> &mut Log {
        self.audience = audience;
        self
    }

    pub fn to_yaml_str(&self) -> Result<String, serde_yaml::Error> {
        return serde_yaml::to_string(self);
    }
}
