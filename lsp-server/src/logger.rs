use std::fs::OpenOptions;
use std::path::Path;

pub fn init_logger() {
    let timestamp = chrono::Local::now().format("%Y%m%d%H%M%S").to_string();
    let log_path = format!("lsp-{}.log", timestamp);
    let path = Path::new(&log_path);
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(path)
        .unwrap();
    simplelog::CombinedLogger::init(vec![simplelog::WriteLogger::new(
        simplelog::LevelFilter::Debug,
        simplelog::Config::default(),
        file,
    )])
    .unwrap();
    log::debug!("logger initialized");
}
