use argh::FromArgs;
use once_cell::sync::Lazy;

use crate::settings::Settings;

#[derive(FromArgs, Debug)]
#[argh(description = "aicam")]
pub struct CliArgs {
    /// show version
    #[argh(switch, short = 'v')]
    pub version: bool,
}
pub static SETTINGS: Lazy<Settings> = Lazy::new(|| match Settings::new() {
    Ok(value) => value,
    Err(err) => panic!("Failed cos wrong config: {}", err),
});

pub mod settings;
