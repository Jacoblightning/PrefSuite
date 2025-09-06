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

use crate::{command_output, run_command};

#[derive(Default)]
pub struct SoundData {
    // The value of the slider
    slider_value: f32,
    // If the value is not out of date (because bool defaults to false)
    reload_not_needed: bool,
    // The save volume
    last_volume: u8,
}

/// VERY expensive function. Do NOT call unless required
fn get_volume() -> u8 {
    command_output!("osascript", "-e", "output volume of (get volume settings)")
        .strip_suffix("\n")
        .unwrap()
        .parse::<u8>()
        .unwrap()
}

/// VERY expensive function. Do NOT call unless required
fn set_volume(volume: u8) {
    run_command!("osascript", "-e", format!("set volume output volume {volume}"))
}

pub fn main(app: &mut MyApp, ctx: &egui::Context) {
    if !app.sound_data.reload_not_needed {
        app.sound_data.last_volume = get_volume();
        app.sound_data.reload_not_needed = true;
    }

    egui::CentralPanel::default().show(ctx, |ui| {
        let spacing = &ui.style().spacing;
        let size = [spacing.slider_width, spacing.slider_rail_height];

        if ui.button(RichText::new("Back")).clicked() {
            app.selected_menu = Menu::Main;
        }
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.label(RichText::new("Sound Menu:").size(36.0));

            ui.label(format!(
                "The volume is currently: {}%",
                app.sound_data.last_volume
            ));
            if ui.button("Reload").clicked() {
                app.sound_data.reload_not_needed = false;
            }

            ui.add_sized(
                size,
                egui::Slider::new(&mut app.sound_data.slider_value, 0.0..=100.0).text("New Volume"),
            );

            if ui.button("Apply").clicked() {
                set_volume(app.sound_data.slider_value as u8);
                app.sound_data.reload_not_needed = false;
            }
        });
    });
}
