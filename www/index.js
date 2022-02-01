import init, { World } from "snake_game";

init().then((_) => {
  const CELL_SIZE = 10;
  const world = World.new();
  const worldWidth = world.width();

  const canvas = document.getElementById("snake-canvas");
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

  drawWorld();
});
