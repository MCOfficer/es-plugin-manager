extern crate clap;

use clap::{App, Arg, SubCommand};

mod lib;

const VERSION: &str = "0.3.1";

fn main() {
    let matches = App::new("Endless Sky Plug-In Manager")
        .version(VERSION)
        .about("A Proof-of-Concept Plug-In Manager for Endless Sky.")
        .subcommand(SubCommand::with_name("update")
            .about("Updates the local plug-in repository.")
        )
        .subcommand(SubCommand::with_name("upgrade")
            .about("Upgrades all installed and plug-ins")
        )
        .subcommand(SubCommand::with_name("list")
            .about("Lists all available plug-ins.")
        )
        .subcommand(SubCommand::with_name("install")
            .about("Installs a plug-in.")
            .arg(Arg::with_name("PLUGIN")
                .help("The plug-in to install.")
                .required(true)
                .index(1)
            )
        )
        .subcommand(SubCommand::with_name("purge")
            .about("Removes a plug-in from the disk entirely.")
            .arg(Arg::with_name("PLUGIN")
                .help("The plug-in to purge.")
                .required(true)
                .index(1)
            )
        )
        .subcommand(SubCommand::with_name("remove")
            .about("Removes a plug-in. The plug-in stays on the disk, but is removed from the ES plug-in folder.")
            .alias("uninstall")
            .arg(Arg::with_name("PLUGIN")
                .help("The plug-in to remove.")
                .required(true)
                .index(1)
            )
        )
        .arg(Arg::with_name("verbose")
            .long("verbose")
            .short("v")
            .help("Prints moar info.")
        ).get_matches();

    let verbose = matches.is_present("verbose");
    if verbose {
        println!("Verbose Output enabled.");
        println!("ESPIM v{}", VERSION);
    }

    if let Some(_matches) = matches.subcommand_matches("update") {
        lib::update(verbose);
    } else if let Some(_matches) = matches.subcommand_matches("upgrade") {
        lib::upgrade(verbose);
    } else if let Some(_matches) = matches.subcommand_matches("list") {
        lib::list(verbose);
    } else if let Some(matches) = matches.subcommand_matches("install") {
        lib::install(matches.value_of("PLUGIN").unwrap(), verbose);
    } else if let Some(matches) = matches.subcommand_matches("purge") {
        lib::purge(matches.value_of("PLUGIN").unwrap(), verbose);
    } else if let Some(matches) = matches.subcommand_matches("remove") {
        lib::remove(matches.value_of("PLUGIN").unwrap(), verbose);
    } else {
        println!("See espim -h for help");
    }
}
