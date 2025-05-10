# How It Works

Confignet is powered by a simple but effective heuristic system:

1. The `ConfigClassifier` is built from a CSV of known config files with associated MIME types and their labels (e.g., ci_cd or non_config).
2. When a file is passed to Confignet:
   - It extracts the filename from the full path.
   - It compares it against the CSV using Levenshtein distance on MIME-matched entries.
3. If a best match is found, the classifier returns:
   - `file_name`: matched entry name from CSV
   - `file_path`: reconstructed absolute or relative path
   - `is_ci_cd`: boolean indicating whether the file is related to CI/CD

It is designed for speed, accuracy, and pluggability in environments like local inference pipelines.

