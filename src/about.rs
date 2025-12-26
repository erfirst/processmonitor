use crate::MyApp;
use egui::{RichText, FontId};

impl MyApp {
    pub fn about(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(RichText::new("System Monitor").font(FontId::proportional(40.0)));
            ui.label(RichText::new("By Ethan First").font(FontId::proportional(20.0)));
        });
    }
}
