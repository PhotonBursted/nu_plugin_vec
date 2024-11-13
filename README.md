# nu_plugin_vec

[![Crates.io Version](https://img.shields.io/crates/v/nu_plugin_vec)](https://crates.io/crates/nu_plugin_vec)
[![Nushell](https://img.shields.io/badge/Nushell-v0.100.0-blue)](https://nushell.sh)

A plugin for [Nushell](https://nushell.sh), a cross-platform shell and scripting language. This plugin adds support for
vector operations.

## Installation

### Cargo

Get the latest version from [crates.io](https://crates.io/crates/nu_plugin_vec) with a local install:

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

## Features

The plugin offers adds some new commands, which aim to make scripting vector-like operations a smoother experience.
Below, a few use cases are shown, which outline the difference this plugin makes.

### Showcase

#### Addition and subtraction

```
# Vanilla Nushell
$vec1 | zip $vec2 | each { |pair| ($pair | first) + ($pair | last) }

# Nushell + nu_plugin_vec
$vec1 | vec add $vec2
```

#### Dot product

```
# Vanilla Nushell
$vec1 | zip $vec2 | each { |pair| ($pair | first) * ($pair | last) } | math sum

# Nushell + nu_plugin_vec
$vec1 | vec dot $vec2
```

#### Vector similarity

```
# Vanilla Nushell
let dot_product = ($vec1 | zip $vec2 | each { |pair| ($pair | first) * ($pair | last) } | math sum)
let magnitude1  = ($vec1 | each { |v| $v * $v } | math sum | math sqrt)
let magnitude2  = ($vec2 | each { |v| $v * $v } | math sum | math sqrt)
$dot_product / ($magnitude1 * $magnitude2)

# Nushell + nu_plugin_vec
$vec1 | vec cos $vec2
```

### Command list

The list of commands currently available is as follows:

- `vec add` and `vec sub` for addition and subtraction
- `vec cos` and `vec sin` for angle calculation
- `vec dot` for dot products
- `vec sqnorm` and `vec magnitude` for length measurements
- `vec normalize` for conversions into unit vectors

For more information, invoke `help <command>` in your Nushell session.