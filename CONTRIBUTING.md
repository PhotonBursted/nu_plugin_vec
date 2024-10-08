# Contributing to nu_plugin_vec

----

This file provides guidance on how you can contribute the plugin.

## Filing bugs
Before filing a bug, ensure it isn't part of the issues already existing in the repository.

In case there is no issue, create one.
Pay attention to the following:
- Provide a clear title and description, including as much relevant information as possible.
- Show the Nushell version, plugin version, and other installed plugins (`plugin list`).

## Contributing code
We welcome contributions, though they should adhere to a few guidelines, in order to keep the repository tidy.

### Workflow
When making changes to the code, first fork the repository, and make your changes to that fork.
This keeps the plugin repository clean of orphan branches and development noise.

Then, run the `workspace/setup.nu` script, which set up your local workspace.

While working on the code, ensure:
- the code style matches the project's code style
- additions are properly covered by tests
- _no changes are made to CHANGELOG.md or versions_

When the work is done, create a PR to the `main` branch of this repository, to allow for reviews and comments.

### Commits
Commits should follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/#summary) specification.
Versioning and changelogs are generated from the commits using [Commitizen](https://commitizen-tools.github.io/commitizen/).

Commits are **required** to follow this format.
This due to the previously mentioned automatic versioning, and improved legibility.
