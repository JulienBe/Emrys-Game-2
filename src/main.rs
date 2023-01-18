use std::{env, path};
use ggez::{conf, Context, event, GameResult};

struct MainState {
}
impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState { };
        Ok(s)
    }
}
impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
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
    let cb = ggez::ContextBuilder::new("Emrys Game", "Pap'")
        .window_setup(conf::WindowSetup::default().title("Emrys Game"))
        .add_resource_path(resource_dir);
    let (mut ctx, events_loop) = cb.build()?;
    let state = MainState::new(&mut ctx).unwrap();
    event::run(ctx, events_loop, state)
}
