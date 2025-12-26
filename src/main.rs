mod about;
mod cpupage;
mod diskpage;
mod graphdata;
mod homepage;
mod mempage;
mod network;
mod process_list;
mod settings;
mod sys_info;

use process_list::{ProcessInfo, ProcessList, SortField};
use settings::SettingsWindow;
use std::collections::HashMap;
use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

use crate::graphdata::{GraphDataList, GraphDataPoint};

fn main() -> eframe::Result<()> {
  // Load image
    let image = image::open("assets/icon.png")
        .expect("Failed to load icon")
        .into_rgba8();

    let (width, height) = image.dimensions();
    let pixels = image.into_raw();

    let icon = egui::IconData {
        rgba: pixels,
        width,
        height,
    };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_icon(icon),
        ..Default::default()
    };
    

    // Note the closure returns Ok(Box<dyn App>) instead of Box<dyn App>
    eframe::run_native(
        "System Monitor",
        options,
        Box::new(
            |_cc| -> Result<Box<dyn eframe::App>, Box<dyn std::error::Error + Send + Sync>> {
                Ok(Box::new(MyApp::default()))
            },
        ),
    )
}

pub struct MyApp {
    processes: ProcessList,
    search_query: String,
    last_refresh: Instant,
    sys: sysinfo::System,
    graph_data: GraphDataList,
    settings_window: SettingsWindow,
    pub cpu_features: Vec<(&'static str, bool)>,
    current_page: Page,
}

pub enum Page {
    Home,
    CPU,
    Memory,
    Disk,
    SysInfo,
    About,
    Network,
}

//entry function for app
impl Default for MyApp {
    fn default() -> Self {
        if sysinfo::IS_SUPPORTED_SYSTEM {
            println!("This OS is supported!");
        } else {
            println!("This OS isn't supported (yet?).");
            std::process::exit(1);
        }
        let mut sys = sysinfo::System::new_all();
        sys.refresh_all();
        let first_read = GraphDataPoint {
            cpu_usage: (sys.global_cpu_usage()),
            mem_usage: (sys.used_memory() as f64 / sys.total_memory() as f64) * 100.0,
        };
        let mut datavec = VecDeque::new();
        datavec.push_back(first_read);
        let graphdata = GraphDataList {
            graph_data: datavec,
        };
        let mut processes = ProcessList::new();
        processes.refresh_from_sysinfo(&mut sys);
        let cpu_features  = cpupage::detect_cpu_features();

        MyApp {
            processes,
            search_query: String::new(),
            last_refresh: Instant::now(),
            sys,
            graph_data: graphdata,
            settings_window: SettingsWindow::default(),
            cpu_features,
            current_page: Page::Home,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top panel for CPU and Memory usage with graphs

        if self.last_refresh.elapsed() > Duration::from_millis(500) {
            self.sys.refresh_all();
            self.graph_data.refresh_from_sysinfo(&mut self.sys);
            self.processes.refresh_from_sysinfo(&mut self.sys);
            self.last_refresh = Instant::now();

            if self.settings_window.collapse {
                let mut merged: HashMap<String, ProcessInfo> = HashMap::new();

                for p in &self.processes.items {
                    //turn to map
                    let entry = merged.entry(p.name.clone()).or_insert(ProcessInfo {
                        pid: p.pid.clone(),
                        name: p.name.clone(),
                        cpu: 0.0,
                        mem: 0,
                        disk_usage: sysinfo::DiskUsage::default(),
                    });

                    entry.cpu += p.cpu;
                    entry.mem += p.mem;
                    entry.disk_usage.total_written_bytes += p.disk_usage.total_written_bytes;
                    entry.disk_usage.written_bytes += p.disk_usage.written_bytes;
                    entry.disk_usage.total_read_bytes += p.disk_usage.total_read_bytes;
                    entry.disk_usage.read_bytes += p.disk_usage.read_bytes;
                }

                // replace the full list with the collapsed one
                self.processes.items = merged.into_values().collect();
                match &self.processes.sortdir {
                    SortField::Default => self.processes.sort_by_pid(),
                    SortField::Cpu => self.processes.sort_by_cpu(),
                    SortField::Mem => self.processes.sort_by_mem(),
                    SortField::Write => self.processes.sort_by_disk_write(),
                    SortField::Pid => self.processes.sort_by_pid(),
                    SortField::Read => self.processes.sort_by_disk_read(),
                    SortField::Name => self.processes.sort_by_name(),
                }
            }
        }
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    ui.separator();
                    if ui.button("Settings").clicked() {
                        self.settings_window.open = true;
                    }
                    if ui.button("System Info").clicked() {
                        self.current_page = Page::SysInfo;
                    }
                    if ui.button("About").clicked() {
                        self.current_page = Page::About;
                    }
                    if ui.button("Quit").clicked() {
                        // Handle quitting
                        std::process::exit(0);
                    }
                });
                if ui.button("Home").clicked() {
                    self.current_page = Page::Home;
                }
                if ui.button("CPU").clicked() {
                    self.current_page = Page::CPU;
                }
                if ui.button("Memory").clicked() {
                    self.current_page = Page::Memory;
                }
                if ui.button("Disk").clicked() {
                    self.current_page = Page::Disk;
                }
                //Network doesnt work on all platforms
                /*  if ui.button("Network").clicked() {
                    self.current_page = Page::Network;
                }*/
            });
        }); //options pannel
        match self.current_page {
            Page::Home => self.showhome(ctx),
            Page::CPU => self.showcpu(ctx),
            Page::Disk => self.showdisk(ctx),
            Page::Memory => self.showmem(ctx),
            Page::SysInfo => self.sysinfo(ctx),
            Page::About => self.about(ctx),
            Page::Network => self.network(ctx),
        }
        self.settings_window.show(ctx); //show settings window if open

        // If user requested refresh of CPU features, recompute once and clear the flag
        if self.settings_window.refresh_cpufeatures {
            self.cpu_features = crate::cpupage::detect_cpu_features();
            self.settings_window.refresh_cpufeatures = false;
        }
    }
}