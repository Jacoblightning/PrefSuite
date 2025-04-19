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

pub fn main(app: &mut MyApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.label(RichText::new("Main Menu").size(36.0));
        });
        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
            if ui.button(RichText::new("Wi-Fi").size(20.0)).clicked() {
                app.selected_menu = Menu::WiFi;
            }
            if ui.button(RichText::new("Bluetooth").size(20.0)).clicked() {
                app.selected_menu = Menu::Bluetooth;
            }
            if ui.button(RichText::new("Wallpaper").size(20.0)).clicked() {
                app.selected_menu = Menu::Wallpaper;
            }
            if ui.button(RichText::new("Sound").size(20.0)).clicked() {
                app.selected_menu = Menu::Sound;
            }
            if ui
                .button(RichText::new("System Integrity Protection").size(20.0))
                .clicked()
            {
                app.selected_menu = Menu::Sip;
            }
        })
    });
}
