mod ui;
mod error;
mod system;
mod log_export;
mod setting;
mod language;
mod play;

use crate::system::init::check;
use log;
use crate::system::system_function::clear_log;
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

fn _main(mut options: NativeOptions){
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

    _main(options)
}

#[cfg(not(target_os = "android"))]
fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Warn) // Default Log Level
        .parse_default_env()
        .init();

    _main(NativeOptions::default());
}

fn log_name_generate() -> String {
    let fmt = "%Y-%m-%d %H%M%S";
    let now = Local::now().format(fmt).to_string();
    let mut log_name = "data/data/com.saving.shapoist/assets/log/[".to_string();
    log_name += &now;
    log_name += &"]running.log".to_string();
    log_name
}