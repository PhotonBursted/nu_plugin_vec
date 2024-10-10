### Builds a version of the plugin locally, without deploying it to crates.io, and registers it with nu.

use std log

def main [] {
    let target = (pwd | path join "dist")
    let repository_root = (pwd)


    # Build package
    let build_command = $"cargo install --path ($repository_root) --root ($target)"
    log info $"Building plugin - executing '(ansi blue)($build_command)(ansi reset)'"
    nu -c $build_command

    # Register package with nu
    let package_name = (open ($repository_root | path join "Cargo.toml") | get package.name)
    let extension = if ($nu.os-info.name == 'windows') { '.exe' } else { '' }
    plugin add $"($target | path join "bin" $package_name)($extension)"


    log info "Build successful!"
}