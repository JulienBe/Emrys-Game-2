use std::{env, path};
use ggez::{conf, Context, ContextBuilder, event, GameResult, graphics};
use ggez::graphics::{Canvas, Color, Image};

struct MainState {
  image1: Image,
}

impl MainState {
  fn new(ctx: &mut Context) -> GameResult<MainState> {
    let s = MainState {
        image1: Image::from_path(ctx, "/images/image1.png")?,
    };
    Ok(s)
  }
}

impl event::EventHandler<ggez::GameError> for MainState {
  fn update(&mut self, _ctx: &mut Context) -> GameResult {
    Ok(())
  }
  fn draw(&mut self, ctx: &mut Context) -> GameResult {
    let mut canvas = Canvas::from_frame(ctx, Color::from([0.0, 0.0, 0.0, 1.0]));
    canvas.draw(
      &self.image1,
      graphics::DrawParam::new()
        .dest([50.0, 50.0])
        .scale([2.0, 2.0])
        .offset([0.5, 0.5])
    );

    canvas.finish(ctx)?;
    Ok(())
  }
}

pub fn main() -> GameResult {
  let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
    let mut path = path::PathBuf::from(manifest_dir);
    path.push("resources");
    path
  } else {
    path::PathBuf::from("./resources")
  };
  let cb = ContextBuilder::new("Emrys Game", "Pap'")
    .window_setup(conf::WindowSetup::default().title("Emrys Game"))
    .add_resource_path(resource_dir);
  let (mut ctx, events_loop) = cb.build()?;
  let state = MainState::new(&mut ctx).unwrap();
  event::run(ctx, events_loop, state)
}
