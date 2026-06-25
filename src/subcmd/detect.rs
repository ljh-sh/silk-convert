// detect: identify audio format

use anyhow::Result;
use std::path::Path;
use crate::codec::AudioFormat;

pub fn run(input: &Path) -> Result<()> {
    let fmt = AudioFormat::from_magic(input)?;
    let ext = input.extension().and_then(|e| e.to_str()).unwrap_or("");

    println!("{}: format={} (extension: {})", input.display(), fmt.as_str(), ext);
    Ok(())
}
