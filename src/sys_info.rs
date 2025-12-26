use crate::MyApp;
use sysinfo::{Motherboard, System, Disks};

impl MyApp {
    pub fn sysinfo(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let headstr = format!("System Info for {:?}", System::host_name().unwrap());
            ui.heading(headstr);
            let kernalstr = format!(
                "Kernel Version: {:?}",
                System::kernel_long_version().to_owned()
            );
            ui.label(kernalstr);
            if let Some(m) = Motherboard::new() {
                let mbstr = format!("Motherboard: {}  {:?}",m.vendor_name().unwrap(), m.name().unwrap());
                ui.label(mbstr);
            }
            let osstr = format!("OS Version: {:?}", System::long_os_version().unwrap());
            ui.label(osstr);
            let cpustr = format!(
                "CPU Architecture: {} with {:?} physical cores",
                System::cpu_arch().to_string(),
                System::physical_core_count().unwrap()
            );
            ui.label(cpustr);
            let disks = Disks::new_with_refreshed_list();
            let diskstr =  format!("{:?} disk devices found", disks.len());
            ui.label(diskstr);
            for disk in &disks{ //map
                let diskstrunit = format!("Disk {:?} is a {:?} with {:?} GB of space", disk.name(), disk.kind(), (disk.total_space()/ 1073741824));
                ui.label(diskstrunit);
            }
            let uptimestr = format!("Uptime: {:?} minutes", System::uptime() / 60);
            ui.label(uptimestr);
        });
    }
}
