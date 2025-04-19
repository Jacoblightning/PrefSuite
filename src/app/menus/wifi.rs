use std::collections::HashSet;
use std::path::PathBuf;
use crate::app::{MyApp, Menu};
use crate::app::password as egui_password;


use eframe::egui;
use eframe::egui::{RichText};

use std::process::Command;

#[derive(Default)]
pub struct WifiData {
    // Currently Selected Network
    selected_network: String,
    // Available networks cache
    network_cache: Option<Result<HashSet<String>, String>>,
    // Password input progress
    password: String,
}


fn get_wifi_name() -> String {
    let binding = os_info::get();
    let os_version = binding.version();

    if os_version < &os_info::Version::Semantic(15, 0, 0) {
        let network = String::from_utf8(Command::new("networksetup")
            .arg("-getairportnetwork")
            .arg("en0")
            .output()
            .unwrap()
            .stdout)
            .unwrap();

        if network == "You are not associated with an AirPort network.\n" {
            return "Not connected".into()
        }

        let network = network.strip_prefix("Current Wi-Fi Network: ").unwrap()
            .strip_suffix("\n").unwrap();

        network.into()
    } else {
        // Sequoia very graciously decided to remove that command in favour of one that can take up to ~100x as long
        let network = String::from_utf8(Command::new("ipconfig")
            .arg("getsummary")
            .arg("en0")
            .output()
            .unwrap()
            .stdout)
            .unwrap();

        if network.contains("Active : FALSE") {
            return "Not connected".into()
        }

        let before = network.find(" SSID").unwrap() + 8;
        let after = network.find("Security").unwrap();

        let netconn = network[before..after].to_string().trim().to_string();

        netconn
    }
}

fn get_available_networks() -> Result<HashSet<String>, String> {
    let airport = PathBuf::from("/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport");

    if !airport.exists() {
        return Err("Sadly, Apple has discontinued the tool that we use to scan for wifi networks :(".into());
    }

    let comm = String::from_utf8(Command::new(airport)
        .arg("-s")
        .output()
        .unwrap()
        .stdout)
        .unwrap();

    let mut networks: HashSet<String> = comm.split("\n")
        .map(|s| s.split_whitespace().next().unwrap().to_owned()).collect();
    println!("Removed SSID? {}", networks.remove("SSID"));

    Ok(networks)
}

fn join_network(ssid: &str, network_password: &str) {
    Command::new("networksetup")
        .arg("-setairportnetwork")
        .arg("en0")
        .arg(ssid)
        .arg(network_password)
        .spawn()
        .unwrap();
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

        ui.label(RichText::new("You are currently connected to:").heading());
        ui.label(get_wifi_name());
        ui.add_space(10.0);

        if app.wifi_data.network_cache.is_none() {
            // TODO: Consider Just using an Option around a HashSet. (No Result)
            app.wifi_data.network_cache = Some(get_available_networks())
        }

        //Dropdown of available networks
        match app.wifi_data.network_cache.as_ref().unwrap() {
            Ok(networks) => {
                egui::ComboBox::from_label("Available Networks")
                    .show_ui(ui, |ui| {
                        for network in networks {
                            if ui.selectable_label(&app.wifi_data.selected_network == network, network).clicked() {
                                app.wifi_data.selected_network = network.clone();
                            }
                        }
                    });

                if ui.button("Re-Scan").clicked() {
                    app.wifi_data.network_cache  = None
                }
            }
            Err(e) => {ui.label(RichText::new(format!("Error: {e}")).heading());}
        }

        if !app.wifi_data.selected_network.is_empty() {
            ui.add(egui_password::password(&mut app.wifi_data.password));
            if ui.button("Connect").clicked() {
                join_network(&app.wifi_data.selected_network, &app.wifi_data.password)
            }
        }
    });
}