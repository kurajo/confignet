# Troubleshooting

### ❌ Error: No match found
- Ensure the MIME type is correct
- Add more diverse entries to the CSV
- Normalize file names

### ❌ Panic: Failed to extract file name
- Ensure you are passing valid paths
- Use PathBuf methods to extract names reliably

### ❌ Invalid CSV format
- Check for unescaped commas or quotes
- All rows must follow `file_name,mime_label,config_type`

### ❌ All results return `is_ci_cd: false`
- Check your `config_type` column values
- Add more known CI/CD examples to improve accuracy

### ✅ Tip
Use tools like `magika`, `file`, or `xdg-mime` to generate MIME labels.

