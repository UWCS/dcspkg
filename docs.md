# Documentation

This document describes the codebase of this repo, and how to deploy and develop it.

## Overview

This repo is organised as a [Cargo Workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html). It's one cargo project, but contains a 5 crates:

- `dcspkg` is the command line client for downloading packages
- `dcspkg_server` is the server for serving packages
- `dcspkg_create` is a command line utility for adding a package to the server
- `dcspkg_client` is a client libray, providing a wrapper around the server's API
- `dcspkg_common` contains common types used within the system.

## Server

The server provides a REST API for downloading packages. A database contains a registry of packages on disk, which are all just tarballs sat in a directory the server provides a file server into. [Rocket](https://rocket.rs/) is used as an async web framework, and [sqlx](https://github.com/launchbadge/sqlx) is used to provide async database interaction.

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

## Releasing

## Deployment
