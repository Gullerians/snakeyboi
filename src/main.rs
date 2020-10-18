use std::io::{stdout, Write};

use crossterm::{
    event, execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand, Result,
};
use std::collections::VecDeque;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Snake {
    snake: VecDeque<(usize, usize)>,
}

impl Snake {
    fn new() -> Self {
        let mut snake = VecDeque::new();
        snake.push_back((39, 11));
        snake.push_back((40, 11));
        snake.push_back((41, 11));
        snake.push_back((42, 11));
        Snake { snake }
    }

    fn snake_move(&mut self, direction: Direction) {
        self.snake.pop_back();
        match direction {
            Direction::Up => {
                if self.snake[0].0 != self.snake[1].0 {
                    self.snake
                        .push_front((self.snake[0].0, self.snake[0].1 + 1))
                }
            }
            Direction::Down => {
                if self.snake[0].0 != self.snake[1].0 {
                    self.snake
                        .push_front((self.snake[0].0, self.snake[0].1 - 1))
                }
            }
            Direction::Right => {
                if self.snake[0].1 != self.snake[1].1 {
                    self.snake
                        .push_front((self.snake[0].0 + 1, self.snake[0].1))
                }
            }
            Direction::Left => {
                if self.snake[0].1 != self.snake[1].1 {
                    self.snake
                        .push_front((self.snake[0].0 - 1, self.snake[0].1))
                }
            }
        }
    }
}

struct App {}

fn main() {}
