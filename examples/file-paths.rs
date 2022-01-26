use std::path::PathBuf;

#[cfg(not(target_arch = "wasm32"))]
pub fn svm_home() -> Option<PathBuf> {
    home::home_dir().map(|dir| dir.join(".svm"))
}

pub fn svm_home_global_version() -> Option<PathBuf> {
    svm_home().map(|p| p.join(".global-version"))
}

fn main() -> anyhow::Result<()> {
    if let Some(path) = svm_home_global_version() {
        println!("{:?}", path);
    }
    Ok(())
}
