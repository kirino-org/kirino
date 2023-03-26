use crate::HOME;
use std::{fs::OpenOptions, io::Write, path::Path};

/// Make sure we're running in a suitable environment, set up anything that need setting up, etc.
///
/// Returns path to the presets file
pub fn init() -> String {
    let cfg_path = format!("{}/.config", HOME.as_str());
    let prs_path = format!("{cfg_path}/presets.chi");

    // Make sure ~/.config (standard user config dir) exists
    if !Path::new(&cfg_path).exists() {
        panic!("'~/.config' does not exist");
    }
    // If presets file doesn't exist, create it and write defaults
    if !Path::new(&prs_path).exists() {
        println!("presets file not found. creating one from defaults.");
        let mut f = OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .open(&prs_path)
            .expect("failed to create default presets file");
        f.write(include_bytes!("../presets.txt"))
            .expect("failed to write to default presets file");
    }

    prs_path
}
