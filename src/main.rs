mod states;

use std::{env, path};
use std::path::PathBuf;
use ggez::{ContextBuilder, event, GameResult};
use ggez::conf::{WindowMode, WindowSetup};

pub fn main() -> GameResult {
  let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
    let mut path = PathBuf::from(manifest_dir);
    path.push("resources");
    path
  } else {
    path::PathBuf::from("./resources")
  };
  let cb = ContextBuilder::new("Emrys Game", "Pap'")
    .window_setup(WindowSetup::default().title("Emrys Game"))
    .window_mode(WindowMode::default().dimensions(640.0, 640.0))
    .add_resource_path(resource_dir);
  let (mut ctx, events_loop) = cb.build()?;
  let state = states::MainState::new(&mut ctx).unwrap();
  event::run(ctx, events_loop, state)
}
