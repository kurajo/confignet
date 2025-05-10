# Confignet

Confignet is a lightweight, pluggable configuration file classifier built in Rust. It’s designed to identify CI/CD-related configuration files from a given project using a fast, Levenshtein-distance-based matching system over a CSV training set.

Built for integration into larger systems like `dodo`, Confignet allows intelligent automation pipelines to skip irrelevant files and focus only on what matters: CI/CD infrastructure.

- 🧠 Zero-network AI
- ⚡ Fast, accurate lookup
- 🧩 Simple CSV-based extensibility
- 📦 Available as a library or a CLI tool

Confignet is ideal for:
- Classifying files detected by file-type scanners (e.g. Magika)
- Filtering config files before parsing them
- Auto-generating structured project pipelines

