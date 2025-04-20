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

use crate::app::password as egui_password;
use crate::{command_output, run_command};
use crate::app::{Menu, MyApp};
use std::collections::HashSet;
use std::path::PathBuf;

use eframe::egui;
use eframe::egui::RichText;

use std::process::Command;
use std::str::Split;
#[derive(Default)]
pub struct WifiData {
    // Currently Selected Network
    selected_network: String,
    // Available networks cache
    network_cache: Option<Result<HashSet<String>, String>>,
    // Password input progress
    password: String,
}

fn is_wifi_on() -> Result<bool, String> {
    let comm: String = command_output!("networksetup", "-getairportpower", "en0");

    let part: Vec<&str> = comm.split(':').collect();
    let part = part[1].trim();

    if part == "On" {
        Ok(true)
    } else if part == "Off" {
        Ok(false)
    } else {
        Err(format!("d{}", "part.to_string()"))
    }
}

fn set_wifi(on: bool) -> Result<(), String> {
    match run_command!("networksetup", "-setairportpower", "Wi-Fi", if on { "On" } else { "Off" }).wait() {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string())
    }
}

fn get_wifi_name() -> Result<String, String> {
    let binding = os_info::get();
    let os_version = binding.version();

    if os_version < &os_info::Version::Semantic(15, 0, 0) {
        let network = command_output!("networksetup", "-getairportnetwork", "en0");

        if network == "You are not associated with an AirPort network.\n" {
            return Ok("Not connected".into());
        }

        let network = network
            .strip_prefix("Current Wi-Fi Network: ")
            .unwrap()
            .strip_suffix("\n")
            .unwrap();

        Ok(network.into())
    } else {
        // Sequoia very graciously decided to remove that command in favour of one that can take up to ~100x as long
        let network = command_output!("ipconfig", "getsummary", "en0");

        if network.contains("Active : FALSE") {
            return Ok("Not connected".into());
        }

        if let Some(before) = network.find(" SSID") {
            if let Some(after) = network.find("Security") {
                let netconn = network[before+8..after].to_string().trim().to_string();

                Ok(netconn)
            } else {
                Err("\"Security\" not in command output".into())
            }
        } else {
            Err("\"SSID\" not in command output".into())
        }
    }
}

#[cfg(target_os = "macos")]
fn get_available_networks_ffi() -> Result<HashSet<String>, String> {
    let interface_wrapped = unsafe {
        objc2_core_wlan::CWWiFiClient::new().interface()
    };

    let interface = match interface_wrapped {
        Some(interface_) => interface_,
        None => return Err("No interface found".into())
    };

    let scan_result_wrapped = unsafe {
        interface.scanForNetworksWithSSID_error(None)
    };


    let scan_result = match scan_result_wrapped {
        Ok(scan_result_) => scan_result_,
        Err(e) => return Err(format!("Scan error: {}", e.to_string()))
    };

    println!("Got {} networks from scan", scan_result.len());

    let mut networks = HashSet::new();

    for network in scan_result {
        match unsafe {network.ssid()} {
            Some(ssid) => {
                networks.insert(ssid.to_string());
            }
            None => return Err("Error getting network SSID".to_string())
        }
    }

    Ok(networks)
}
#[cfg(not(target_os = "macos"))]
fn get_available_networks_ffi() -> Result<HashSet<String>, String> {panic!("This should never be run!");}

fn get_available_networks() -> Result<HashSet<String>, String> {
    let airport = PathBuf::from(
        "/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport",
    );

    if !airport.exists() {
        return if cfg!(target_os = "macos") {
            // Try using the ffi interface as a last resort (very temperamental)
            get_available_networks_ffi()
        } else {
            Err(
                "Sadly, Apple has discontinued the tool that we use to scan for wifi networks :("
                    .into(),
            )
        }
    }

    let comm = String::from_utf8(Command::new(airport).arg("-s").output().unwrap().stdout).unwrap();

    let mut raw_networks: Split<&str> = comm.split("\n");
    //        .map(|s| s.split("                   ").next().unwrap_or_default().to_owned()).collect();
    let header = raw_networks.next().unwrap();

    if let Some(netend) = header.find("BSSID") {
        let mut networks: HashSet<String> = HashSet::new();

        for network in raw_networks {
            if network.len() > netend {
                let realname = &network[..netend];
                networks.insert(realname.trim().into());
            }
        }

        Ok(networks)
    } else if cfg!(target_os = "macos") {
        // Try using the ffi interface as a last resort (very temperamental)
        get_available_networks_ffi()
    } else {
        Err(
            "Sadly, Apple has discontinued the tool that we use to scan for wifi networks :("
                .into(),
        )
    }
}

