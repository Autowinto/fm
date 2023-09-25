use eframe::App;
use egui::epaint::Shadow;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        ..Default::default()
    };

    eframe::run_native(
        "App",
        native_options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    )
}

struct MyApp {
    processNodes: Vec<ProcessNode>,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext) -> Self {
        configure_styles(&cc.egui_ctx);

        Self {
            processNodes: Vec::new(),
        }
    }
}

fn configure_styles(ctx: &egui::Context) {
    let mut style: egui::Style = Default::default();
    style.visuals.window_shadow = Shadow::NONE;
    style.visuals.resize_corner_size = 0.0;

    ctx.set_style(style);
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                egui::Window::new("Aooga").show(ctx, |ui| ui.heading("Aooga"));
                ui.horizontal(|ui| {
                    egui::Window::new("Title").show(ctx, |ui| ui.heading("Header"));
                    if ui.add(egui::Button::new("Add")).clicked() {
                        self.processNodes.push(ProcessNode {
                            title: "New Title".to_string(),
                        })
                    };
                });
                // egui::Button::new("Aooga")
            });

            for (idx, processNode) in self.processNodes.iter().enumerate() {
                egui::Window::new(processNode.title.to_string())
                    .id(egui::Id::new(idx))
                    .title_bar(false)
                    .show(ui.ctx(), |ui| ui.heading("Header"));
            }
        });
    }
}

#[derive(Default)]
struct ProcessNode {
    title: String,
}

impl ProcessNode {
    pub fn show(&mut self, ui: &mut eframe::egui::Ui) {
        egui::Window::new(self.title.as_str()).show(ui.ctx(), |ui| ui.heading("Header"));
    }
}
