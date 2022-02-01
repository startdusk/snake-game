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
pub struct World {
    pub width: usize,
}

#[wasm_bindgen]
impl World {
    pub fn new() -> World {
        World { width: 8 }
    }

    pub fn width(&self) -> usize {
        self.width
    }
}
