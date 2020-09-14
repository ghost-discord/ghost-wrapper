use log::info;
use iced::{Element, Sandbox, Settings, Text};

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

struct GhostWrapper;

impl Sandbox for GhostWrapper {
    type Message = ();

    fn new() -> GhostWrapper {
        GhostWrapper
    }

    fn title(&self) -> String {
        String::from("ghost2-wrapper")
    }

    fn update(&mut self, _message: Self::Message) {
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        Text::new("Hello, ghost!")
            .size(64)
            .into()
    }
}

fn fern_init() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .level_for("iced", log::LevelFilter::Info)
        .level_for("iced_wgpu", log::LevelFilter::Info)
        .level_for("wgpu_native", log::LevelFilter::Info)
        .level_for("gfx_backend_vulkan", log::LevelFilter::Error)
        .level_for("gfx_backend_metal", log::LevelFilter::Error)
        .level_for("gfx_backend_dx11", log::LevelFilter::Error)
        .level_for("gfx_backend_dx12", log::LevelFilter::Error)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

fn main() {
    if let Err(why) = fern_init() {
        panic!("Failed to initialize Fern: {:?}", why)
    }

    info!("ghost2-wrapper version {}", VERSION.unwrap_or("???"));
    GhostWrapper::run(Settings::default());
    info!("exiting");
}