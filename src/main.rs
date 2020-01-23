use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderDebugLines, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    window::DisplayConfig,
    LoggerConfig, StdoutLog,
};

mod commands;
mod common;
mod components;
mod resources;
mod states;
mod systems;

fn main() {
    match run_app() {
        Ok(_) => {}
        Err(e) => {
            println!("Application quit with error: {:?}", e);
        }
    }
}

fn run_app() -> amethyst::Result<()> {
    let app_root = amethyst::utils::application_root_dir()?;
    let mut logger: LoggerConfig = Default::default();
    logger.log_file = Some(app_root.join("log.txt"));
    logger.stdout = StdoutLog::Off;
    amethyst::start_logger(logger);

    let assets_path = app_root.join("assets/");

    let display_config = DisplayConfig {
        title: "Pretender".to_string(),
        fullscreen: None,
        dimensions: Some((800, 800)),
        min_dimensions: Some((300, 300)),
        max_dimensions: None,
        visibility: true,
        icon: Some(assets_path.join("icon.png")),
        always_on_top: false,
        decorations: true,
        maximized: false,
        multitouch: false,
        resizable: true,
        transparent: false,
        loaded_icon: None,
    };

    let game_data = GameDataBuilder::default()
        // .with(ExampleLinesSystem::new(), "example_lines_system", &[])
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(InputBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config(display_config)
                        .with_clear([0.0005, 0.0005, 0.0005, 1.0]),
                )
                .with_plugin(RenderUi::default())
                .with_plugin(RenderDebugLines::default()),
        )?;
    let initial_state = states::RootState {
        // zoom_level: 1.0,
        // origin_x: 0.0,
        // origin_y: 0.0,
        // domain_w: 600.0,
        // domain_h: 600.0,
        cursor: (0.0, 0.0),
    };

    let mut game = Application::new(app_root, initial_state, game_data)?;

    game.run();

    Ok(())
}
