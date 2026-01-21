# rs_gitdiffcopy

[日本語](README.md)

A CLI tool that makes `git diff` results easier to understand for beginners and non-technical users. It copies changed files into a separate folder while preserving the directory structure, and can output summaries and Excel reports.

## Features

- **Two-way comparison**: Compare two refs (branch/tag/commit) and extract diffs
- **Three-way comparison**: Compare base/ours/theirs and detect conflicts
- **Structure preserved**: Keeps original directory structure in output
- **Patch generation**: Generate unified diff patches for changed files
- **Excel report**: Export results to an Excel file (.xlsx)
- **Parallel processing**: Faster for large file sets
- **Config file**: Reusable TOML config

## Installation

### Build from source

```bash
cargo build --release
```

The binary is generated at `target/release/rs_gitdiffcopy`.

### Install with Cargo

```bash
cargo install --path .
```

> `git` is required at runtime.

## Basic usage

### Two-way comparison

```bash
# Basic usage
rs_gitdiffcopy -S <source> -T <target> -O <output>

# Example: compare main and develop
rs_gitdiffcopy -S main -T develop -O output
```

### Three-way comparison

```bash
# Three-way (merge)
rs_gitdiffcopy --three-way -B <base> -S <ours> -T <theirs> -O <output>

# Example
rs_gitdiffcopy --three-way -B main -S feature/ours -T feature/theirs -O output
```

## All options

| Option | Short | Description |
|--------|-------|-------------|
| `--source` | `-S` | Source ref (branch/tag/commit) |
| `--target` | `-T` | Target ref (branch/tag/commit) |
| `--output` | `-O` | Output directory |
| `--repository` | `-R` | Repository path or URL (current version supports local path only) |
| `--full-clone` | | Full clone for remote URL (not implemented) |
| `--cache-dir <PATH>` | | Cache directory for remote clone (not implemented) |
| `--config <PATH>` | `-c` | Config file (TOML) |
| `--git-path <PATH>` | | Path to git executable |
| `--exclude <PATTERN>` | `-e` | Exclude patterns (glob, repeatable) |
| `--force` | `-f` | Delete output directory and re-run (confirmation required) |
| `--summary <PATH>` | `-s` | Write summary to a file |
| `--verbose` | `-v` | Verbose mode (prints file names during processing) |
| `--dry-run` | `-n` | Dry run (do not copy files) |
| `--both-versions` | `-b` | Copy both old/new versions (.old/.new) |
| `--check-permissions <MODE>` | `-P` | Permission check (none/scripts/all) |
| `--patch` | `-p` | Generate per-file patches |
| `--patch-file <PATH>` | `-F` | Generate a combined patch file |
| `--excel <PATH>` | `-E` | Generate an Excel report |
| `--excel-fold-level <LEVEL>` | `-L` | Fold level for Excel tree |
| `--show-unchanged` | `-u` | Show unchanged files in summary details |
| `--save-config <PATH>` | `-C` | Save current options and exit |
| `--filter-status <STATUS>` | | Status filter (comma-separated) |
| `--stats-only` | | Show only statistics |
| `--no-tree` | | Hide File Tree section |
| `--no-details` | | Hide detail sections |
| `--copy-deleted` | | Copy deleted files (.deleted) |
| `--preserve-timestamps` | | Preserve timestamps |
| `--workers <NUM>` | `-j` | Number of worker threads |
| `--temp-dir <PATH>` | | Temporary directory |
| `--color <MODE>` | | Color output (auto/always/never) |
| `--log-level <LEVEL>` | | Log level (error/warn/info/debug) |
| `--three-way` | `-3` | Three-way comparison mode |
| `--base` | `-B` | Base ref for three-way |
| `--merge-style` | `-M` | Merge style (all/ours/theirs) |
| `--conflict-only` | | Output only conflicts |
| `--help` | `-h` | Show help |
| `--version` | `-V` | Show version |

> Note: `--stats-only` affects console output only. Full summaries are still written to `--summary` and `--excel`.

## Examples (detailed)

### Exclude patterns

```bash
# Exclude log files and node_modules
rs_gitdiffcopy -S main -T develop -O output -e "*.log" -e "node_modules/**"
```

### Output and reports

```bash
# Write summary to a file
rs_gitdiffcopy -S main -T develop -O output --summary summary.txt

# Generate Excel report
rs_gitdiffcopy -S main -T develop -O output --excel report.xlsx
```

### Patch generation

```bash
# Per-file patches
rs_gitdiffcopy -S main -T develop -O output --patch

# Combined patch file
rs_gitdiffcopy -S main -T develop -O output --patch-file changes.patch

# Both
rs_gitdiffcopy -S main -T develop -O output --patch --patch-file changes.patch
```

### Copy options

```bash
# Copy both versions
rs_gitdiffcopy -S main -T develop -O output --both-versions

# Copy deleted files
rs_gitdiffcopy -S main -T develop -O output --copy-deleted

# Preserve timestamps
rs_gitdiffcopy -S main -T develop -O output --preserve-timestamps
```

### Filters and display

```bash
# Show only added and modified
rs_gitdiffcopy -S main -T develop -O output --filter-status added,modified

# Show everything except unchanged (implicit all + exclude)
rs_gitdiffcopy -S main -T develop -O output --filter-status ^unchanged

# Hide File Tree
rs_gitdiffcopy -S main -T develop -O output --no-tree

# Hide details
rs_gitdiffcopy -S main -T develop -O output --no-details
```

### Color and logging

```bash
# Always color output
rs_gitdiffcopy -S main -T develop -O output --color always

# Debug logs
rs_gitdiffcopy -S main -T develop -O output --log-level debug
```

### Performance

```bash
# Set worker count
rs_gitdiffcopy -S main -T develop -O output --workers 4

# Set temp directory
rs_gitdiffcopy -S main -T develop -O output --temp-dir /tmp/rs_gitdiffcopy
```

### Permission checks

```bash
# Script files only
rs_gitdiffcopy -S main -T develop -O output --check-permissions scripts

# All files
rs_gitdiffcopy -S main -T develop -O output --check-permissions all
```

### Save config

```bash
# Save current options and exit
rs_gitdiffcopy -S main -T develop -O output -e "*.log" --save-config gitdiffcopy.toml
```

### Three-way comparison

```bash
# Conflicts only
rs_gitdiffcopy --three-way -B main -S feature/ours -T feature/theirs -O output --conflict-only

# Prefer ours
rs_gitdiffcopy --three-way -B main -S feature/ours -T feature/theirs -O output --merge-style ours

# Prefer theirs
rs_gitdiffcopy --three-way -B main -S feature/ours -T feature/theirs -O output --merge-style theirs
```

## Exit codes

| Code | Meaning |
|------|---------|
| 0 | Differences found (success) |
| 1 | Error |
| 2 | No differences |
| 3 | Conflicts found (three-way) |

## Config file

You can use a TOML config file. For application-wide `settings.toml`, see `rs_gitdiffcopy.md`.

```toml
# config.toml
source = "main"
target = "develop"
output = "./output"
exclude = ["*.log", "node_modules/**", "__pycache__/**"]
both_versions = true
patch = true
verbose = false
```

Use config file:

```bash
rs_gitdiffcopy --config config.toml
```

Save current options and exit:

```bash
rs_gitdiffcopy -S main -T develop -O output -e "*.log" --save-config gitdiffcopy.toml
```

## Important note

- The current version **does not support remote URL comparison**. Use local paths with `--repository`.

## Requirements

- Rust 1.70+
- Supported OS: Linux, macOS, Windows

## License

MIT License
