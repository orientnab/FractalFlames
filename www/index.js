import { Picture } from "fractal-flames";
import { memory } from "fractal-flames/fractal_flames_bg";

const CELL_SIZE = 1;

const picture = Picture.new();
const width = picture.width();
const height = picture.height();

const canvas = document.getElementById("picture-canvas");
// canvas.height = (CELL_SIZE + 1) * height + 1;
// canvas.width = (CELL_SIZE + 1) * width + 1;
canvas.height = (CELL_SIZE) * height + 1;
canvas.width = (CELL_SIZE) * width + 1;

const ctx = canvas.getContext('2d');
// const ctx = canvas.getContext('webgl2');

const getIndex = (row, column) => {
  return row * width + column;
}

const getColor = (val) => {
  // val = Math.min(Math.max(val, 0.0), 1.0);
  let res = Math.floor(((1 - val))*0xFF).toString(16);
  if (res.length < 2) { res = "0" + res };
  return res;
}

const drawCells = () => {
  // const cell_counter_ptr = picture.cell_counter();
  // const cell_counter = new Uint32Array(memory.buffer, cell_counter_ptr, width * height);
  // const cell_color_ptr = picture.cell_color();
  // const cell_color = new Uint8Array(memory.buffer, cell_color_ptr, 3 * width * height);
  const cell_alpha_ptr = picture.cell_alpha();
  const cell_alpha = new Float32Array(memory.buffer, cell_alpha_ptr, width * height);
  const cell_color_ptr = picture.cell_color();
  const cell_color = new Float32Array(memory.buffer, cell_color_ptr, 3 * width * height);

  ctx.beginPath();

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);
      // ctx.fillStyle = "#" + getColor(velx[idx]).repeat(3);
      // ctx.fillStyle = "#" + getColor(velx[idx]) + "ff" + getColor(velx[idx]);
      // console.log(getColor(velx[idx]))

      // if (cell_counter[idx] === 1) {
      //   ctx.fillStyle = "#333333";
      // } else {
      //   ctx.fillStyle = "#cccccc";
      // }
      //

      // console.log(cell_alpha[idx]);
      // console.log("(", cell_color[3*idx],", ",
      //   cell_color[3*idx+1], ", ", cell_color[3*idx+2], ")");

      // ctx.fillStyle = "#" + getColor(cell_alpha[idx]).repeat(3);
      ctx.fillStyle = `rgb(
        ${Math.floor(255 * cell_color[3*idx])},
        ${Math.floor(255 * cell_color[3*idx + 1])},
        ${Math.floor(255 * cell_color[3*idx + 2])}`;
      ctx.fillRect(
        col * (CELL_SIZE) + 1,
        row * (CELL_SIZE) + 1,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }

  ctx.stroke();
};

// let animationId = null;

// const isPaused = () => {
//   return animationId === null;
// }

// const play = () => {
//   renderLoop();
// }

// const pause = () => {
//   cancelAnimationFrame(animationId);
//   animationId = null;
// };


// document.addEventListener("keydown", event => {
//   if (isPaused()) {
//     play();
//   } else {
//     pause();
//   }
// });

// const renderLoop = () => {
//   // drawGrid();
//   velocity_field.tick();

//   drawCells();

//   animationId = requestAnimationFrame(renderLoop);
// }

// drawGrid();
picture.paint();
drawCells();
// play();
