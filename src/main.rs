use clap::{load_yaml, value_t, value_t_or_exit, App};

use log::*;

use std::process;
use std::str;

use failure;

fn main() -> Result<(), failure::Error> {
    env_logger::init();
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml);
    let matches = app.get_matches();
    //dbg!(&matches);

    let channel = value_t!(matches, "channel", String).unwrap_or("nixos".to_owned());
    debug!("{:?}", &channel);
    let nix_channel = format!("<{}>", channel);
    debug!("{:?}", &nix_channel);
    let package = value_t_or_exit!(matches, "package", String);
    debug!("{:?}", &package);
    let attribute = value_t!(matches, "attribute", String).unwrap_or("".to_owned());
    debug!("{:?}", &attribute);
    let package_attribute = match attribute.as_str() {
        "" => package,
        s => format!("{}.{}", package, s),
    };
    debug!("{:?}", &package_attribute);

    let output = process::Command::new("nix-instantiate")
        .arg("--eval")
        .arg(nix_channel)
        .arg("-A")
        .arg(&package_attribute)
        .output()
        .expect("Failed to evaluate nix property");

    trace!("{:?}", &output);

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
