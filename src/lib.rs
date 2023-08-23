mod ui;
mod error;
mod system;
mod log_export;
mod setting;
mod language;
mod play;

use crate::system::init::check;
use crate::system::system_function::clear_log;
use crate::system::system_function::load_icon;
use crate::error::error::ShapoError;
use crate::system::system_function::create_file;
use eframe::NativeOptions;
use eframe::run_native;
use once_cell::sync::Lazy;
use chrono::Local;
use std::env;
#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;

pub static LOGPATH: Lazy<String> = Lazy::new(|| {log_name_generate()});
#[cfg(target_os = "android")]
pub static ASSETS_PATH: Lazy<String> = Lazy::new(|| String::from("data/data/com.saving.shapoist"));
#[cfg(not(target_os = "android"))]
pub static ASSETS_PATH: Lazy<String> = Lazy::new(|| String::from("."));

fn entry(mut options: NativeOptions){
    let args: Vec<String> = env::args().collect();
    let _ = check();
    create_file(&LOGPATH).unwrap();
    options.renderer = eframe::Renderer::Wgpu;
    run_native("shapoist",options, Box::new(|cc| Box::new(crate::ui::page::Page::new(cc, args).unwrap()))).unwrap();
}

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: AndroidApp) {
    use winit::platform::android::EventLoopBuilderExtAndroid;

    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Debug),
    );

    let options = NativeOptions {
        event_loop_builder: Some(Box::new(move |builder| {
            builder.with_android_app(app);
        })),
        ..Default::default()
    };

    entry(options)
}

#[cfg(not(target_os = "android"))]
fn main() {
    env_logger::builder()
        .filter(Some("adgk-shapoist::log_out::log_export"), log::LevelFilter::Info)
        .parse_default_env()
        .init();
    let icon = match load_icon(&format!("{}/assets/icon/icon.png", *ASSETS_PATH)) {
        Ok(t) => Some(t),
        Err(_) => None
    };
    let native_options = NativeOptions{
        resizable: true,
        initial_window_size: Some(egui::Vec2 { x: 800.0, y: 600.0 }),
        min_window_size: Some(egui::Vec2 { x: 800.0, y: 600.0 }),
        icon_data: icon,
        drag_and_drop_support: true,
        ..Default::default()
    };
    entry(native_options);
}

fn log_name_generate() -> String {
    let fmt = "%Y-%m-%d %H%M%S";
    let now = Local::now().format(fmt).to_string();
    format!("{}/assets/log/[{}]running.log", *ASSETS_PATH, now)
}