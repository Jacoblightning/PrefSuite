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
use std::collections::HashSet;
use log::{debug, error, log_enabled, info, trace, Level};
use eframe::egui;
use eframe::egui::RichText;

#[cfg(target_os = "macos")]
fn get_nearby_bluetooth() -> Result<HashSet<String>, String> {
    use objc2_io_bluetooth::IOBluetoothDevice;
    info!("Scanning for bluetooth devices");

    let inquiry = unsafe { objc2_io_bluetooth::IOBluetoothDeviceInquiry::new() };

    trace!("Aquired IOBluetoothDeviceInquiry");
    //inquiry.setInquiryLength()
    //inquiry.setUpdateNewDeviceNames()
    unsafe {
        inquiry.start();
    }

    // TODO: Temp
    std::thread::sleep(std::time::Duration::from_secs(11));
    let devices = match unsafe { inquiry.foundDevices() } {
        Some(devices) => devices,
        None => {
            error!("Error unwrapping found devices!");
            return Err("Error unwrapping found devices!".into());
        }
    };

    info!("Found {} bluetooth devices.", devices.len());

    let mut device_names = HashSet::new();

    for item in devices {
        // See https://github.com/madsmtm/objc2/issues/743
        let device = item.downcast::<IOBluetoothDevice>().unwrap();

        let devname = unsafe { device.name() };

        debug!("Found Bluetooth device: {}", devname);

        let name = devname.to_string();

        device_names.insert(name);
    }

    Ok(device_names)
}

#[cfg(not(target_os = "macos"))]
fn get_nearby_bluetooth() -> Result<HashSet<String>, String> {
    Err("Bluetooth is not supported on this system.".into())
}

pub fn main(app: &mut MyApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        if ui.button(RichText::new("Back")).clicked() {
            app.selected_menu = Menu::Main;
        }
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.label(RichText::new("Bluetooth Menu:").size(36.0));
        });

        if ui.button("Scan").clicked() {
            for item in get_nearby_bluetooth().unwrap() {
                ui.label(item);
            }
        }
    });
}
