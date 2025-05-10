# Integration in Projects

Confignet is designed to be embedded easily.

## As a Library

Import it in your Rust project:

```rust
use confignet::ConfigClassifier;

let classifier = ConfigClassifier::from_csv("data/labeled/ci_cd.csv")?;
let result = classifier.classify("Cargo.toml", "toml");
```

## As a CLI in Automation Pipelines

Use Magika (or similar tool) to detect file types:

```bash
magika path/to/file | jq '.mimetype'
```

Then pass the result to Confignet:

```bash
confignet path/to/file toml
```

Pipe JSON output to your parser or decision logic.

## In `dodo`

Confignet is integrated directly into `dodo` to:

* Skip non-CI/CD files
* Send CI/CD-related configs to parsers
* Build `dodo.toml` incrementally
