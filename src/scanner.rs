use std::{collections::HashMap, fs, io::Result, path::Path};

pub fn visit_dirs_and_count(dir: &str) -> Result<HashMap<String, usize>> {
    let mut counts = HashMap::new();
    visit_dirs(Path::new(dir), &mut counts)?;
    Ok(counts)
}

fn visit_dirs(dir: &Path, counts: &mut HashMap<String, usize>) -> Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                visit_dirs(&path, counts)?;
            } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                let ext = ext.to_lowercase();
                *counts.entry(ext).or_insert(0) += 1;
            }
        }
    }
    Ok(())
}
