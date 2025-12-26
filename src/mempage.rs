use crate::MyApp;
use egui::{RichText, FontId};

impl MyApp {
    pub fn showmem(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Memory Usage");
            ui.columns(2, |columns|{
                columns[0].vertical(|ui|{
                    let totalmemstr = format!("{:?}GB / {:?}GB used",(self.sys.used_memory() / 1073741824).to_owned(), (self.sys.total_memory() / 1073741824).to_owned());
                    ui.label(RichText::new("Total Memory").font(FontId::proportional(20.0)));
                    ui.label(totalmemstr);
                });
                 columns[1].vertical(|ui|{
                    ui.label(RichText::new("Swap Memory").font(FontId::proportional(20.0)));
                    let swapmemstr = format!("{:?}GB / {:?}GB used",(self.sys.used_swap() / 1073741824).to_owned(), (self.sys.total_swap() / 1073741824).to_owned());
                    ui.label(swapmemstr);
                });
            });
        });
    }
}
