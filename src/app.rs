#[derive(Default)]
pub struct VcApp {
    checkbox: bool,
}

impl eframe::App for VcApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Test application");

            ui.checkbox(&mut self.checkbox, "Test button");
            if self.checkbox {
                ui.label("Hello world");
            }
        });
    }
}
