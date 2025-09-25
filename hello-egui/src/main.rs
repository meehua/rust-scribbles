use eframe::egui;
use egui_extras::{install_image_loaders};

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Hello Egui",
        options,
        Box::new(|_cc|
            {
            Ok(Box::new(MyApp::default()))
        } ),
    ).expect("Failed to start eframe");
}

#[derive(Default)]
struct MyApp{
    reload_counter: i32
}
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        install_image_loaders(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello, World!");
            if ui.button("重新加载图片").clicked() {
                self.reload_counter += 1;
            }
            let image_url = format!("https://www.loliapi.com/acg/?{}", self.reload_counter);
            ui.image(&image_url);
        });
    }
}
