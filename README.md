# nu_plugin_vec

A plugin for [Nushell](https://nushell.sh), a cross-platform shell and scripting language. This plugin adds support for
vector operations.

## Status

The plugin is still under construction at the moment.
Work is being done on getting the plugin stable and tested, before an eventual v1 release.

## Installation

### Cargo

Early versions are now shipping to [crates.io](https://crates.io/crates/nu_plugin_vec), and can thus be installed by
with Cargo:

```bash
cargo install nu_plugin_vec             # Downloads and installs the plugin
plugin add ~/.cargo/bin/nu_plugin_vec   # Registers the plugin with Nushell
plugin use vec                          # Activates the plugin
```

### Manual build

Manual builds can also be used:

```bash
git clone https://github.com/PhotonBursted/nu_plugin_vec.git  # Clone the repository
cd nu_plugin_vec                                              # Enter the repo folder
cargo build -r                                                # Build a release version of the plugin
plugin add target/release/nu_plugin_vec                       # Registers the plugin with Nushell
plugin use vec                                                # Activates the plugin
```