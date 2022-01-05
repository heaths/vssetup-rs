// Copyright 2020 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

#![cfg(windows)]

extern crate clap;
use clap::{crate_version, App, Arg, ArgMatches};

extern crate vssetup;
use vssetup::{Result, SetupConfiguration};

extern crate com;
use com::runtime::{ApartmentRuntime, ApartmentType};

mod formatter;

fn main() -> Result<()> {
    let opts = parse();
    let _apartment = ApartmentRuntime::new(ApartmentType::SingleThreaded)?;

    let config = SetupConfiguration::new();
    if let Some(e) = config.instances(opts.all) {
        formatter::print_instances(e)?;
    }

    Ok(())
}

struct Opts {
    all: bool,
    _include: Vec<String>,
    _locale: String,
    _path: Option<String>,
}

impl From<ArgMatches<'_>> for Opts {
    fn from(m: ArgMatches<'_>) -> Self {
        Opts {
            all: m.is_present("all"),
            _include: vec![],
            _locale: m.value_of("locale").unwrap_or("en").to_owned(),
            _path: m.value_of("path").map(|s| s.to_string()),
        }
    }
}

fn parse() -> Opts {
    let matches = App::new("Visual Studio Locator")
        .version(crate_version!())
        .arg(
            Arg::with_name("all")
                .long("all")
                .help("Finds all instances even if they are incomplete and may not launch."),
        )
        .arg(
            Arg::with_name("include")
                .long("include")
                .help("Other information to include.")
                .takes_value(true)
                .value_names(&["errors", "packages"]),
        )
        .arg(
            Arg::with_name("locale")
                .long("locale")
                .help("The locale to use for localized values.")
                .takes_value(true)
                .default_value("en"),
        )
        .arg(
            Arg::with_name("path")
                .long("path")
                .help("Gets an instance for the given path, if any defined for that path.")
                .takes_value(true),
        )
        .get_matches();

    Opts::from(matches)
}
