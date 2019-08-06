# es-plugin-manager (ESPIM) [![Build Status](https://travis-ci.org/MCOfficer/es-plugin-manager.svg?branch=master)](https://travis-ci.org/MCOfficer/es-plugin-manager)
A Proof of concept Plug-In Manager for [Endless Sky](https://endless-sky.github.io).

## Usage
```ShellSession
$ espim -h
Endless Sky Plug-In Manager 0.3.1
A Proof-of-Concept Plug-In Manager for Endless Sky.

USAGE:
    espim.exe [FLAGS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Prints moar info.

SUBCOMMANDS:
    help       Prints this message or the help of the given subcommand(s)
    install    Installs a plug-in.
    list       Lists all available plug-ins.
    purge      Removes a plug-in from the disk entirely.
    remove     Removes a plug-in. The plug-in stays on the disk, but is removed from the ES plug-in folder.
    update     Updates the local plug-in repository.
    upgrade    Upgrades all installed and plug-ins

$ espim list
 Installed |                  Name                  |                   Version
-----------|----------------------------------------|---------------------------------------------
    No     |  Sarcina                               |  6891ccaf0be803d7dfb7e96fa5d46f318b9a8976
    No     |  all-content-plugin                    |  a2f7175d89ea205377b4cbf1e044e1d41187b202
    No     |  endless-sky-high-dpi                  |  eb7c808e1b19db24ab0cdf427d8303afc6f4ee36
    No     |  world-forge                           |  e9dfb35a76298b376a0cddae62fb113941a903ec

$ espim install world-forge
Attempting to install world-forge as [ESPIM] world-forge
Done.
```
![And it works!](https://i.imgur.com/pn3wdWV.png)

## Building

1. Install [rustup](https://rustup.rs/) (Choose the default values unless you know better)
2. build it using `cargo build` (without optimizations), `cargo build --release` or `cargo install --path .`
