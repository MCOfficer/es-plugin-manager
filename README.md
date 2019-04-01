# es-plugin-manager (ESPIM)
A Proof of concept client for [endless-sky-plugins](https://github.com/MCOfficer/endless-sky-plugins).

## Usage
```ShellSession
$ espim -h
Endless Sky Plug-In Manager 0.1.0
A Proof-of-Concept Plug-In Manager for Endless Sky.

USAGE:
    espim [FLAGS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Prints moar info.

SUBCOMMANDS:
    help       Prints this message or the help of the given subcommand(s)
    init       Clones the Plug-In repository. Use before anything else.
    install    Installs a plug-in.
    list       Lists all available plug-ins.
    update     Updates the local Plug-In repository.

$ espim init
Done.

$ espim list
- all-content-plugin
- endless-sky-high-dpi
- world-forge

$ espim install world-forge
Attempting to install world-forge as [ESPIM] world-forge
Done.
```
![And it works!](https://i.imgur.com/pn3wdWV.png)
## Building

1. Install [rustup](https://rustup.rs/) (Choose the default values unless you know better)
2. build it using `cargo build` (without optimizations), `cargo build --release` or `cargo install --path .`
