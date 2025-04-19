mod app;

use app::MyApp;
use eframe::egui;

fn main() -> eframe::Result {
    println!("Hello, world!");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };

    eframe::run_native(
        "PrefSuite",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
        //Box::new(|_cc| Ok(if cfg!(target_os = "macos") {Box::<MyApp>::default()} else {Box::<MacosOnly>::default()})),
    )
}

#[derive(Default)]
struct MacosOnly {}

impl eframe::App  for MacosOnly {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(egui::RichText::new("PrefSuite is only available for MacOS. Sorry :(.").size(36.0));
            if cfg!(target_os = "linux") {
                ui.label(egui::RichText::new("May I recommend ReSet instead for your settings needs?").size(36.0));
            }
        });
    }
}