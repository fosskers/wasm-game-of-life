import { Universe, Cell } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

const CELL_SIZE = 5;
const GRID_COLOUR = "#CCCCCC";
const DEAD_COLOUR = "#FFFFFF";
const ALIVE_COLOUR = "#000000";

let universe = Universe.new();
const width = universe.width();
const height = universe.height();

const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;
const ctx = canvas.getContext("2d");

const bitIsSet = (n, arr) => {
    const byte = Math.floor(n / 8);
    const mask = 1 << (n % 8);
    return (arr[byte] & mask) === mask;
};

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOUR;

    // Vertical lines.
    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
};

const getIndex = (row, column) => {
    return row * width + column;
};

const drawCells = () => {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height / 8);

    ctx.beginPath();

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);

            ctx.fillStyle = bitIsSet(idx, cells) ? ALIVE_COLOUR : DEAD_COLOUR;

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    ctx.stroke();
};

let animationId = null;

const renderLoop = () => {
    universe.tick();

    drawGrid();
    drawCells();

    animationId = requestAnimationFrame(renderLoop);
};

const isPaused = () => {
    return animationId === null;
};

const playPauseButton = document.getElementById("play-pause");

const play = () => {
    playPauseButton.textContent = "Pause";
    renderLoop();
};

const pause = () => {
    playPauseButton.textContent = "Play";
    cancelAnimationFrame(animationId);
    animationId = null;
};

playPauseButton.addEventListener("click", event => {
    if (isPaused()) {
        play();
    } else {
        pause();
    }
});

const resetButton = document.getElementById("reset-universe");

resetButton.addEventListener("click", event => {
    universe = Universe.new();
});

canvas.addEventListener("click", event => {
    const boundingRect = canvas.getBoundingClientRect();
    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;
    const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
    const canvasTop = (event.clientY - boundingRect.top) * scaleY;
    const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
    const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);
    universe.toggle_cell(row, col);
    drawGrid();
    drawCells();
});

drawGrid();
drawCells();
play();