fn join_network(ssid: &str, network_password: &str) -> Result<(), String> {
    match Command::new("networksetup")
        .arg("-setairportnetwork")
        .arg("en0")
        .arg(ssid)
        .arg(network_password)
        .spawn() {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string())
    }
}

// TODO: Use threads so UI keeps responding

pub fn main(app: &mut MyApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        if ui.button(RichText::new("Back")).clicked() {
            app.selected_menu = Menu::Main;
        }
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.label(RichText::new("Wi-Fi Menu:").size(36.0));
        });


        let mut connected = false;
        ui.horizontal(|ui| {
            let errored: bool;
            ui.label(RichText::new(format!("Wi-Fi is {}", match is_wifi_on() {
                Ok(o) => {
                    errored = false;
                    if o {
                        connected = true;
                        "On".to_string()
                    } else {
                        "Off".to_string()
                    }
                },
                Err(e) => {
                    errored = true;
                    format!("Unknown\nError: {}", e.to_string())
                }
            })).size(24.0));

            if !errored {
                if ui.button(RichText::new(format!("Turn {}", if connected {"Off"} else {"On"}))).clicked() {
                    match set_wifi(!connected) {
                        Ok(_) => (),
                        Err(e) => {
                            rfd::MessageDialog::new()
                                .set_title(format!("Error turning Wi-Fi {}", if connected {"Off"} else {"On"}))
                                .set_description(format!("There was an error turning Wi-Fi {}:\n{}", if connected {"Off"} else {"On"}, e.to_string()))
                                .set_buttons(rfd::MessageButtons::Ok)
                                .set_level(rfd::MessageLevel::Error)
                                .show();
                        }
                    }
                }
            }
        });

        if connected {
            ui.label(RichText::new("You are currently connected to:").heading());
            ui.label(get_wifi_name().unwrap_or_else(|e| format!("\nError: {}", e.to_string())));
            ui.add_space(10.0);

            if app.wifi_data.network_cache.is_none() {
                // TODO: Consider Just using an Option around a HashSet. (No Result)
                app.wifi_data.network_cache = Some(get_available_networks())
            }

            //Dropdown of available networks
            match app.wifi_data.network_cache.as_ref().unwrap() {
                Ok(networks) => {
                    egui::ComboBox::from_label("Available Networks")
                        .selected_text(&app.wifi_data.selected_network)
                        .show_ui(ui, |ui| {
                            for network in networks {
                                if ui
                                    .selectable_label(
                                        &app.wifi_data.selected_network == network,
                                        network,
                                    )
                                    .clicked()
                                {
                                    app.wifi_data.selected_network = network.clone();
                                }
                            }
                        });

                    if ui.button("Re-Scan").clicked() {
                        app.wifi_data.network_cache = None
                    }
                }
                Err(e) => {
                    ui.label(RichText::new(format!("Error: {e}")).heading());
                }
            }

            if !app.wifi_data.selected_network.is_empty() {
                ui.add_space(10.0);

                ui.add(egui_password::password(&mut app.wifi_data.password));
                if ui.button("Connect").clicked() {
                    match join_network(&app.wifi_data.selected_network, &app.wifi_data.password) {
                        Ok(_) => {}
                        Err(e) => {
                            rfd::MessageDialog::new()
                                .set_title("Error Connecting to Network")
                                .set_description(format!("There was an error connecting to {}:\n{}", app.wifi_data.selected_network, e))
                                .set_buttons(rfd::MessageButtons::Ok)
                                .set_level(rfd::MessageLevel::Error)
                                .show();
                        }
                    }
                }
            }
        }
    });
}
