use serde::Deserialize;
use anyhow::Result;
use std::{fs::File, path::Path, io::BufReader};

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigRecord {
    pub file_name: String,
    pub mime_label: String,
    pub config_type: String,
}

pub struct ConfigClassifier {
    records: Vec<ConfigRecord>,
}

impl ConfigClassifier {
    pub fn from_csv<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path)?;
        let mut rdr = csv::Reader::from_reader(BufReader::new(file));
        let mut records = Vec::new();

        for result in rdr.deserialize() {
            let record: ConfigRecord = result?;
            records.push(record);
        }

        Ok(Self { records })
    }

    pub fn classify(&self, file_name: &str, mime_label: &str, project_root: &str) -> Option<(String, String, bool)> {
        // Find the config record based on the file name and mime label
        self.records
            .iter()
            .filter(|r| r.mime_label == mime_label)
            .min_by_key(|r| levenshtein(&r.file_name, file_name))
            .map(|r| {
                // Dynamically construct file path from project root and file_name
                let file_path = format!("{}/{}", project_root, r.file_name);
                let is_ci_cd = r.config_type != "non_config";
                (r.file_name.clone(), file_path, is_ci_cd)
            })
    }
}

fn levenshtein(a: &str, b: &str) -> usize {
    let mut costs = (0..=b.len()).collect::<Vec<_>>();
    for (i, ca) in a.chars().enumerate() {
        let mut last_val = i;
        costs[0] = i + 1;
        for (j, cb) in b.chars().enumerate() {
            let new_val = if ca == cb {
                last_val
            } else {
                1 + *[last_val, costs[j], costs[j + 1]].iter().min().unwrap()
            };
            last_val = costs[j + 1];
            costs[j + 1] = new_val;
        }
    }
    costs[b.len()]
}
