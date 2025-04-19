use crate::app::{MyApp, Menu};


use eframe::egui;
use eframe::egui::{RichText};

use std::process::Command;
use rusqlite::fallible_iterator::FallibleIterator;

fn get_wifi_name() -> String {
    let binding = os_info::get();
    let os_version = binding.version();

    if false && os_version < &os_info::Version::Semantic(15, 0, 0) {
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

        let before = network.find(" SSID").unwrap();
        let after = network.find("Security").unwrap();

        let netconn = network[before..after].to_string();

        netconn
    }
}

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
    });
}