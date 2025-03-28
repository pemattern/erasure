use std::{path::PathBuf, str::FromStr};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    #[arg(short, long, default_value_t = false)]
    pub debug: bool,
    #[arg(short, long, value_name = "CONFIG_PATH", default_value_os_t = get_default_config_path())]
    pub config: PathBuf,
}

fn get_default_config_path() -> PathBuf {
    PathBuf::from_str("crates/erasure-app/config.toml").unwrap()
}
