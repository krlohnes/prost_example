use std::io::Result;
use std::path::PathBuf;
use walkdir::WalkDir;
fn main() -> Result<()> {
    let protos: Vec<PathBuf> = WalkDir::new("protos")
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.into_path())
        .collect();
    prost_build::compile_protos(&protos, &["protos"])?;
    Ok(())
}
