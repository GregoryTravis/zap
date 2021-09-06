extern crate nalgebra as na;

use macroquad::prelude::*;

mod fiz;
mod game;
mod thing;

use crate::game::Game;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Macroquad"),
        window_width: 1260,
        window_height: 768,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
  let mut game = Game::new();
  game.main().await;
}
