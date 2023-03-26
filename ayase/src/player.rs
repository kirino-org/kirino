use std::error::Error;

use eframe::egui_wgpu::wgpu;

#[cfg(feature = "desktop")]
extern crate libmpv;
#[cfg(feature = "desktop")]
use libmpv::Mpv;

fn play(file: String) -> Result<(), Box<dyn Error>> {
    Mpv::new();
}

