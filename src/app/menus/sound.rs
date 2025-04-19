use crate::app::{MyApp, Menu};


use eframe::egui;
use eframe::egui::{RichText};

use std::process::Command;

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
    String::from_utf8(Command::new("osascript")
        .arg("-e")
        .arg("output volume of (get volume settings)")
        .output()
        .unwrap()
        .stdout)
        .unwrap()
        .strip_suffix("\n")
        .unwrap()
        .parse::<u8>()
        .unwrap()
}

/// VERY expensive function. Do NOT call unless required
fn set_volume(volume: u8) {
    Command::new("osascript")
        .arg("-e")
        .arg(format!("set volume output volume {volume}"))
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

pub fn main(app: &mut MyApp, ctx: &egui::Context) {
    if !app.sound_data.reload_not_needed {
        //app.sound_data.last_volume = get_volume();
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

            ui.label(format!("The volume is currently: {}%", app.sound_data.last_volume));
            if ui.button("Reload").clicked() {
                app.sound_data.reload_not_needed = false;
            }


            ui.add_sized(
                size,
                egui::Slider::new(&mut app.sound_data.slider_value, 0.0..=100.0).text("New Volume")
            );

            if ui.button("Apply").clicked() {
                set_volume(app.sound_data.slider_value as u8);
                app.sound_data.reload_not_needed = false;
            }
        });
    });
}