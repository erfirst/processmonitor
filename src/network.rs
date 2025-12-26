use crate::MyApp;

use sysinfo::{Networks};

impl MyApp {
    pub fn network(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Network Stats");
            let networks = Networks::new_with_refreshed_list();
            networks.iter().for_each(|network|{
                if network.1.total_packets_transmitted() > 4{
                    ui.label(format!(""));
                }
            });
       });
    }
}
