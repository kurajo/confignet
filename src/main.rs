use std::{env, process};
use confignet::ConfigClassifier;
use anyhow::Result;

fn main() -> Result<()> {
    // 1. Get file name, mime type, and project root from args
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: cnet <file_name> <mime_type> <project_root>");
        process::exit(1);
    }
    
    let file_name = args[1].clone();
    let mime_type = args[2].clone();
    let project_root = args[3].clone();
    
    // 2. Load classifier and classify using the corrected function
    let classifier = ConfigClassifier::from_csv("data/labeled/ci_cd.csv")?;
    
    // 3. Classify and output structured result
    match classifier.classify(&file_name, &mime_type, &project_root) {
        Some((file_name, file_path, is_ci_cd)) => {
            println!("{{");
            println!("  \"file_name\": \"{}\",", file_name);
            println!("  \"file_path\": \"{}\",", file_path);
            println!("  \"is_ci_cd\": {}", is_ci_cd);
            println!("}}");
        },
        None => {
            println!("{{");
            println!("  \"file_name\": \"{}\",", file_name);
            println!("  \"is_ci_cd\": false");
            println!("}}");
        },
    }    

    Ok(())
}
