use clap::{load_yaml, value_t, value_t_or_exit, values_t, App};

use log::*;

use std::process;
use std::process::{exit, ExitStatus};
use std::str;

use failure;

fn main() -> Result<(), failure::Error> {
    env_logger::init();
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml);
    let matches = app.get_matches();

    let channel = value_t!(matches, "channel", String).unwrap_or("nixos".to_owned());
    let nix_channel = format!("<{}>", channel);
    let package = value_t_or_exit!(matches, "package", String);
    let attribute = value_t!(matches, "attr", String).unwrap_or(String::from(""));
    let package_attribute = match Some(attribute) {
        Some(s) => format!("{}.{}", package, s),
        _ => package,
    };

    // TODO: investigate rnix implementation -- https://gitlab.com/jD91mZM2/rnix
    let output = process::Command::new("nix-instantiate")
        .arg("--eval")
        .arg(nix_channel)
        .arg("-A")
        .arg(&package_attribute)
        .output()
        .expect("Failed to evaluate nix property");

    match output.status.success() {
        true => println!(
            "{}: {}",
            package_attribute,
            str::from_utf8(&output.stdout).expect("Failed to parse result as string")
        ),
        false => error!(
            "Error: {}",
            str::from_utf8(&output.stderr).expect("Failed to parse error message as string")
        ),
    }

    Ok(())
}
