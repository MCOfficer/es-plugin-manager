# es-plugin-manager (ESPIM)
A Proof of concept client for [endless-sky-plugins](https://github.com/MCOfficer/endless-sky-plugins).

## Usage
```ShellSession
$ espim -h
Endless Sky Plug-In Manager 0.2.0
A Proof-of-Concept Plug-In Manager for Endless Sky.

USAGE:
    espim.exe [FLAGS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Prints moar info.

SUBCOMMANDS:
    help       Prints this message or the help of the given subcommand(s)
    init       Clones the plug-In repository. Use before anything else.
    install    Installs a plug-in.
    list       Lists all available plug-ins.
    remove     Removes a plug-in. The Plug-In stays on the disk, but is removed from the plug-in folder.
    update     Updates the local plug-In repository.

$ espim init
Done.

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
