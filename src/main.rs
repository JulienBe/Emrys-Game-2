use std::{env, path};
use ggez::{conf, Context, ContextBuilder, event, GameResult, graphics};
use ggez::graphics::{Canvas, Color, Image as Img};

struct Assets {
  block_imgs: [Img; 2],
}
impl Assets {
  fn new(ctx: &mut Context) -> GameResult<Assets> {
    Ok(Assets {
      block_imgs: [
        Img::from_path(ctx, "/images/block_img1.png")?,
        Img::from_path(ctx, "/images/block_img2.png")?
      ]
    })
  }
  fn block_img(&self, block: &Block) -> &Img {
    &self.block_imgs[block.lvl as usize]
  }
}
struct Block {
  cell_x: i8,
  cell_y: i8,
  lvl: i8,
}
impl Block {
  fn new() -> GameResult<Block> {
    Ok(Block { cell_x: 0, cell_y: 0, lvl: 1, })
  }
}

struct MainState {
  assets: Assets,
  block: Block,
}
impl MainState {
  fn new(ctx: &mut Context) -> GameResult<MainState> {

    Ok(MainState {
      assets: Assets::new(ctx)?,
      block: Block::new().unwrap(),
    })
  }
}

impl event::EventHandler<ggez::GameError> for MainState {
  fn update(&mut self, _ctx: &mut Context) -> GameResult {
    Ok(())
  }
  fn draw(&mut self, ctx: &mut Context) -> GameResult {
    let mut canvas = Canvas::from_frame(ctx, Color::from([0.0, 0.0, 0.0, 1.0]));
    let block_img = self.assets.block_img(&self.block);
    let draw_params = graphics::DrawParam::new()
      .dest([50.0, 50.0])
      .scale([2.0, 2.0])
      .offset([0.5, 0.5]);
    canvas.draw(block_img, draw_params);

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
