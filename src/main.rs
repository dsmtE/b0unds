mod app;
mod camera;
mod runtime_shader_builder;

use fern::colors::ColoredLevelConfig;
fn main() {
    let color_config = ColoredLevelConfig::new();
    fern::Dispatch::new()
        .level(log::LevelFilter::Trace)
        .level_for("wgpu_hal", log::LevelFilter::Warn)
        .level_for("naga", log::LevelFilter::Warn)
        .level_for("wgpu_core", log::LevelFilter::Warn)
        .format(|out, message, record| {
            out.finish(format_args!(
                "{time}[{level}][{target}] {message}",
                time = chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                level = record.level(),
                target = record.target(),
                message = message,
            ))
        })
        .chain(fern::Dispatch::new().chain(std::io::stdout()).format(move |out, message, record| {
            out.finish(format_args!(
                "{color}{message}{color_reset}",
                message = message,
                color = format!("\x1B[{}m", color_config.get_color(&record.level()).to_fg_str()),
                color_reset = "\x1B[0m",
            ))
        }))
        .apply()
        .unwrap();

    oxyde::run_application::<app::B0oundsApp>(oxyde::AppConfig {
        is_resizable: true,
        title: "B0unds",
        icon: None,
    }, oxyde::RenderingConfig::default())
    .unwrap();
}
