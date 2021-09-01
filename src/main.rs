extern crate nalgebra as na;

mod fiz;
mod game;
mod thing;

use crate::game::Game;

#[macroquad::main("3D")]
async fn main() {
  let mut game = Game::new();
  game.main().await;
}
