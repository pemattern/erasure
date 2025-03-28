use std::{
    fs,
    num::{NonZero, NonZeroU8},
};

use erasure_core::ErasureStandard;
use serde::Deserialize;

use crate::cli_args::CliArgs;

#[derive(Debug, Default, Deserialize)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
pub struct Config {
    pub erasure: ErasureConfig,
    pub user_data: Vec<UserDataField>,
}

impl Config {
    pub fn load(args: &CliArgs) -> Self {
        let path = &args.config;
        let toml = fs::read_to_string(path).unwrap();
        let config = toml::from_str::<Self>(&toml).unwrap_or_else(|err| {
            panic!("{}", err.message());
        });
        config
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct ErasureConfig {
    pub standard: ErasureStandard,
    pub passes: NonZeroU8,
}

impl Default for ErasureConfig {
    fn default() -> Self {
        Self {
            standard: ErasureStandard::default(),
            passes: NonZero::new(1).unwrap(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UserDataField {
    name: String,
    #[serde(default)]
    regex: Option<String>,
}
