#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] 
pub struct TextEditorApp {
    label: String,

    #[serde(skip)]
    value: f32,
}

impl Default for TextEditorApp {
    fn default() -> Self {
        Self {
            label: "Hello World!".to_owned(),
            value: 2.7,
        }
    }
}

impl TextEditorApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TextEditorApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { label, value } = self;

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Text Editor");
            egui::warn_if_debug_build(ui);
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
            });
        }
    }
}
