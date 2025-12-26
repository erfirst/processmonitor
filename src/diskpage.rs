use crate::MyApp;
use sysinfo::Disks;

impl MyApp {
    pub fn showdisk(&mut self, ctx: &egui::Context) {
        let disks = Disks::new_with_refreshed_list();
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Disk Usage");
            disks.iter().enumerate().for_each(|idx| {
                /*  let mut removestr = "";
                if idx.1.is_removable() {
                    removestr = "Removable";
                } else {
                    removestr = "Nonremovable";
                }*/
                ui.horizontal(|ui| {
                    ui.label(format!(
                        //     "{} {} drive {:?} {}\nFormat: {}\n{}GB used of {}GB total",
                        //   removestr,
                        "{} drive {:?} {}\nFormat: {}\n{}GB used of {}GB total",
                        idx.1.mount_point().to_string_lossy(),
                        idx.1.kind(),
                        idx.1.name().to_string_lossy(),
                        idx.1.file_system().to_string_lossy(),
                        idx.1.available_space() / (1024 * 1024 * 1024),
                        idx.1.total_space() / (1024 * 1024 * 1024),
                    ))
                });
                ui.add_space(20.0);
            });
        });
    }
}
