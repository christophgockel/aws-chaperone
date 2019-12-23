use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ConfigError {
    NoProfileFound,
    NoMfaDeviceArnDefined,
    NoAwsProfileDefined,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match *self {
            ConfigError::NoProfileFound => f.write_str("Wat"),
            ConfigError::NoMfaDeviceArnDefined => f.write_str("..."),
            ConfigError::NoAwsProfileDefined => f.write_str("..."),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
enum IniValue {
    Value(String),
    Map(HashMap<String, IniValue>),
}

type IniContent = HashMap<String, IniValue>;

#[derive(Debug, PartialEq)]
pub struct Config {
    mfa_device_arn: String,
    aws_cli_profile_name: String,
}

impl Config {
    pub fn for_profile(profile: String, content: String) -> Result<Config, ConfigError> {
        let content = content.trim();
        let entries = serde_ini::from_str::<IniContent>(&content).unwrap();

        let profile_section = match entries.get(&profile) {
            Some(IniValue::Map(section)) => section,
            _ => return Err(ConfigError::NoProfileFound),
        };

        let mfa_device_arn = match profile_section.get("mfa-device-arn") {
            Some(IniValue::Value(x)) => x.to_string(),
            _ => return Err(ConfigError::NoMfaDeviceArnDefined),
        };

        let aws_cli_profile_name = match profile_section.get("aws-cli-profile") {
            Some(IniValue::Value(x)) => x,
            _ => return Err(ConfigError::NoAwsProfileDefined),
        };

        return Ok(Config {
            mfa_device_arn: mfa_device_arn.to_string(),
            aws_cli_profile_name: aws_cli_profile_name.to_string(),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_valid_config_file_into_struct() {
        let profile = "the-profile".to_string();
        let content = r"
            [the-profile]
            mfa-device-arn = the-arn
            aws-cli-profile = aws-profile
        "
        .to_string();

        let config = Config::for_profile(profile, content).unwrap();

        assert_eq!("the-arn", config.mfa_device_arn);
        assert_eq!("aws-profile", config.aws_cli_profile_name);
    }

    #[test]
    fn errors_when_no_profile_could_be_found() {
        let profile = "the-profile".to_string();
        let content = r"".to_string();

        let result = Config::for_profile(profile, content);

        assert_eq!(result, Err(ConfigError::NoProfileFound));
    }

    #[test]
    fn errors_when_mfa_device_arn_is_missing_from_profile() {
        let profile = "the-profile".to_string();
        let content = r"
            [the-profile]
            aws-cli-profile = aws-profile
        "
        .to_string();

        let result = Config::for_profile(profile, content);

        assert_eq!(result, Err(ConfigError::NoMfaDeviceArnDefined));
        assert_eq!(result.err(), Some(ConfigError::NoMfaDeviceArnDefined));
    }

    #[test]
    fn errors_when_aws_cli_profile_is_missing_from_profile() {
        let profile = "the-profile".to_string();
        let content = r"
            [the-profile]
            mfa-device-arn = the-arn
        "
        .to_string();

        let result = Config::for_profile(profile, content);

        assert_eq!(result, Err(ConfigError::NoAwsProfileDefined));
    }
}
