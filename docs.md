# Documentation

This document describes the codebase of this repo, and how to deploy and develop it.

## Overview

This repo is organised as a [cargo workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html). It's one cargo project, but contains 3 crates:

- `dcspkg` is the command line client for downloading packages
- `dcspkg_server` is the server for serving packages
- `dcspkg_create` is a command line utility for adding a package to the server

## CLI (`dcspkg`)

This is the tool that is used to interact with the server.

- [`clap`](https://github.com/clap-rs/clap) is used to provide a nice cli
- [`reqwest`](https://github.com/seanmonstar/reqwest) is used for http requests
- [`config-rs`](https://github.com/mehcode/config-rs) is used to provide a configuration system

### Subcommands

- `list`
  - Fetch all packages and list them to stdout
  - Optionally dump json instead
- `install <pkgname>`
  - Install a package, specified by it's pkgname
- `installed`
  - Show all installed packages
  - Optionall dump json instead
- `run <pkgname>`
  - Run the executable within a package

### Code Organisation

- `main.rs`
  - Entry point
- `lib.rs`
  - Contains types that are to be exposed as a library for use by other crates
  - Current just the `Package` struct
- `config.rs`
  - Contains types and functions for defining the configuration, and loading it from a file/environment variables
- `util.rs`
  - Misc utility and helper functions
- `cli.rs`
  - Contains the definition of the command line interface using clap
  - Contains the entry point for each subcommand
- `commands`
  - Contains code associated with various subcommands
  - `install.rs`
    - Code to handle installing a package
  - `list.rs`
    - Code to fetch a package list

## Server (`dcspkg_server`)

The server provides a REST API for downloading packages. A database contains a registry of packages on disk, which are all just tarballs sat in a directory the server provides a file server into. [Rocket](https://rocket.rs/) is used as an async web framework, and [sqlx](https://github.com/launchbadge/sqlx) is used to provide async database interaction.

It uses the lib exposed by `dcspkg` to provide the definition of the `Package` struct.

### API Endpoints

- `/list` - returns a list of all the packages in the database
- `/pkgdata/<name>` - get all the data of a package by name
- `/download` - a file server into the package directory

### Code Organisation

- `main.rs`
  - Entry point
- `db.rs`
  - Async functions to get packages from the database and return their info as Rust structs
- `handlers.rs`
  - The function handlers for the API endpoints

## Create (`dcspkg_create`)

This tool takes a directory and packages it up, writing the metadata you give it to the database. See `dcspkg-create --help` for usage info. The tool will prompt you with various options that you may configure.

### Code Organisation

- `main.rs`
  - Entry point
  - Contains most of the driver code for prompting
  - Contains CLI definition using `clap`
- `db.rs`
  - Helpers for interacting with the database
  - We use `sqlx` here too, but use the `smol` async runtime to wrap the async calls into synchronous functions
- `archive.rs`
  - Stuff for interacting with archive files
- `opts.rs`
  - Functions for prompting for each option
  - We use [dialoguer](https://github.com/mitsuhiko/dialoguer) for fancy stdin prompts

## Deployment

All three crates in this repo serve different purposes, and have different deployment methods

### CLI

The CLI is published to [crates.io](https://crates.io/crates/dcspkg). CI is set up such that whenever a version tag is pushed, it will automatically create a release on Github, and then publish the current version to crates.io.

To trigger this, create a tag that starts with the letter v and contains a valid semver version, ie `v0.2.1` (`git tag v0.2.1` will tag the current commit), and then push the tag to Github (`git push --tags`).

It would be nice if this CI job also built a binary and attached it to the release on Github for download.

### Server

The server is designed to run within a docker container. Every push to master will build a new container image for `dcspkg-server` and publish it to `ghcr.io/uwcs/dcspkg-server:latest`. Note that the server does not use the version of the library exposed by `dcspkg` from crates.io, but from the main branch on `Github`.

Re-deployment on the host will need to be done manually. This is done easiest using portainer, see the tech team wiki for more.

### Create

This doesn't really need deploying. If you need this tool, just pull the repo and build a binary.

## Package Format

A `.dcspkg` file is just a `.tar.gz`.

- The name of the package should be the same as `pkgname` in the database
- The database contains the relative path of the executable within the package
  - This file is run when doing `dcspkg run`
- Packages may contain an `install.sh` script, which will be run by `dcspkg install` if the database says that there is one

## Server Repo Layout

The package repo is on beryllium at `/home/uwucs/packages`.

- `packages/packagedb.sqlite` is the package database
- The `packages/packages` directory contains all the package archives

Docker uses a bind mount to mount this directory in the container. A bind mount is used over a volume to make it easier to add to/edit the package repos on the host system.

## Local Repo Layout

The CLI creates `$HOME/.dcspkg` when you first use it.

- `.dcspkg/config.toml` contains the config for the cli
  - The three paths below, as well as server url, can be configured here
- `.dcspkg/registry.json` contains the metadata for all packages you have installed
- `.dcspkg/bin` contains symlinks to executables for packages that requested to be added to path
- `.dcspkg/package` contains all the packages

## Development Notes

- Do not change the database schema or package format without good reason. Changing it will mean having to manually rebuild all the packages, which takes a lot of time.
- Feel free to expose more API endpoints
