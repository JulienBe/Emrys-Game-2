use ggez::{Context, GameError, GameResult};
use ggez::event::EventHandler;
use ggez::graphics::{Canvas, Color, DrawParam, Image as Img};
use ggez::input::keyboard::{KeyCode, KeyInput};
use rand::prelude::ThreadRng;
use rand::Rng;

const CELL_SIZE: f32 = 160.0;
const HALF_CELL_SIZE: f32 = CELL_SIZE / 2.0;
const MIN_CELL: i8 = 0;
const MAX_CELL: i8 = 3;

pub(crate) struct MainState {
  assets: Assets,
  input_state: InputState,
  blocks: Vec<Block>,
  all_valid_pos: Vec<(i8, i8)>,
  rand: ThreadRng,
}
impl MainState {
  pub(crate) fn new(ctx: &mut Context) -> GameResult<MainState> {
    Ok(MainState {
      assets: Assets::new(ctx)?,
      input_state: InputState::default(),
      blocks: vec![Block::new()],
      all_valid_pos: (MIN_CELL..=MAX_CELL).flat_map(|x| (MIN_CELL..=MAX_CELL).map(move |y| (x, y))).collect(),
      rand: rand::thread_rng(),
    })
  }
}

impl EventHandler<GameError> for MainState {
  fn update(&mut self, _ctx: &mut Context) -> GameResult {
    match self.input_state.dir {
      Dir::Up => self.blocks.iter_mut().for_each(|block| { block.cell_y -= 1 }),
      Dir::Down => self.blocks.iter_mut().for_each(|block| { block.cell_y += 1 }),
      Dir::Left => self.blocks.iter_mut().for_each(|block| { block.cell_x -= 1 }),
      Dir::Right => self.blocks.iter_mut().for_each(|block| { block.cell_x += 1 }),
      Dir::None => {}
    }
    if self.input_state.spawn_block {
      self.input_state.spawn_block = false;
      let valid_pos: Vec<&(i8, i8)> = self.all_valid_pos.iter()
        .filter(|(x, y)| {
          !self.blocks.iter().any(|b| &b.cell_x == x && &b.cell_y == y)
        })
        .collect();

      if valid_pos.len() > 0 {
        let index = self.rand.gen_range(0..valid_pos.len());
        self.blocks.push(Block::new_from_pos(*valid_pos[index]));
      }
    }

    self.input_state.dir = Dir::None;
    self.blocks.iter_mut().for_each(|block| {
      block.actual_x += ((block.cell_x as f32 * CELL_SIZE) - block.actual_x) / 10.0;
      block.actual_y += ((block.cell_y as f32 * CELL_SIZE) - block.actual_y) / 10.0;
    });
    Ok(())
  }
  fn draw(&mut self, ctx: &mut Context) -> GameResult {
    let mut canvas = Canvas::from_frame(ctx, Color::from([0.0, 0.0, 0.0, 1.0]));
    &self.blocks.iter().for_each(|block| {
      let block_img = self.assets.block_img(block);

      let draw_params = DrawParam::new()
        .dest([block.actual_x + HALF_CELL_SIZE, block.actual_y + HALF_CELL_SIZE])
        .scale([0.5, 0.5])
        .offset([0.5, 0.5]);
      canvas.draw(block_img, draw_params);
    });

    canvas.finish(ctx)?;
    Ok(())
  }
  fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeated: bool) -> Result<(), GameError> {
    self.input_state.spawn_block = true; // will be set to false if it's not a valid key
    match input.keycode {
      Some(KeyCode::Up) => self.input_state.dir = Dir::Up,
      Some(KeyCode::Down) => self.input_state.dir = Dir::Down,
      Some(KeyCode::Left) => self.input_state.dir = Dir::Left,
      Some(KeyCode::Right) => self.input_state.dir = Dir::Right,
      _ => self.input_state.spawn_block = false,
    }
    Ok(())
  }
}
enum Dir { Up,  Down,  Left,  Right,  None }
struct InputState {
  dir: Dir,
  spawn_block: bool,
}
impl Default for InputState {
  fn default() -> Self {
    InputState { dir: Dir::None, spawn_block: false }
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
enum BlockState { MOVING, IN_PLACE }
struct Block {
  cell_x: i8,  cell_y: i8,
  actual_x: f32,  actual_y: f32,
  lvl: i8,
  block_state: BlockState,
}
impl Block {
  fn new() -> Block {
    Block { cell_x: 0, cell_y: 0, actual_x: 0.0, actual_y: 0.0, lvl: 0, block_state: BlockState::IN_PLACE}
  }
  fn new_from_pos(pos: (i8, i8)) -> Block {
    Block { cell_x: pos.0, cell_y: pos.1, actual_x: 0.0, actual_y: 0.0, lvl: 0, block_state: BlockState::IN_PLACE}
  }
}