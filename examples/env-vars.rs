use anyhow::Result;

/// Some of the other environment variables Cargo sets for crates
/// https://doc.rust-lang.org/cargo/reference/environment-variables.html
pub fn main() -> Result<()> {
  let manifest_dir = env!("CARGO_MANIFEST_DIR");
  println!("{:?}", manifest_dir);
  Ok(())
}
