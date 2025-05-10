# API Reference

This page documents the public API of the Confignet library. If you are embedding Confignet into another tool (like Dodo), you’ll primarily interact with the `ConfigClassifier` type.

---

## Structs

### `ConfigRecord`

A deserialized record from the classifier CSV.

```rust
pub struct ConfigRecord {
    pub file_name: String,
    pub mime_label: String,
    pub config_type: String,
}
```

Fields:

* `file_name`: The canonical file name for comparison (e.g. `Cargo.toml`)
* `mime_label`: The mime-type label assigned to the file (e.g. `toml`, `yaml`)
* `config_type`: Either a type like `ci_cd`, `build`, or `non_config`

This struct is used internally by the classifier.

---

### `ConfigClassifier`

The main classifier struct that loads and queries classification rules.

```rust
pub struct ConfigClassifier {
    // Hidden internals
}
```

#### Constructor

```rust
pub fn from_csv<P: AsRef<Path>>(path: P) -> Result<Self>
```

Loads a `ConfigClassifier` from a given CSV file.

* `path`: The path to the `.csv` file
* Returns: `Result<ConfigClassifier>`

Usage:

```rust
let classifier = ConfigClassifier::from_csv("data/labeled/ci_cd.csv")?;
```

#### Method

```rust
pub fn classify(&self, file_name: &str, mime_label: &str) -> Option<ClassifiedResult>
```

Attempts to classify a file given its name and mime type.

* `file_name`: Name of the file (e.g., `main.rs`, `Dockerfile`)
* `mime_label`: Mime type label from tools like Magika (e.g., `toml`, `json`)
* Returns: `Option<ClassifiedResult>`, or `None` if no suitable match is found

Example:

```rust
let result = classifier.classify("Cargo.toml", "toml");
```

---

## Structs

### `ClassifiedResult`

Returned from `classify()` if a match is found.

```rust
pub struct ClassifiedResult {
    pub file_name: String,
    pub is_ci_cd: bool,
}
```

Fields:

* `file_name`: The best-matching canonical file name (e.g., from CSV)
* `is_ci_cd`: Whether this file is used for CI/CD based on `config_type`

---

## Internal Utilities

Confignet also includes a Levenshtein distance utility for fuzzy file matching:

```rust
fn levenshtein(a: &str, b: &str) -> usize
```

This is used internally in `classify()` to find the closest filename match in the dataset when multiple candidates exist with the same mime type.

---

## Example Integration

```rust
use confignet::{ConfigClassifier, ClassifiedResult};

let classifier = ConfigClassifier::from_csv("data/labeled/ci_cd.csv")?;
let result = classifier.classify("Dockerfile.ci", "text");

match result {
    Some(r) => println!("File: {}, Is CI/CD? {}", r.file_name, r.is_ci_cd),
    None => println!("Unrecognized file"),
}
```
