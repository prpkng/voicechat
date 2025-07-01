#[derive(Default)]
pub struct VcApp {
    checkbox: bool,
    host: Option<Box<dyn Fn() -> ()>>,
    join: Option<Box<dyn Fn() -> ()>>,
}

impl VcApp {
    pub fn set_host(&mut self, host_func: impl Fn() -> () + 'static) {
        self.host = Some(Box::new(host_func));
    }

    pub fn set_join(&mut self, join_func: impl Fn() -> () + 'static) {
        self.join = Some(Box::new(join_func));
    }
}

impl eframe::App for VcApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Test application");

            if ui.button("Host").clicked() {
                self.host.as_mut().unwrap()();

                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }

            if ui.button("Join").clicked() {
                self.join.as_mut().unwrap()();

                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });
    }
}
