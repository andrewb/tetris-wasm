import init, { Cmd, Game } from "./pkg/tetris.js";

const ROW_COUNT = 20;
const COL_COUNT = 10;
const CELL_SIZE = 30;

const COLORS = [
  "#2efdfa",
  "#0a23f5",
  "#fda829",
  "#fdfb3c",
  "#2afc30",
  "#9925f9",
  "#fb0e1b",
];

const INPUTS = {
  ArrowUp: Cmd.Rotate,
  ArrowDown: Cmd.Drop,
  ArrowLeft: Cmd.Left,
  ArrowRight: Cmd.Right,
};

function block(ctx, x, y, value) {
  let sX = x * CELL_SIZE;
  let sY = y * CELL_SIZE;
  ctx.fillStyle = COLORS[value - 1];
  ctx.strokeStyle = "#000";
  ctx.beginPath();
  ctx.rect(sX, sY, CELL_SIZE, CELL_SIZE);
  ctx.closePath();
  ctx.fill();
  // Highlight
  ctx.beginPath();
  ctx.fillStyle = "rgba(255, 255, 255, 0.3)";
  ctx.moveTo(sX, sY);
  ctx.lineTo(sX + CELL_SIZE / 2, sY);
  ctx.lineTo(sX, sY + CELL_SIZE / 2);
  ctx.closePath();
  ctx.fill();
  // Outline
  ctx.strokeRect(sX, sY, CELL_SIZE, CELL_SIZE);
}

function load() {
  return Game.new(ROW_COUNT, COL_COUNT);
}

init().then(({ memory }) => {
  const canvas = document.getElementById("c");
  const ctx = canvas.getContext("2d");
  const hud = document.getElementById("hud");
  let game = load();
  let lastTickTime = 0;

  ctx.scale(2, 2);

  document.addEventListener("keydown", (e) => {
    const cmd = INPUTS[e.key];
    if (cmd !== undefined) {
      e.preventDefault();
      game.pushCommand(cmd);
    }
    if (e.key === "r") {
      game = load();
    }
  });

  const render = () => {
    const [pieceRowCount, pieceColCount] = game.pieceSize();
    const piece = new Uint8Array(
      memory.buffer,
      game.pieceCellsPtr(),
      pieceRowCount * pieceColCount
    );
    const [pieceX, pieceY] = game.pieceCoord();
    const [dropX, dropY] = game.dropCoord();
    const cells = new Uint8Array(
      memory.buffer,
      game.boardCellsPtr(),
      ROW_COUNT * COL_COUNT
    );
    ctx.fillStyle = "#000";
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    [...new Array(ROW_COUNT * COL_COUNT)].forEach((_, i) => {
      const x = i % COL_COUNT;
      const y = Math.floor(i / COL_COUNT);
      ctx.strokeStyle = "#333";
      ctx.strokeRect(x * CELL_SIZE, y * CELL_SIZE, CELL_SIZE, CELL_SIZE);
    });

    cells.forEach((cell, i) => {
      const x = i % COL_COUNT;
      const y = Math.floor(i / COL_COUNT);
      if (cell !== 0) {
        block(ctx, x, y, cell);
      }
    });

    piece.forEach((cell, i) => {
      const x = i % pieceColCount;
      const y = Math.floor(i / pieceColCount);
      if (cell !== 0) {
        ctx.fillStyle = "rgba(255, 255, 255, 0.1)";
        ctx.fillRect(
          (x + dropX) * CELL_SIZE,
          (y + dropY) * CELL_SIZE,
          CELL_SIZE,
          CELL_SIZE
        );
      }
    });

    piece.forEach((cell, i) => {
      const x = i % pieceColCount;
      const y = Math.floor(i / pieceColCount);
      if (cell !== 0) {
        block(ctx, x + pieceX, y + pieceY, cell);
      }
    });

    hud.textContent = `Score: ${game.score}`;
  };

  const loop = (timestamp) => {
    const delta = (timestamp - lastTickTime) / 1000;
    lastTickTime = timestamp;
    game.update(delta);
    render();
    if (game.gameOver) {
      hud.textContent = `Game Over! Score: ${game.score}`;
    }
    window.requestAnimationFrame(loop);
  };
  loop(0);
});
