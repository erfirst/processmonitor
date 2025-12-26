use egui;

use system_shutdown::{hibernate, logout, shutdown};

pub struct SettingsWindow {
    pub open: bool,
    pub collapse: bool,
    pub showcpufeatures: bool,
    pub refresh_cpufeatures: bool,
}

impl Default for SettingsWindow {
    fn default() -> Self {
        Self {
            open: false,
            collapse: false,
            showcpufeatures: false,
            refresh_cpufeatures: false,
        }
    }
}

impl SettingsWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        if self.open {
            egui::Window::new("Settings")
                .open(&mut self.open)
                .default_width(400.0)
                .show(ctx, |ui| {
                    ui.heading("Settings");
                    ui.checkbox(&mut self.collapse, "Collapse same name processes");
                    ui.checkbox(&mut self.showcpufeatures, "Show CPU Features");
                    if ui.button("Refresh CPU features").clicked() {
                        self.refresh_cpufeatures = true;
                    }
                    ui.separator();
                    ui.label("Global Theme:");
                    egui::menu::bar(ui, |ui| {
                        egui::widgets::global_theme_preference_buttons(ui);
                    });
                    ui.separator();
                    ui.label("System Actions:");
                    if ui.button("Shutdown").clicked() {
                        let _ = shutdown();
                    }
                    if ui.button("Hibernate").clicked() {
                        let _ = hibernate();
                    }
                    if ui.button("Log Out").clicked() {
                        let _ = logout();
                    }
                    if ui.button("Sleep").clicked() {
                        let _ = system_shutdown::sleep();
                    }
                });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_values() {
        let s = SettingsWindow::default();
        assert_eq!(s.open, false);
        assert_eq!(s.collapse, false);
        assert_eq!(s.showcpufeatures, false);
        assert_eq!(s.refresh_cpufeatures, false);
    }
}
