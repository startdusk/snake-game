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

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy)]
pub struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    fn new(spawn_index: usize, size: usize) -> Self {
        let mut body = vec![];
        for i in 0..size {
            body.push(SnakeCell(spawn_index - i));
        }

        Snake {
            body,
            direction: Direction::Right,
        }
    }
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    snake: Snake,
    next_cell: Option<SnakeCell>,
    reward_cell: usize,
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
            snake: Snake::new(spawn_idx, 3),
            next_cell: None,
            reward_cell: 10,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn reward_cell(&self) -> usize {
        self.reward_cell
    }

    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn change_snake_dir(&mut self, direction: Direction) {
        let next_cell = self.gen_next_snake_cell(&direction);
        // 禁止反方向移动
        if self.snake.body.len() > 1 && self.snake.body[1].0 == next_cell.0 {
            return;
        }
        // 通过按键 更新下一步蛇头要到的位置
        self.next_cell = Some(next_cell);
        self.snake.direction = direction;
    }

    pub fn snake_length(&self) -> usize {
        self.snake.body.len()
    }

    // *const is raw pointer
    // borrowing rules doesn't apply to it
    pub fn snake_cells(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()
    }
    // cannot return a reference to JavaScript
    // because of borrowing rules
    // pub fn snake_cells(&self) -> &Vec<SnakeCell> {
    //     &self.snake.body
    // }

    pub fn oopsie(&mut self) {
        self.snake.body = vec![SnakeCell(2048)]
    }

    pub fn step(&mut self) {
        // tmp是记录原贪吃蛇的位置信息，方便后面更新蛇身的位置
        let tmp = self.snake.body.clone();

        // 更新贪吃蛇头的位置
        match self.next_cell {
            // 通过方向键改变蛇头的位置
            Some(cell) => {
                self.snake.body[0] = cell;
                self.next_cell = None;
            }
            // 同一个方向上自动生成蛇头的位置
            None => {
                self.snake.body[0] = self.gen_next_snake_cell(&self.snake.direction);
            }
        }

        // 更新贪吃蛇蛇身的位置
        // 蛇头更新了，那蛇身就会往原蛇头的位置挪动并覆盖
        let len = self.snake.body.len();
        for i in 1..len {
            self.snake.body[i] = SnakeCell(tmp[i - 1].0);
        }
    }

    fn gen_next_snake_cell(&self, direction: &Direction) -> SnakeCell {
        let snake_idx = self.snake_head_idx();
        let row = snake_idx / self.width;
        // 除法在wasm-pack打包后体积会很大，所以改用了加法
        return match direction {
            Direction::Right => {
                let treshold = (row + 1) * self.width;
                if snake_idx + 1 == treshold {
                    SnakeCell(treshold - self.width)
                } else {
                    SnakeCell(snake_idx + 1)
                }
            }
            Direction::Left => {
                let treshold = row * self.width;
                if snake_idx == treshold {
                    SnakeCell(treshold + (self.width - 1))
                } else {
                    SnakeCell(snake_idx - 1)
                }
            }
            Direction::Up => {
                let treshold = snake_idx - (row * self.width);
                if snake_idx == treshold {
                    SnakeCell((self.size - self.width) + treshold)
                } else {
                    SnakeCell(snake_idx - self.width)
                }
            }
            Direction::Down => {
                let treshold = snake_idx + ((self.width - row) * self.width);
                if snake_idx + self.width == treshold {
                    SnakeCell(treshold - ((row * 1) * self.width))
                } else {
                    SnakeCell(snake_idx + self.width)
                }
            }
        };
    }
}
