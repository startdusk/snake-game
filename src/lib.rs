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

struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
}

impl Snake {
    fn new(spawn_index: usize) -> Self {
        Snake {
            body: vec![SnakeCell(spawn_index)],
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
        self.snake.body[0].0 = (snake_idx + 1) % self.size;
    }
}
