# U2 CLI

A cli of u2.dmhy

```
Usage: u2_cli <COMMAND>

Commands:
  config    Configure the CLI
  download  Download a torrent and add it to transmission
  clean     Clean all torrents from the save directory
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

Only available on maxos, linux, freebsd or other unix like system.

## Installation

Assume you have rust environment installed.

steps:

1. clone the repository
2. `cargo build --release`
3. `cargo install --path .`
