# Confignet

📁 A lightweight configuration file classifier for CI/CD tools — fast, pluggable, and embeddable.

Confignet is a Rust library and optional CLI tool that takes a file path and MIME type, then classifies whether the file is related to CI/CD tooling or not. It's designed for projects like [Dodo](https://github.com/kurajo/dodo) that need to process config files across a wide variety of languages and tools.

---

## ✨ Features

- Classifies config files as CI/CD or non-CI/CD using a lightweight CSV-based lookup.
- Computes Levenshtein similarity to match noisy filenames.
- Designed to be embedded (e.g. in automated pipelines).
- Includes a CLI for testing and debugging locally.

---

## 🚀 Usage (Library)

Add it to your `Cargo.toml`:

```toml
[dependencies]
confignet = "0.1"
````

Use it in your Rust project:

```rust
use confignet::{ConfigClassifier, ClassifiedResult};

let classifier = ConfigClassifier::from_csv("data/labeled/ci_cd.csv")?;
let file_path = "path/to/Cargo.toml";
let mime_type = "text/plain";

if let Some(result) = classifier.classify(file_path, mime_type) {
    println!("Is CI/CD: {}", result.is_ci_cd);
    println!("File: {}", result.file_path);
}
```

---

## 🛠 CLI (Optional)

You can run Confignet from the command line:

```sh
cargo run -- path/to/file.yaml application/x-yaml
```

This prints a structured JSON result:

```json
{
  "file_name": "file.yaml",
  "file_path": "./file.yaml",
  "is_ci_cd": true
}
```

---

## 📦 CSV Format

Confignet reads a CSV like this:

```csv
file_name,mime_label,config_type
Cargo.toml,text/plain,ci_cd
.github/workflows/main.yml,application/x-yaml,ci_cd
Makefile,text/x-makefile,non_config
```

* `mime\_label`: MIME type reported by file detectors like [`magika`](https://github.com/google/magika)
* `config\_type`: "ci\_cd" or "non\_config"

---

## 🧩 Integration with Dodo

Confignet is designed to be part of the [Dodo](https://github.com/kurajo/dodo) system:

```
Magika → Confignet → Parser (for CI/CD files) → dodo.toml
```

Confignet handles whether a file is CI/CD-related based on name and MIME type, enabling smarter filtering before parsing begins.

---

## 📁 File Path Heuristics

* If the file is in the project root, Confignet normalizes the file path to `./<file_name>`.
* Otherwise, it returns the full path.

---

## 🔧 Development

Run tests:

```sh
cargo test
```

Try the CLI:

```sh
cargo run -- path/to/file.rs text/plain
```

---

## 📄 License

MIT and Apache 2.0

---

Built by [Kurajo](https://github.com/kurajo/) 🚀
