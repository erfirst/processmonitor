use crate::{graphdata::GraphDataPoint, MyApp};
use egui::RichText;
use egui_plot::{Line, Plot, PlotPoints};

const MAX_POINTS: usize = 120;

impl MyApp {
    pub fn showcpu(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("CPU Usage");

            let cpus = self.sys.cpus();
            ui.label(format!("Number of CPU cores in CPU: {}", cpus.len()));

            //  Update CPU history ONCE per frame
            for (_i, cpu) in cpus.iter().enumerate() {
                let history = &mut self.graph_data.graph_data;

                history.push_back(GraphDataPoint {
                    cpu_usage: cpu.cpu_usage() as f32,
                    mem_usage: (self.sys.used_memory() as f64 / self.sys.total_memory() as f64)
                        * 100.0,
                });

                if history.len() > MAX_POINTS {
                    history.pop_front();
                }
            }
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    egui::Grid::new("cpu_grid").show(ui, |ui| {
                        let mut count = 0;

                        for (i, cpu) in cpus.iter().enumerate() {
                            let cpu_points: PlotPoints = self
                                .graph_data
                                .iter()
                                .enumerate()
                                .map(|(x, _)| [x as f64, cpu.cpu_usage() as f64])
                                .collect();

                            ui.push_id(cpu.name(), |ui| {
                                ui.vertical(|ui| {
                                    Plot::new(format!("cpu_plot_{i}"))
                                        .width(150.0)
                                        .height(75.0)
                                        .include_y(0.0)
                                        .include_y(100.0)
                                        .include_x(0.0)
                                        .include_x(MAX_POINTS as f64)
                                        // disable interaction
                                        .allow_drag(false)
                                        .allow_zoom(false)
                                        .allow_scroll(false)
                                        .show(ui, |plot_ui| {
                                            plot_ui.line(Line::new(cpu_points));
                                        });

                                    ui.label(format!(
                                        "Core usage: {:.1}% @ {:.0} MHz",
                                        cpu.cpu_usage(),
                                        cpu.frequency()
                                    ));

                                    ui.add_space(6.0);
                                });
                            });

                            count += 1;
                            if count > 2 {
                                ui.end_row();
                                count = 0;
                            }
                        }
                    });
                });
            if self.settings_window.showcpufeatures {
                ui.heading("CPU Features");
                ui.label(RichText::new("Detected CPU features:").strong());
                for (feature, supported) in &self.cpu_features {
                    ui.label(format!(
                        "{}: {}",
                        feature,
                        if *supported { "Supported" } else { "Unsupported" }
                    ));
                }
            }
        });
    }
}

// Detect useful CPU features depending on the target architecture.
pub fn detect_cpu_features() -> Vec<(&'static str, bool)> {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        vec![
            ("sse", is_x86_feature_detected!("sse")),
            ("sse2", is_x86_feature_detected!("sse2")),
            ("sse3", is_x86_feature_detected!("sse3")),
            ("ssse3", is_x86_feature_detected!("ssse3")),
            ("sse4.1", is_x86_feature_detected!("sse4.1")),
            ("sse4.2", is_x86_feature_detected!("sse4.2")),
            ("avx", is_x86_feature_detected!("avx")),
            ("avx2", is_x86_feature_detected!("avx2")),
            ("avx512f", is_x86_feature_detected!("avx512f")),
            ("fma", is_x86_feature_detected!("fma")),
            ("popcnt", is_x86_feature_detected!("popcnt")),
            ("bmi1", is_x86_feature_detected!("bmi1")),
            ("bmi2", is_x86_feature_detected!("bmi2")),
        ]
    }
    #[cfg(target_arch = "aarch64")]
    {
        vec![
            ("neon", is_aarch64_feature_detected!("neon")),
            ("fp", is_aarch64_feature_detected!("fp")),
            ("aes", is_aarch64_feature_detected!("aes")),
            ("pmull", is_aarch64_feature_detected!("pmull")),
            ("sha2", is_aarch64_feature_detected!("sha2")),
        ]
    }
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
    {
        Vec::new()
    }
}
