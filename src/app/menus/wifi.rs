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
use crate::app::{Menu, MyApp};
use crate::{command_output, run_command, command_output_option};
use std::collections::HashSet;
use std::path::PathBuf;
use log::{debug, error, info, trace};

use eframe::egui;
use eframe::egui::RichText;

use std::str::Split;

struct WifiInfo {
    // Current Network
    current: Option<String>,
    // Available Networks
    nearby: Option<HashSet<String>>,
}

#[derive(Default)]
pub struct WifiData {
    // Wifi info struct
    wifi_info: Option<WifiInfo>,
    // Currently selected network input storage
    selected_network: String,
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
        Err(part.to_string())
    }
}

fn set_wifi(on: bool) -> Result<(), String> {
    match run_command!(
        "networksetup",
        "-setairportpower",
        "en0",
        if on { "On" } else { "Off" }
    )
    .wait()
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}


#[cfg(target_os = "macos")]
fn get_current_wifi_ffi() -> Oprion<String> {
    match unsafe { objc2_core_wlan::CWWiFiClient::sharedWiFiClient().interface() } {
        Some(interface) => match unsafe {interface.ssid()} {
            Some(ssid) => Some(ssid.to_string()),
            None => {
                error!("Could not get current SSID");
                None
            },
        },
        None => {
            error!("No interface found");
            None
        },
    }
}
#[cfg(not(target_os = "macos"))]
fn get_current_wifi_ffi() -> Option<String> {
    panic!("This should never be run!");
}

#[cfg(target_os = "macos")]
fn get_nearby_wifi_ffi() -> Option<HashSet<String>> {
    let interface = match unsafe { objc2_core_wlan::CWWiFiClient::sharedWiFiClient().interface() } {
        Some(interface) => interface,
        None => {
            error!("No interface found");
            return None;
        },
    };

    let scan_result = match unsafe { interface.scanForNetworksWithSSID_error(None) } {
        Ok(scan_result) => scan_result,
        Err(e) => {
            error!("Scan error: {e}");
            return None;
        },
    };

    debug!("Got {} networks from scan", scan_result.len());

    let mut networks = HashSet::new();

    let mut errored = false;

    for network in scan_result {
        match unsafe { network.ssid() } {
            Some(ssid) => {
                let ssid_str = ssid.to_string();
                trace!(" -{}", &ssid_str);
                networks.insert(ssid_str);
            },
            None => {errored = true;},
        }
    }

    if errored && networks.is_empty() {
        error!("Error getting network SSID");
        return None;
    }

    Some(networks)
}
#[cfg(not(target_os = "macos"))]
fn get_nearby_wifi_ffi() -> Option<HashSet<String>> {panic!("This should never be run!");}


fn get_nearby_wifi_airport() -> Option<HashSet<String>> {
    let airport = PathBuf::from(
        "/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport",
    );

    if !airport.exists() {
        return None;
    }

    let comm = command_output_option!(airport, "-s");

    let mut raw_networks: Split<&str> = comm.split("\n");
    let header = raw_networks.next().unwrap();

    if let Some(netend) = header.find("BSSID") {
        let mut networks: HashSet<String> = HashSet::new();

        for network in raw_networks {
            if network.len() > netend {
                let realname = &network[..netend];
                networks.insert(realname.trim().into());
            }
        }
        return Some(networks);
    }
    None
}

fn get_nearby_wifi_heuristic() -> Option<HashSet<String>> {
    info!("Initializing nearby wifi heuristic!");

    
    info!("Trying airport method.");
    let nets = get_nearby_wifi_airport();
    if nets.is_some() {
        info!("Airport method worked!");
        return nets;
    }
    info!("Airport method failed :(");


    info!("Trying FFI method.");
    let nets = get_nearby_wifi_ffi();
    if nets.is_some() {
        info!("FFI method worked!");
        return nets;
    }
    info!("FFI method failed :(");


    error!("All methods failed :(");
    None
}

fn get_current_wifi_networksetup(os_version: &os_info::Version) -> Option<String> {
    if os_version < &os_info::Version::Semantic(15, 0, 0) {
        let network = command_output_option!("networksetup", "-getairportnetwork", "en0");

        if network == "You are not associated with an AirPort network.\n" {
            return Some("Not connected".into());
        }

        let network = network
            .strip_prefix("Current Wi-Fi Network: ")
            .unwrap()
            .strip_suffix("\n")
            .unwrap();

        Some(network.into())
    } else {
        None
    }
}

