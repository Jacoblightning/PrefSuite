mod menus;

pub mod password;

use eframe::egui;
use eframe::egui::RichText;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};
use crate::app::menus::sound::SoundData;
use crate::app::menus::wallpaper::WallpaperData;
use crate::app::menus::wifi::WifiData;

#[derive(Display, EnumIter)]
#[derive(Default)]
enum Menu {
    #[default]
    Main,
    WiFi,
    Bluetooth,
    Wallpaper,
    Sound,
    SIP,
    About
}
#[derive(Default)]
pub struct MyApp {
    selected_menu: Menu,
    wallpaper_data: WallpaperData,
    sound_data: SoundData,
    wifi_data: WifiData,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        std::process::exit(0);
                    }
                });
                ui.menu_button("Settings", |ui| {
                    for menu in Menu::iter() {
                        if ui.button(menu.to_string() + " Menu").clicked() {
                            self.selected_menu = menu;
                        }
                    }
                });
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button(RichText::new("About").heading()).clicked() {
                    self.selected_menu = Menu::About;
                }
                ui.centered_and_justified(|ui| {
                    ui.label("© 2025-Present Jacob (https://github.com/jacoblightning)");
                })
            });
        });

        match self.selected_menu {
            Menu::Main => menus::main_menu::main(self, ctx),
            Menu::Wallpaper => menus::wallpaper::main(self, ctx),
            Menu::SIP => menus::sip::main(self, ctx),
            Menu::WiFi => menus::wifi::main(self, ctx),
            Menu::Bluetooth => menus::bluetooth::main(self, ctx),
            Menu::Sound => menus::sound::main(self, ctx),
            Menu::About => menus::about::main(self, ctx),
        }
    }
}