mod app;

fn main() {
    let app = app::App;
    let native_options = eframe::NativeOptions {
        initial_window_size: Some([1280.0, 720.0].into()),
        ..Default::default()
    };
    eframe::run_native(Box::new(app), native_options);
}
