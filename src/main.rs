mod task;
mod storage;
mod sorting;
mod app;
use eframe::NativeOptions;

fn main() {
    let options = NativeOptions::default();
    let _ = eframe::run_native(
        "Task-Tracker",
        options,
        Box::new(|_cc| Ok(Box::new(app::MyApp::default()))),
    );
}


