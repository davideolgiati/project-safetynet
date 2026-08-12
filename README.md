# project-safetynet

A small command-line backup tool written in Rust. It reads a list of jobs
from a config file, each pointing at a directory, and produces a
`.tar.gz` archive of the matching files.

## Install

Requires the Rust toolchain (edition 2024).

```
cargo build --release
```

The binary is produced at `target/release/project-safetynet`.

## Usage

```
project-safetynet [--config-path=<path/to/config.json>] [--no-logo] [--help]
```

- `--config-path=<path>` — path to the config file (default: `./config.json`)
- `--no-logo` — skip the startup banner
- `--help` — print usage and exit

Each valid job in the config is archived in turn; invalid entries are
skipped with a logged error. Archives are named
`<nickname>-<timestamp>.tar.gz` and written to the job's
`output_directory` (or the current directory if unset).

## Config file

The config file is a JSON array of job objects:

```json
[
  {
    "nickname": "docs",
    "input_path": "/home/user/Documents",
    "output_directory": "/home/user/backups",
    "compression_level": "best",
    "include": [".*\\.pdf"],
    "exclude": [".*contract.*\\.pdf"]
  }
]
```

- `nickname` (required) — job name, lowercase letters/digits only
  (`^[a-z0-9]+$`); also used as the archive's filename prefix.
- `input_path` (required) — directory to archive; must exist.
- `output_directory` (optional) — where the archive is written; must
  exist if given, defaults to the current directory.
- `compression_level` (optional) — `"fast"` or `"best"`; defaults to
  fast if omitted.
- `include` (optional) — list of regex patterns; a file must match at
  least one to be included. If omitted, all files are included.
- `exclude` (optional) — list of regex patterns; a file matching any of
  these is skipped, even if it matched `include`.
