/*
    PrefSuite. A Preferences suite for MacOS
    Copyright (C) 2025-Present Jacob (https://github.com/jacoblightning)

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use crate::app::{Menu, MyApp};

use eframe::egui;
use eframe::egui::RichText;
use log::{debug, error, info, trace};

#[cfg(target_os = "macos")]
fn get_sip() -> Result<u32, String> {

    info!("Loading /usr/lib/libSystem.dylib");

    let lib = match unsafe {libloading::Library::new("/usr/lib/libSystem.dylib")} {
        Ok(lib) => lib,
        Err(e) => {
            error!("Failed to load libSystem.dylib: {}", e);
            return Err(e.to_string())
        },
    };
    trace!("Successfully loaded /usr/lib/libSystem.dylib");
    debug!("Loading function csr_get_active_config");

    let func: libloading::Symbol<unsafe extern fn() -> u32> = match unsafe { lib.get(b"sip_active_config") } {
        Ok(func) => func,
        Err(e) => {
            error!("Failed to load function csr_get_active_config: {}", e);
            return Err(e.to_string())
        }
    };

    trace!("Successfully loaded /usr/lib/libSystem.dylib");

    debug!("Calling sip_active_config function");
    let sip_bits = unsafe {func()};
    trace!("Successfully called sip_active_config function");
    info!("sip bits: {}", sip_bits);

    Ok(sip_bits)
}

pub fn main(app: &mut MyApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        if ui.button(RichText::new("Back")).clicked() {
            app.selected_menu = Menu::Main;
        }
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.label(RichText::new("System Integrity Protection Menu:").size(36.0));
        });

        if ui.button("Call function").clicked(){
            ui.label(match get_sip() {
                Ok(sip) => {sip.to_string()}
                Err(e) => e
            });
        }
    });
}
