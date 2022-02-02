import init, { World } from "snake_game";

init().then((_) => {
  const CELL_SIZE = 10;
  const WORLD_WIDTH = 8;
  const snakeSpawnIdx = Date.now() % (WORLD_WIDTH * WORLD_WIDTH);

  const world = World.new(WORLD_WIDTH, snakeSpawnIdx);
  const worldWidth = world.width();

  const canvas = <HTMLCanvasElement>document.getElementById("snake-canvas");
  const ctx = canvas.getContext("2d");

  canvas.height = worldWidth * CELL_SIZE;
  canvas.width = worldWidth * CELL_SIZE;

  function drawWorld() {
    ctx.beginPath();

    for (let x = 0; x <= worldWidth; x++) {
      // 水平移动 CELL_SIZE * x 距离的位置
      ctx.moveTo(CELL_SIZE * x, 0);
      // 垂直方向上画线 线长度为worldWidth * CELL_SIZE
      ctx.lineTo(CELL_SIZE * x, worldWidth * CELL_SIZE);
    }

    for (let y = 0; y <= worldWidth; y++) {
      // 垂直移动 CELL_SIZE * y 距离的位置
      ctx.moveTo(0, CELL_SIZE * y);
      // 水平方向上画线 线长度为worldWidth * CELL_SIZE
      ctx.lineTo(worldWidth * CELL_SIZE, CELL_SIZE * y);
    }

    ctx.stroke();
  }

  function drawSnake() {
    const snakeIdx = world.snake_head_idx();
    const col = snakeIdx % worldWidth;
    const row = Math.floor(snakeIdx / worldWidth);

    ctx.beginPath();
    ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
    ctx.stroke();
  }

  function paint() {
    drawWorld();
    drawSnake();
  }

  function update() {
    const fps = 3;
    setTimeout(() => {
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      paint();
      world.update();
      // the method takes a callback
      // to invoked before the next repaint
      requestAnimationFrame(update);
    }, 1000 / fps);
  }

  paint();
  update();
});
