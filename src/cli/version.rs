use std::num::ParseIntError;

#[derive(Debug, Eq, PartialEq)]
pub struct Version {
    major: u32,
    minor: u32,
    bug: u32,
}

#[derive(Debug)]
pub struct VersionParseError {
    value: String,
}

impl std::fmt::Display for VersionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:?} should have form 'major.minor.bug', where major, minor and bug are of type u32 and bug > 0",
            self.value
        )
    }
}

// This is important for other errors to wrap this one.
impl std::error::Error for VersionParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

impl std::str::FromStr for Version {
    type Err = VersionParseError;
    fn from_str(str: &str) -> Result<Version, VersionParseError> {
        let split: Vec<&str> = str.split(".").collect();
        let versions: Vec<Result<u32, ParseIntError>> =
            split.into_iter().map(|str| str.parse::<u32>()).collect();
        match *versions {
            [Ok(major), Ok(minor), Ok(bug)] if bug > 0 => Ok(Version { major, minor, bug }),
            _ => Err(VersionParseError {
                value: str.to_string(),
            }),
        }
    }
}

impl ToString for Version {
    fn to_string(&self) -> String {
        let v = vec![self.major, self.minor, self.bug]
            .into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>();

        v.join(".")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_from_str() {
        assert_eq!(
            Version {
                major: 1,
                minor: 2,
                bug: 3
            },
            Version::from_str("1.2.3").unwrap()
        )
    }

    #[test]
    fn test_from_str_failure() {
        assert!(
            Version::from_str("0.0.0").is_err(),
            "should have been an error, but is not"
        );
        assert!(
            Version::from_str("0.-1.1").is_err(),
            "should have been an error, but is not"
        );
        assert!(
            Version::from_str("hello").is_err(),
            "should have been an error, but is not"
        );
        assert!(
            Version::from_str("1.2.x").is_err(),
            "should have been an error, but is not"
        );
    }

    #[test]
    fn test_to_string() {
        let version = Version {
            major: 3,
            minor: 6,
            bug: 8,
        }
        .to_string();
        assert_eq!(version, "3.6.8")
    }
}
