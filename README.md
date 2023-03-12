# dcspkg

[![](https://img.shields.io/crates/v/dcspkg)](https://crates.io/crates/dcspkg)
[![GitHub Workflow Status (with branch)](https://img.shields.io/github/actions/workflow/status/UWCS/dcspkg/ci.yml?branch=main)](https://github.com/UWCS/dcspkg/actions)

`dcspkg` is a simple package manager, designed for used on DCS systems, or any system where packages cannot be installed as root. It fetches packages containing precompiled binaries from a server, and then installs them under your home directory.

## Documentation

This repo is a cargo workspace containing three crates:

- `dcspkg`, the CLI package manager
- `dcspkg_server`, the package server
- `dcspkg_create`, a helper tool for creating `.dcspkg` archives

See the tech team wiki page for full documentation: https://techteam.uwcs.co.uk/en/apps/dcspkg

## Contributing

Contributions are welcome and encouraged. Check out the [issues](https://github.com/UWCS/dcspkg/issues) for things that we've noted need working on. 
