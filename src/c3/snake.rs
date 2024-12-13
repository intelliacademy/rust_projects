extern crate rand;
extern crate piston_window;

use std::collections::LinkedList;
use piston_window::types::Color;
use piston_window::{Context, G2d,Rectangle};
use crate::c3::draw::*;
use crate::c3::SNAKE_COLOR;

pub fn proceed() {
    println!("Snake game");

}


#[derive(PartialEq, Clone)]
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

    pub fn new(x: i32,y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block{ x: x+2, y });
        body.push_back(Block{ x: x+1, y });
        body.push_back(Block{ x, y });

        Snake {
            direction: Direction::Right,
            body,
            tail: None,
        }
    }

    pub fn draw(&self, context: &Context,g2d: &mut G2d) {
        for block in self.body.iter() {
            draw_block(SNAKE_COLOR, block.x, block.y, context, g2d);
        }
    }

    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }

    pub fn move_forward(&mut self, dir: Option<Direction>){

    }

    pub fn default() -> Snake {
        Snake::new(0,0)
    }

    pub fn right(&mut self) {
        if self.direction != Direction::Left {
            self.direction = Direction::Right;
        }
    }

    pub fn left(&mut self) {
        if self.direction != Direction::Right {
            self.direction = Direction::Left;
        }
    }

    pub fn up(&mut self) {
        if self.direction != Direction::Down {
            self.direction = Direction::Up;
        }
    }

    pub fn down(&mut self) {
        if self.direction != Direction::Up {
            self.direction = Direction::Down;
        }
    }

}

