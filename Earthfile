VERSION 0.8
PROJECT nu_plugin_vec/plugin

IMPORT github.com/earthly/lib/rust:2.2.11 AS rust

install:
  FROM rust:1.81.0-bookworm
  RUN rustup component add clippy rustfmt

  # Call +INIT before copying the source file to avoid installing function depencies every time source code changes
  # This parametrization will be used in future calls to functions of the library
  DO rust+INIT --keep_fingerprints=true

source:
  FROM +install
  WORKDIR /build

  DO +COPY_SOURCE

# lint runs cargo clippy on the source code
lint:
  FROM +source

  DO rust+CARGO --args="clippy --all-features --all-targets -- -D warnings"

# build builds with the Cargo release profile
build:
  FROM +source

  DO rust+CARGO --args="build --release" --output="release/[^/\.]+"
  SAVE ARTIFACT ./target/release/ target AS LOCAL artifact/target

# test executes all unit and integration tests via Cargo
test:
  FROM +source

  DO rust+CARGO --args="test"

# fmt checks whether Rust code is formatted according to style guidelines
fmt:
  FROM +source

  DO rust+CARGO --args="fmt --check"

# all runs all other targets in parallel
check:
  BUILD +test
  BUILD +fmt

# bumps the version of the plugin if impactful commits have been made
bumpVersion:
  FROM commitizen/commitizen:3.29.0
  WORKDIR /build

  # Check whether the repository is in a good state to push
  DO +COPY_SOURCE
  WAIT
    BUILD +check
  END

  # Conservatively copy other files needed for next steps
  DO +COPY_CZ
  # Test whether a version bump is necessary
  IF cz bump --dry-run
    # Copy all files in the git repo, as cz bump does a git commit
    # (and we don't want to delete things from the repository)
    DO +COPY_ALL

    # Bump the version, and push this change to the repository
    RUN cz bump
    RUN --push git push
  END

# releases the plugin to crates.io
release:
  FROM +build

  COPY --keep-ts cargo_token README.md ./
  DO rust+CARGO --args="login < cargo_token" # Uses the cargo_token file to log in
  RUN rm -f cargo_token                      # Delete the cargo_token file! Otherwise it is pushed too

  DO rust+CARGO --args="publish"



COPY_ALL:
  FUNCTION

  COPY --keep-ts * ./
  COPY --keep-ts --dir * ./

COPY_CZ:
  FUNCTION

  DO +COPY_GIT             # Prerequisite, as the git information has to be present (tags, commit author has to be set)
  RUN git fetch --tags     # Explicitly fetch tags, as they are not included by default

  COPY --keep-ts .cz.toml ./

COPY_GIT:
  FUNCTION

  COPY --keep-ts --dir .git ./

COPY_SOURCE:
  FUNCTION

  COPY --keep-ts Cargo.toml Cargo.lock ./
  COPY --keep-ts --dir src ./