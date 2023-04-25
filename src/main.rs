use smithay::reexports::{calloop::EventLoop, wayland_server::Display};
use tracing::Level;

use crate::{
    backend::winit::initialize_winit,
    state::{NoWayData, NoWayState},
};

pub mod backend;
pub mod handlers;
pub mod state;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(env_filter) = tracing_subscriber::EnvFilter::try_from_env("NOWAY_LOG") {
        tracing::info!(
            "Logging is being initialized with env filter: {}",
            env_filter
        );

        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .with_env_filter(env_filter)
            .init();
        tracing::info!("Initialized logging with env filter successfully");
    } else {
        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .init();
        tracing::info!("Initialized logging with default filter successfully");
    }

    tracing::info!("Starting NoWay");
    let mut event_loop = EventLoop::try_new()?;
    let mut display = Display::new()?;
    let state = NoWayState::try_new(event_loop.handle(), event_loop.get_signal(), &mut display)?;
    let mut data = NoWayData { state, display };

    initialize_winit(&mut event_loop, &mut data)?;
    event_loop.run(None, &mut data, move |_| {})?;

    Ok(())
}
