import { Universe, Cell } from "game-of-life";
import { memory } from "game-of-life/game_of_life_bg";
import { Fps } from "./fps.js";

let animationId = null;
let tickCount = 1;
let fpsCounter = new Fps();
const CELL_SIZE = 5; // px
const GRID_COLOUR = "#CCCCCC";
const ALIVE_COLOUR = "#FFFFFF";
const DEAD_COLOUR = "#000000";

const universe = Universe.new();
const width = universe.width();
const height = universe.height();

const ticks = document.getElementById("ticks");
const reset = document.getElementById("reset");
const clear = document.getElementById("clear");
const playPauseButton = document.getElementById("play-pause"); 
const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

const drawGrid = () => {
  ctx.beginPath();
  ctx.strokStyle = GRID_COLOUR;

  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
  }

  for (let j = 0; j <= height; j++) {
    ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
  }

  ctx.stroke();
}

const getIndex = (row, column) => {
  return row * width + column;
}

const bitIsSet = (n, arr) => {
  const byte = Math.floor(n / 8);
  const mask = 1 << (n % 8);
  return (arr[byte] & mask) === mask;
}

const drawCells = () => {
  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, (width * height) );

  ctx.beginPath();

  ctx.fillStyle = ALIVE_COLOUR;
  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);

      if (bitIsSet(idx, cells)) {
        continue;
      }

      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }
  
  ctx.fillStyle = DEAD_COLOUR;
  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);
      if (!bitIsSet(idx, cells)) {
        continue;
      }

      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }
  ctx.stroke();
}

const isPaused = () => {
  return animationId === null;
};

const play = () => {
  playPauseButton.textContent = "pause";
  renderLoop();
}

const pause = () => {
  playPauseButton.textContent = "play"
  cancelAnimationFrame(animationId);
  animationId = null;
}

playPauseButton.addEventListener("click", event => {
  if (isPaused()) {
    play();
  } else {
    pause();
  }
});

reset.addEventListener("click", event => {
  universe.reset();
  drawGrid();
  drawCells();
});

clear.addEventListener("click", event => {
  universe.clear();
  drawGrid();
  drawCells();
});

canvas.addEventListener("click", event => {
  const boundingRect = canvas.getBoundingClientRect();

  const scaleX = canvas.width / boundingRect.width;
  const scaleY = canvas.height / boundingRect.height;

  const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
  const canvasTop = (event.clientY - boundingRect.top) * scaleY;

  const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
  const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width -1);

  if (event.shiftKey) {
    universe.toggle_cell(row, col);
    universe.toggle_cell(mod(row+1, width), col);
    universe.toggle_cell(mod(row-1, width), col);
  } else if (event.altKey) {
    universe.toggle_cell(row, col);
    universe.toggle_cell(mod(row+1, width), mod(col+1, height));
    universe.toggle_cell(mod(row+1, width), mod(col-1, height));
    universe.toggle_cell(row,  mod(col+1, height));
    universe.toggle_cell(mod(row-1, width), col);
  
  } else {
    universe.toggle_cell(row, col);
  }
  drawGrid();
  drawCells();
});

function mod(n, m) {
  return ((n % m) + m) % m;
}

const universeCycles = () => {
  for (let i = 0; i < tickCount; i++) {
    universe.tick();
  }
}

ticks.addEventListener("change", event => {
  tickCount = ticks.value;
});

const renderLoop = () => {
  fpsCounter.render();

  universeCycles();
  drawGrid();
  drawCells();

  animationId = requestAnimationFrame(renderLoop);
};


play();
