extern crate rand;
extern crate piston_window;

use std::collections::LinkedList;
use piston_window::types::Color;
use piston_window::{Context, G2d,Rectangle};


pub fn proceed() {
    println!("Snake game");

}


pub enum Direction{
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => {Direction::Down}
            Direction::Down => {Direction::Up}
            Direction::Left => {Direction::Right}
            Direction::Right => {Direction::Left}
        }
    }
}

pub struct Block{
    x: i32,
    y: i32,
}

pub struct Snake{
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

impl Snake {


}

