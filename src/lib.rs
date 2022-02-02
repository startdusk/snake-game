use std::cmp::PartialEq;
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

// 替换原有的内存分配，使得打包后体积更小
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

// 编译成前端可导入代码: wasm-pack build web
// 前端www依赖该编译代码:
//   "dependencies": {
//     "snake_game": "file:../pkg"
//   },
// npm install
// npm run dev

#[derive(PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    fn new(spawn_index: usize) -> Self {
        Snake {
            body: vec![SnakeCell(spawn_index)],
            direction: Direction::Down,
        }
    }
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    snake: Snake,
}

#[wasm_bindgen]
impl World {
    // width: 贪吃蛇游戏界面的宽度
    // width = 8 界面如下:
    // 0  1  2  3  4  5  6  7
    // 8  9  10 11 12 13 14 15
    // 16 17 18 19 20 21 22 23
    // 24 25 26 27 28 29 30 31
    // 32 33 34 35 36 37 38 39
    // 40 41 42 43 44 45 46 47
    // 48 49 50 51 52 53 54 55
    // 56 57 58 59 60 61 62 63
    // spawn_idx: 贪吃蛇初始化位置(在哪个格子上)
    // 如初始位置在 11 的位置上
    // 0  1  2  3  4  5  6  7
    // 8  9  10 ■  12 13 14 15
    // 16 17 18 19 20 21 22 23
    // 24 25 26 27 28 29 30 31
    // 32 33 34 35 36 37 38 39
    // 40 41 42 43 44 45 46 47
    // 48 49 50 51 52 53 54 55
    // 56 57 58 59 60 61 62 63
    pub fn new(width: usize, spawn_idx: usize) -> World {
        World {
            width,
            size: width * width,
            snake: Snake::new(spawn_idx),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn update(&mut self) {
        let snake_idx = self.snake_head_idx();
        let row = snake_idx / self.width;
        let col = snake_idx % self.width;

        if self.snake.direction == Direction::Right {
            let next_col = col + 1;
            self.snake.body[0].0 = (row * self.width) + next_col;
        }
        if self.snake.direction == Direction::Left {
            let next_col = col - 1;
            self.snake.body[0].0 = (row * self.width) + next_col;
        }
        if self.snake.direction == Direction::Up {
            let next_row = (row - 1) % self.width;
            self.snake.body[0].0 = (next_row * self.width) + col;
        }
        if self.snake.direction == Direction::Down {
            let next_row = (row + 1) % self.width;
            self.snake.body[0].0 = (next_row * self.width) + col;
        }
    }
}
