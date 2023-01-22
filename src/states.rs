use ggez::{Context, GameError, GameResult};
use ggez::event::EventHandler;
use ggez::graphics::{Canvas, Color, DrawParam, Image as Img};
use ggez::input::keyboard::{KeyCode, KeyInput};

pub(crate) struct MainState {
  assets: Assets,
  input_state: InputState,
  block: Block,
}
impl MainState {
  pub(crate) fn new(ctx: &mut Context) -> GameResult<MainState> {

    Ok(MainState {
      assets: Assets::new(ctx)?,
      input_state: InputState::default(),
      block: Block::new().unwrap(),
    })
  }
}

impl EventHandler<GameError> for MainState {
  fn update(&mut self, _ctx: &mut Context) -> GameResult {
    Ok(())
  }
  fn draw(&mut self, ctx: &mut Context) -> GameResult {
    let mut canvas = Canvas::from_frame(ctx, Color::from([0.0, 0.0, 0.0, 1.0]));
    let block_img = self.assets.block_img(&self.block);
    let draw_params = DrawParam::new()
      .dest([50.0, 50.0])
      .scale([2.0, 2.0])
      .offset([0.5, 0.5]);
    canvas.draw(block_img, draw_params);

    canvas.finish(ctx)?;
    Ok(())
  }
  fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeated: bool) -> Result<(), GameError> {
    match input.keycode {
      Some(KeyCode::Up) => self.input_state.dir = Dir::Up,
      Some(KeyCode::Down) => self.input_state.dir = Dir::Down,
      Some(KeyCode::Left) => self.input_state.dir = Dir::Left,
      Some(KeyCode::Right) => self.input_state.dir = Dir::Right,
      _ => (),
    }
    Ok(())
  }
}
enum Dir { Up,  Down,  Left,  Right,  None }
struct InputState {
  dir: Dir,
}
impl Default for InputState {&
  fn default() -> Self {
    InputState { dir: Dir::None }
  }
}
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