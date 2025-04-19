use crate::app::{MyApp, Menu};


use eframe::egui;
use eframe::egui::{RichText};

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
            if ui.button(RichText::new("System Integrity Protection").size(20.0)).clicked() {
                app.selected_menu = Menu::SIP;
            }
        })
    });
}