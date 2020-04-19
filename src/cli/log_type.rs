#[derive(Debug, PartialEq)]
pub enum LogType {
    Bugfix,
    Feature,
    Technical,
    Misc,
}

#[derive(Debug)]
pub struct LogTypeParseError {
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

impl ToString for LogType {
    fn to_string(&self) -> String {
        return format!("{:?}", self).to_lowercase();
    }
}

#[cfg(test)]
mod tests {
    use crate::cli::log_type::LogType;
    use std::str::FromStr;

    #[test]
    fn test_to_string() {
        assert_eq!(LogType::Technical.to_string(), "technical");
        assert_eq!(LogType::Feature.to_string(), "feature");
        assert_eq!(LogType::Misc.to_string(), "misc");
        assert_eq!(LogType::Bugfix.to_string(), "bugfix");
    }

    #[test]
    fn test_from_string() {
        assert_eq!(LogType::from_str("technical").unwrap(), LogType::Technical);
        assert_eq!(LogType::from_str("feature").unwrap(), LogType::Feature);
        assert_eq!(LogType::from_str("bugfix").unwrap(), LogType::Bugfix);
        assert_eq!(LogType::from_str("misc").unwrap(), LogType::Misc);
    }

    #[test]
    fn test_from_string_failure() {
        assert!(LogType::from_str("thisisillegal").is_err())
    }
}
