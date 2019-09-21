use serde::{Deserialize, Serialize};
use serde_yaml;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Log {
    summary: String,
    log_type: String,
    author: Option<String>,
    merge_request: Option<String>,
}

impl Log {
    pub fn new(log_type: String, summary: String) -> Log {
        Log {
            log_type,
            summary,
            author: None,
            merge_request: None,
        }
    }

    pub fn from_yaml_str(s: &str) -> Result<Log, serde_yaml::Error> {
        return serde_yaml::from_str(s);
    }

    pub fn set_mr(&mut self, mr: String) -> &mut Log {
        self.merge_request = Some(mr);
        self
    }

    pub fn set_author(&mut self, author: String) -> &mut Log {
        self.author = Some(author);
        self
    }

    pub fn to_yaml_str(&self) -> Result<String, serde_yaml::Error> {
        return serde_yaml::to_string(self);
    }
}
