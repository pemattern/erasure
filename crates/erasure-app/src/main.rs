use clap::Parser;
use cli_args::CliArgs;
use config::Config;

mod cli_args;
mod config;

fn main() {
    set_panic_hook();
    let args = CliArgs::parse();
    let config = Config::load(&args);
    println!("{:?}", config);
}

fn set_panic_hook() {
    std::panic::set_hook(Box::new(|panic_info| {
        if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            eprintln!("{}", s);
        }
    }));
}
