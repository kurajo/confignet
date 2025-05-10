use std::{env, path::Path, process};
use confignet::ConfigClassifier;
use anyhow::Result;
use serde_json::{json, Value};

fn main() -> Result<()> {
// 1. Get file_path and mime_type from args
let args: Vec<String> = env::args().collect();
if args.len() != 3 {
eprintln!("Usage: confignet <file_path> <mime_type>");
process::exit(1);
}

let file_path = args[1].clone();
let mime_type = args[2].clone();
let file_name = Path::new(&file_path)
    .file_name()
    .and_then(|n| n.to_str())
    .unwrap_or("")
    .to_string();

// 2. Load classifier
let classifier = ConfigClassifier::from_csv("data/labeled/ci_cd.csv")?;

// 3. Classify
let result: Value = match classifier.classify(&file_name, &mime_type) {
    Some((file_name, _, is_ci_cd)) => {
        let path = if Path::new(&file_path).parent().unwrap_or_else(|| Path::new("")) == Path::new("") {
            format!("./{}", file_name)
        } else {
            file_path.clone()
        };

        json!({
            "file_name": file_name,
            "file_path": path,
            "is_ci_cd": is_ci_cd
        })
    }
    None => {
        let path = if Path::new(&file_path).parent().unwrap_or_else(|| Path::new("")) == Path::new("") {
            format!("./{}", file_name)
        } else {
            file_path.clone()
        };

        json!({
            "file_name": file_name,
            "file_path": path,
            "is_ci_cd": false
        })
    }
};

// Instead of println! send this `result` to your parser
// For now, just printing nicely for testing
println!("{}", serde_json::to_string_pretty(&result)?);

Ok(())

}