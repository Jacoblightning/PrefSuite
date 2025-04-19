use crate::app::{MyApp, Menu};


use eframe::egui;
use eframe::egui::{RichText};

use std::process::Command;

#[derive(Default)]
pub struct SoundData {
}


fn get_volume() -> u8 {
    String::from_utf8(Command::new("osascript")
        .arg("-e")
        .arg("output volume of (get volume settings)")
        .output()
        .unwrap()
        .stdout)
        .unwrap()
        .strip_suffix("\n")
        .parse::<u8>()
        .unwrap()
}

fn set_volume(volume: u8) {
    Command::new("osascript")
        .arg("-e")
        .arg(format!("set volume output volume {volume}"))
        .spawn()
        .unwrap();
}

pub fn main(app: &mut MyApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        if ui.button(RichText::new("Back")).clicked() {
            app.selected_menu = Menu::Main;
        }
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.label(RichText::new("Sound Menu:").size(36.0));


            ui.label(format!("The volume is currently: {}%", get_volume()));

            let mut new_volume: f32 = 0.0;
            ui.add(egui::Slider::new(&mut new_volume, 0.0..=100.0).text("New Volume"));

            if ui.button("Apply").clicked() {
                set_volume(new_volume as u8)
            }
        });
    });
}