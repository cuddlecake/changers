#[derive(Debug)]
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
        return match self {
            LogType::Technical => "Technical",
            LogType::Feature => "Feature",
            LogType::Misc => "Misc",
            LogType::Bugfix => "Bugfix",
        }
        .to_string();
    }
}
