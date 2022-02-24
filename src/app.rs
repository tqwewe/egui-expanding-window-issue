use eframe::{
    egui::{self, Layout, RichText, ScrollArea, TextEdit, Window},
    epi,
};

const LEFT_POS: f32 = 200.0;
const TOP_POS: f32 = 100.0;
const HEIGHT: f32 = 200.0;
const WIDTH: f32 = 800.0;

pub struct App;

impl epi::App for App {
    fn name(&self) -> &str {
        "app"
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        Window::new("Console")
            .default_pos([LEFT_POS, TOP_POS])
            .default_size([WIDTH, HEIGHT])
            .show(ctx, |ui| {
                let scroll_height = ui.available_height() - 30.0;

                ui.vertical(|ui| {
                    // Scroll area
                    ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .stick_to_bottom()
                        .show(ui, |ui| {
                            ui.set_max_height(scroll_height);
                        });

                    // Input
                    ui.add(
                        TextEdit::singleline(&mut String::new())
                            .desired_width(f32::INFINITY)
                            .font(egui::TextStyle::Monospace),
                    );
                });
            });
    }
}
