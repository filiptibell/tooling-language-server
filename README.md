# Tooling Language Server

A language server for several tools:

- [Aftman](https://github.com/LPGhatguy/aftman)
- [Foreman](https://github.com/roblox/foreman)
- [Wally](https://github.com/UpliftGames/wally)

Mostly a personal project, to learn how to write a performant language server.

## Aftman

Features that are currently supported:

- Diagnostics for:
  - A newer tool version is available
  - Invalid author / name / version
- Hover for information about a tool (description)
- Autocomplete for commonly used tool authors & names
- Quick action to update to new tool version

Features that will be supported:

- Diagnostic for unsupported platform/arch
- All of the listed Aftman features for Foreman as well

## Cargo

Features that will be supported:

- Diagnostics for:
  - A newer package version is available
  - Invalid package name / version
- Hover for information about a package (description)
- Quick action to update to a new package version

## Wally

Features that will be supported:

- Diagnostics for invalid package names and/or realms
- Autocomplete for dependencies - scopes + names + versions
- Hover for more information about a package
- Inlay hints and quick actions for new package versions
