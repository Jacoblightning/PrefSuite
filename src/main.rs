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

mod app;

use app::MyApp;
use eframe::egui;
use egui_extras::install_image_loaders;

#[allow(unused_doc_comments)]
fn main() -> eframe::Result {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };

    eframe::run_native(
        "PrefSuite",
        options,
        /*
        Box::new(|cc| {
            // Image support
            install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
        */
        Box::new(|cc| {
            // Image Support
            install_image_loaders(&cc.egui_ctx);

            Ok(if cfg!(target_os = "macos") {Box::<MyApp>::default()} else {Box::<MacosOnly>::default()})
        })
    )
}

#[derive(Default)]
struct MacosOnly {}

impl eframe::App for MacosOnly {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(
                egui::RichText::new("PrefSuite is only available for MacOS. Sorry :(.").size(36.0),
            );
            if cfg!(target_os = "linux") {
                ui.label(
                    egui::RichText::new("May I recommend ReSet instead for your settings needs?")
                        .size(36.0),
                );
            }
        });
    }
}
