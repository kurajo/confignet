use serde::Deserialize;
use anyhow::Result;
use std::{fs::File, path::Path, io::BufReader};

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigRecord {
    pub file_name: String,
    pub mime_label: String,
    pub config_type: String,
}

#[derive(Debug, Clone)]
pub struct ClassifiedResult {
    pub file_name: String,
    pub file_path: String,
    pub is_ci_cd: bool,
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

    pub fn classify<P: AsRef<Path>>(
        &self,
        file_path: P,
        mime_label: &str,
    ) -> Option<ClassifiedResult> {
        let file_path = file_path.as_ref();
        let file_name = file_path.file_name()?.to_string_lossy().to_string();

        self.records
            .iter()
            .filter(|r| r.mime_label == mime_label)
            .min_by_key(|r| levenshtein(&r.file_name, &file_name))
            .map(|r| {
                let normalized_path = normalize_path(file_path);
                let is_ci_cd = r.config_type != "non_config";
                ClassifiedResult {
                    file_name: r.file_name.clone(),
                    file_path: normalized_path,
                    is_ci_cd,
                }
            })
    }
}

fn normalize_path(path: &Path) -> String {
    let cwd = std::env::current_dir().unwrap_or_default();
    match path.strip_prefix(&cwd) {
        Ok(p) => {
            let path_str = p.to_string_lossy();
            if path_str.contains('/') {
                format!("./{}", path_str)
            } else {
                format!("./{}", path_str)
            }
        }
        Err(_) => path.to_string_lossy().to_string(),
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
