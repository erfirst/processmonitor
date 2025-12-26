use crate::MyApp;
use egui::ScrollArea;
use egui_extras::{Column, TableBuilder};
use egui_plot::{Line, Plot, PlotPoints};

impl MyApp {
    pub fn showhome(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            let cpu_points: PlotPoints = self
                .graph_data
                .iter()
                .enumerate()
                .map(|(idx, i)| [idx as f64, i.cpu_usage as f64])
                .collect();

            let mem_points: PlotPoints = self
                .graph_data
                .iter()
                .enumerate()
                .map(|(idx, i)| [idx as f64, i.mem_usage as f64])
                .collect();

            ui.columns(2, |columns| {
                columns[0].vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("CPU Usage");
                        ui.label(format!("{:.2}%", self.sys.global_cpu_usage()));
                    });
                    Plot::new("CPU Usage")
                        .x_axis_label("Time")
                        .y_axis_label("Percent")
                        .view_aspect(2.0)
                        .include_y(0.0)
                        .allow_drag(false)
                        .allow_zoom(false)
                        .allow_scroll(false)
                        .include_y(100.0)
                        .show(ui, |plot_ui| {
                            // Add a line to the plot
                            plot_ui.line(Line::new(cpu_points));
                        });
                });
                columns[1].vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Memory Usage");
                        ui.label(format!(
                            "{:.2}%",
                            (self.sys.used_memory() as f64 / self.sys.total_memory() as f64)
                                * 100.0
                        ));
                    });
                    Plot::new("Mem Usage")
                        .x_axis_label("Time")
                        .y_axis_label("Percent")
                        .view_aspect(2.0)
                        .allow_drag(false)
                        .allow_zoom(false)
                        .allow_scroll(false)
                        .include_y(0.0)
                        .include_y(100.0)
                        .show(ui, |plot_ui| {
                            // Add a line to the plot
                            plot_ui.line(Line::new(mem_points));
                        });
                });
            });
        });
        // Central panel for process list
        egui::CentralPanel::default().show(ctx, |ui| {
            // ---- Heading at the top ----
            ui.heading("Process List");

            // ---- Search row ----
            ui.horizontal(|ui| {
                ui.label("Process Search:");
                ui.text_edit_singleline(&mut self.search_query);
                egui::ComboBox::from_label("")
                    .selected_text("Sort By")
                    .show_ui(ui, |ui| {
                        if ui.selectable_label(false, "CPU").clicked() {
                            self.processes.refresh_from_sysinfo(&mut self.sys);
                            self.processes.sort_by_cpu();
                        }
                        if ui.selectable_label(false, "Memory").clicked() {
                            self.processes.refresh_from_sysinfo(&mut self.sys);
                            self.processes.sort_by_mem();
                        }
                        if ui.selectable_label(false, "PID").clicked() {
                            self.processes.refresh_from_sysinfo(&mut self.sys);
                            self.processes.sort_by_pid();
                        }
                        if ui.selectable_label(false, "Name").clicked() {
                            self.processes.refresh_from_sysinfo(&mut self.sys);
                            self.processes.sort_by_name();
                        }
                        if ui.selectable_label(false, "Disk Read").clicked() {
                            self.processes.refresh_from_sysinfo(&mut self.sys);
                            self.processes.sort_by_disk_read();
                        }
                        if ui.selectable_label(false, "Disk Write").clicked() {
                            self.processes.refresh_from_sysinfo(&mut self.sys);
                            self.processes.sort_by_disk_write();
                        }
                    });
            });

            ui.separator();

            // ---- The table ----
            ScrollArea::vertical().show(ui, |ui| {
                TableBuilder::new(ui)
                    .striped(true)
                    .column(Column::auto()) // PID
                    .column(Column::remainder()) // Process Name
                    .column(Column::auto()) // CPU
                    .column(Column::auto()) // Memory
                    .column(Column::auto()) // Disk Read Usage
                    .column(Column::auto())
                    .header(20.0, |mut header| {
                        header.col(|ui| {
                            ui.strong("PID      ");
                        });
                        header.col(|ui| {
                            ui.strong("Process Name");
                        });
                        header.col(|ui| {
                            ui.strong("CPU        ");
                        });
                        header.col(|ui| {
                            ui.strong("Memory");
                        });
                        header.col(|ui| {
                            ui.strong("Disk Read");
                        });
                        header.col(|ui| {
                            ui.strong("Disk Write");
                        });
                    })
                    .body(|mut body| {
                        for i in self.processes.iter() {
                            //turn to map
                            let name = format!("{}", i.name);
                            if self.search_query.is_empty() || name.contains(&self.search_query) {
                                body.row(20.0, |mut row| {
                                    row.col(|ui| {
                                        ui.label(i.pid.clone());
                                    });
                                    row.col(|ui| {
                                        ui.label(&name);
                                    });
                                    row.col(|ui| {
                                        ui.label(format!(
                                            "{:.2}%",
                                            i.cpu / self.sys.cpus().len() as f32
                                        )); //we need to convert to handle multicore processor everything shows as 0 now
                                    });
                                    row.col(|ui| {
                                        ui.label(format!(
                                            "{:.2} MB",
                                            i.mem as f32 / (1024.0 * 1024.0) 
                                        ));
                                    });
                                    row.col(|ui| {
                                        ui.label(format!(
                                            "{:.2} MB/s",
                                            i.disk_usage.read_bytes as f32 / (1024.0 * 1024.0)
                                        ));
                                    });
                                    row.col(|ui| {
                                        ui.label(format!(
                                            "{:.2} MB/s",
                                            i.disk_usage.written_bytes as f32 / (1024.0 * 1024.0)
                                        ));
                                    });
                                });
                            }
                        }
                    });
            });
        });
    }
}
