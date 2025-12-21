use log::LevelFilter;

pub fn init_logger(debug: bool) {
    let level = if debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };
    env_logger::builder()
        .format_timestamp_millis()
        .format_module_path(false)
        .format_target(false)
        .filter_level(level)
        .init();
}
