import init, { World, Direction } from "snake_game";
import { rnd } from "./utils/md";

const HEADER_COLOR = "#7878db";
const UNHEADER_COLOR = "#000000";
const REWARD_COLOR = "#ff0000";

init().then((wasm) => {
  const CELL_SIZE = 20;
  const WORLD_WIDTH = 4;
  const snakeSpawnIdx = rnd(WORLD_WIDTH * WORLD_WIDTH);

  const world = World.new(WORLD_WIDTH, snakeSpawnIdx);
  const worldWidth = world.width();

  const gameStatus = document.getElementById("game-status");
  const gameControlBtn = document.getElementById("game-control-btn");
  const canvas = <HTMLCanvasElement>document.getElementById("snake-canvas");
  const ctx = canvas.getContext("2d");

  gameControlBtn.addEventListener("click", (_) => {
    const status = world.game_status();
    if (status === undefined) {
      gameControlBtn.textContent = "Playing...";
      world.start_game();
      play();
    } else {
      location.reload();
    }
  });

  document.addEventListener("keydown", (ev: KeyboardEvent) => {
    switch (ev.code) {
      case "ArrowUp":
        world.change_snake_dir(Direction.Up);
        break;
      case "ArrowDown":
        world.change_snake_dir(Direction.Down);
        break;
      case "ArrowLeft":
        world.change_snake_dir(Direction.Left);
        break;
      case "ArrowRight":
        world.change_snake_dir(Direction.Right);
        break;
    }
  });
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

  // 生成贪吃蛇要吃的食物方块
  function drawReward() {
    const idx = world.reward_cell();
    const col = idx % worldWidth;
    const row = Math.floor(idx / worldWidth);

    ctx.beginPath();
    ctx.fillStyle = REWARD_COLOR;
    ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
    ctx.stroke();
  }

  function drawSnake() {
    const snakeCells = new Uint32Array(
      wasm.memory.buffer,
      world.snake_cells(),
      world.snake_length()
    );
    snakeCells.forEach((cellIdx, i) => {
      const col = cellIdx % worldWidth;
      const row = Math.floor(cellIdx / worldWidth);

      // we are overriding snake head color by body when we crush
      ctx.fillStyle = i === 0 ? HEADER_COLOR : UNHEADER_COLOR;
      ctx.beginPath();
      ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
    });
    ctx.stroke();
  }

  function drawGameStatus() {
    gameStatus.textContent = world.game_status_text();
  }

  function paint() {
    drawWorld();
    drawSnake();
    drawReward();
    drawGameStatus();
  }

  function play() {
    const fps = 3;
    setTimeout(() => {
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      paint();
      world.step();
      // the method takes a callback
      // to invoked before the next repaint
      requestAnimationFrame(play);
    }, 1000 / fps);
  }

  paint();
});
