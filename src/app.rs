mod menus;

use eframe::egui;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};
use crate::app::menus::wallpaper::WallpaperData;

#[derive(Display, EnumIter)]
enum Menu {
    Main,
    Wallpaper
}

impl Default for Menu {
    fn default() -> Self { Menu::Main }
}

#[derive(Default)]
pub struct MyApp {
    selected_menu: Menu,
    wallpaper_data: WallpaperData,
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

        match self.selected_menu {
            Menu::Main => {
                menus::main_menu::main(self, ctx);
            },
            Menu::Wallpaper => {
                menus::wallpaper::main(self, ctx);
            }
        }
    }
}