fn get_current_wifi_heuristic(os_version: &os_info::Version) -> Option<String> {
    info!("Initializing current wifi heuristic!");


    info!("Trying networksetup method.");
    let net = get_current_wifi_networksetup(os_version);
    if net.is_some() {
        info!("Networksetup method worked!");
        return net;
    }
    info!("Networksetup method failed :(");


    info!("Trying FFI method.");
    let net = get_current_wifi_ffi();
    if net.is_some() {
        info!("FFI method worked!");
        return net;
    }
    info!("FFI method failed :(");


    error!("All methods failed :(");
    None
}

fn get_wifi_info_heuristic() -> Option<WifiInfo> {
    info!("Initializing wifi info heuristic!");
    let binding = os_info::get();
    let os_version = binding.version();
    info!("OS version: {os_version}");

    let mut nearby = get_nearby_wifi_heuristic();
    let mut current = get_current_wifi_heuristic(os_version);

    if nearby.is_none() || current.is_none() {
        // Reliable method (at least currently) but SLOOOOOOW!!!
        let mut wifi_info_json = json::parse(
            &command_output_option!("system_profiler", "-json", "SPAirPortDataType")
        ).unwrap();

        if current.is_none() {
            current = Some(wifi_info_json["SPAirPortDataType"][0]["spairport_airport_interfaces"][0]["spairport_current_network_information"]["_name"].take_string().unwrap());
        }
        if nearby.is_none() {
            let mut nearby_new = HashSet::new();
            
            for member in wifi_info_json["SPAirPortDataType"][0]["spairport_airport_interfaces"][0]["spairport_airport_other_local_wireless_networks"].members_mut() {
                nearby_new.insert(
                    member["_name"].take_string().unwrap()
                );
            }

            nearby = Some(nearby_new);
        }
    }

    Some(WifiInfo {
        nearby,
        current
    })
}

fn join_network(ssid: &str, network_password: &str) -> Result<(), String> {
    run_command!(
        "networksetup",
        "-setairportnetwork",
        "en0",
        ssid,
        network_password
    );
    Ok(())
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
            ui.label(
                RichText::new(format!(
                    "Wi-Fi is {}",
                    match is_wifi_on() {
                        Ok(o) => {
                            errored = false;
                            if o {
                                connected = true;
                                "On".to_string()
                            } else {
                                "Off".to_string()
                            }
                        }
                        Err(e) => {
                            errored = true;
                            format!("Unknown\nError: {e}")
                        }
                    }
                ))
                .size(24.0),
            );

            if !errored
                && ui
                    .button(RichText::new(format!(
                        "Turn {}",
                        if connected { "Off" } else { "On" }
                    )))
                    .clicked()
            {
                match set_wifi(!connected) {
                    Ok(_) => (),
                    Err(e) => {
                        rfd::MessageDialog::new()
                            .set_title(format!(
                                "Error turning Wi-Fi {}",
                                if connected { "Off" } else { "On" }
                            ))
                            .set_description(format!(
                                "There was an error turning Wi-Fi {}:\n{e}",
                                if connected { "Off" } else { "On" }
                            ))
                            .set_buttons(rfd::MessageButtons::Ok)
                            .set_level(rfd::MessageLevel::Error)
                            .show();
                    }
                }
            }
        });

        if connected {
            if app.wifi_data.wifi_info.is_none() {
                // TODO: Consider Just using an Option around a HashSet. (No Result)
                app.wifi_data.wifi_info = get_wifi_info_heuristic()
            }
            
            ui.label(RichText::new("You are currently connected to:").heading());
            let errmsg = "Error! Please check the logs.";
            ui.label(if let Some(curr) = app.wifi_data.wifi_info.as_ref().unwrap().current.as_ref() {
                curr
            } else {
                errmsg
            });
            ui.add_space(10.0);

            //Dropdown of available networks
            match app.wifi_data.wifi_info.as_ref().unwrap().nearby.as_ref() {
                Some(networks) => {
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
                        app.wifi_data.wifi_info = None
                    }
                }
                None => {
                    ui.label(RichText::new("Error! Please check the logs.").heading());
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
                                .set_description(format!(
                                    "There was an error connecting to {}:\n{}",
                                    app.wifi_data.selected_network, e
                                ))
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
