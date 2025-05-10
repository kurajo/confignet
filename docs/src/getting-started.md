# Getting Started

## Installation

You can add Confignet to your project by adding this line to your `Cargo.toml`:

```toml
confignet = "0.1"
```

Or install the CLI tool locally:

```bash
cargo install --path .
```

## CLI Usage

```bash
confignet <file_path> <mime_type>
```

Example:

```bash
confignet ./Cargo.toml toml
```

This will output:
```json
{
  "file_name": "Cargo.toml",
  "file_path": "./Cargo.toml",
  "is_ci_cd": true
}
```
