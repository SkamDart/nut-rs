//! # rupsc
//! A demo program to display UPS variables.
//! This a Rust clone of [upsc](https://github.com/networkupstools/nut/blob/master/clients/upsc.c).
//!
//! P.S.: pronounced "r-oopsie".
use core::convert::TryInto;

use anyhow::Context;
use clap::{Arg, Command};

use rups::UpsdName;

mod cmd;

fn main() -> anyhow::Result<()> {
    let args = Command::new(env!("CARGO_CRATE_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("list")
                .short('l')
                .conflicts_with_all(&["list-full", "clients"])
                .action(clap::ArgAction::SetTrue)
                .help("Lists each UPS on <hostname>, one per line."),
        )
        .arg(
            Arg::new("list-full")
                .short('L')
                .conflicts_with_all(&["list", "clients"])
                .action(clap::ArgAction::SetTrue)
                .help("Lists each UPS followed by its description (from ups.conf)."),
        )
        .arg(
            Arg::new("clients")
                .short('c')
                .conflicts_with_all(&["list", "list-full"])
                .action(clap::ArgAction::SetTrue)
                .help("Lists each client connected on <upsname>, one per line."),
        )
        .arg(
            Arg::new("debug")
                .short('D')
                .long("debug")
                .action(clap::ArgAction::SetTrue)
                .help("Enables debug mode (logs network commands to stderr)."),
        )
        .arg(
            Arg::new("ssl")
                .short('S')
                .long("ssl")
                .action(clap::ArgAction::SetTrue)
                .help("Enables SSL on the connection with upsd."),
        )
        .arg(
            Arg::new("insecure-ssl")
                .long("insecure-ssl")
                .action(clap::ArgAction::SetTrue)
                .help("Disables SSL verification on the connection with upsd."),
        )
        .arg(
            Arg::new("upsd-server")
                .required(false)
                .value_name("[upsname][@<hostname>[:<port>]]")
                .help("upsd server (with optional upsname, if applicable)."),
        )
        .arg(
            Arg::new("variable")
                .required(false)
                .value_name("variable")
                .help("Optional, display this variable only."),
        )
        .get_matches();

    let server: UpsdName = args.get_one::<UpsdName>("upsd-server").map_or_else(
        || Ok::<UpsdName, anyhow::Error>(UpsdName::default()),
        |s| Ok(*s),
    )?;

    let debug = args.get_flag("debug");
    // let insecure_ssl = args.is_present("insecure-ssl");
    // let ssl = insecure_ssl || args.is_present("ssl");

    let host = server.try_into()?;
    let config = rups::ConfigBuilder::new()
        .with_host(host)
        .with_debug(debug)
        // .with_ssl(ssl)
        // .with_insecure_ssl(insecure_ssl)
        .build();

    if args.get_flag("list") {
        return cmd::list_devices(config, false);
    }

    if args.get_flag("list-full") {
        return cmd::list_devices(config, true);
    }

    if args.get_flag("clients") {
        return cmd::list_clients(config, get_ups_name(&server)?);
    }

    // Fallback: prints one variable (or all of them)
    if let Some(variable) = args.get_one::<String>("variable") {
        cmd::print_variable(config, get_ups_name(&server)?, variable)
    } else {
        cmd::list_variables(config, get_ups_name(&server)?)
    }
}

fn get_ups_name<'a>(server: &'a UpsdName) -> anyhow::Result<&'a str> {
    server
        .upsname
        .with_context(|| "ups name must be specified: <upsname>[@<hostname>[:<port>]]")
}
