use ignore::WalkBuilder;
use std::path::PathBuf;

pub fn get_files_to_scan(dir: &str) -> Vec<PathBuf> {
    let walker = WalkBuilder::new(dir).build();
    
    walker.filter_map(|result| result.ok())
          .filter(|entry| {
              let is_file = entry.file_type().map_or(false, |ft| ft.is_file());
              if !is_file {
                  return false;
              }
              
              if let Some(ext) = entry.path().extension() {
                  let ext_str = ext.to_string_lossy();
                  matches!(ext_str.as_ref(), "js" | "ts" | "tsx" | "jsx" | "py")
              } else {
                  false
              }
          })
          .map(|entry| entry.into_path())
          .collect()
}