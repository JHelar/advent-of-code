// INPUT URL: https://adventofcode.com/2021/day/13/input
import { TerminalCanvas } from "https://deno.land/x/terminal@0.1.0-dev.3/src/mod.ts";
import {
  createRenderer,
  makeColor,
  PALETTE,
  setBackground,
  sleep,
} from "../renderer/render.ts";
const enum Tile {
  EMPTY = 0,
  DOT = 1,
}
type PaperRow = Tile[];
type Paper = PaperRow[];
type Position = [number, number];
type Instruction = Position;
type Instructions = Instruction[];

const getPaper = async () => {
  const inputString = await Deno.readTextFile("./day-13/input.txt");
  const splitted = inputString.split("\n").map((line) => line.trim());
  const instructions: Instructions = [];
  const coordinates: Position[] = [];

  let parseCoordinates = true;
  for (const line of splitted) {
    if (line === "") {
      parseCoordinates = false;
      continue;
    }
    if (parseCoordinates) {
      const [x, y] = line.split(",").map(Number);
      coordinates.push([x, y]);
    } else {
      const [, , instruction] = line.split(" ");
      const [axis, amount] = instruction.split("=");
      if (axis === "y") {
        instructions.push([0, Number(amount)]);
      } else if (axis === "x") {
        instructions.push([Number(amount), 0]);
      } else {
        throw new Error(
          `Unable to parse instruction: ${line}="${instruction}"`,
        );
      }
    }
  }

  const paperWidth =
    coordinates.reduce((max, [x]) => max < x ? x : max, -Infinity) + 1;
  const paperHeight =
    coordinates.reduce((max, [, y]) => max < y ? y : max, -Infinity) + 1;
  const paper: Paper = Array(paperHeight).fill(0).map(() =>
    Array(paperWidth).fill(Tile.EMPTY)
  );

  for (const [x, y] of coordinates) {
    paper[y][x] = Tile.DOT;
  }

  return {
    paper,
    instructions,
  };
};

const zipRows = (row1: PaperRow, row2: PaperRow) =>
  row1.map((row1Tile, i) => row1Tile || row2[i]);

const fold = (paper: Paper, [foldX, foldY]: Instruction) => {
  if (foldY) {
    const rows = paper.splice(foldY + 1);
    paper.splice(foldY, 1);
    let paperY = paper.length - rows.length;
    for (let i = rows.length - 1; i >= 0; i--) {
      const row = rows[i];
      const newRow = zipRows(row, paper[paperY]);
      paper[paperY] = newRow;
      paperY++;
    }
  } else if (foldX) {
    const paperColumns = paper.reduce((columns, row) => {
      for (let col = 0; col < row.length; col++) {
        if (columns[col] === undefined) {
          columns[col] = [];
        }
        columns[col].push(row[col]);
      }
      return columns;
    }, [] as PaperRow[]);

    const foldColumns = paperColumns.splice(foldX + 1);
    paperColumns.splice(foldX, 1);
    let paperX = paperColumns.length - foldColumns.length;

    for (let i = foldColumns.length - 1; i >= 0; i--) {
      const column = foldColumns[i];
      const newColumn = zipRows(column, paperColumns[paperX]);

      for (let y = 0; y < newColumn.length; y++) {
        const tile = newColumn[y];
        paper[y].splice(foldX);
        paper[y][paperX] = tile;
      }
      paperX++;
    }
  }
};

const prepCanvas = async (canvas: TerminalCanvas) => {
  setBackground(PALETTE.BLUE_DARK);
  await canvas.render();
};

const drawPaper = async (
  paper: Paper,
  canvas: TerminalCanvas,
  render = false,
) => {
  if (paper.length > canvas.height) return;
  for (let y = 0; y < canvas.height; y++) {
    for (let x = 0; x < canvas.width; x++) {
      const tile = paper[y]?.[x];
      if (tile !== undefined) {
        const isEmpty = tile === Tile.EMPTY;
        canvas.drawPixel(
          x,
          y,
          isEmpty ? PALETTE.BLUE_DARK : PALETTE.YELLOW,
        );
      }
    }
  }
  if (render) {
    await canvas.render();
    await sleep(500);
  }
};

const drawFold = async (
  [foldX, foldY]: Instruction,
  canvas: TerminalCanvas,
  render = false,
) => {
  if (foldX > canvas.width || foldY > canvas.height) return;
  for (let y = 0; y < canvas.height; y++) {
    for (let x = 0; x < canvas.width; x++) {
      if ((foldX && foldX === x) || (foldY && foldY === y)) {
        canvas.drawPixel(
          x,
          y,
          PALETTE.GREEN_DARK,
        );
      }
    }
  }
  if (render) {
    await canvas.render();
    await sleep(500);
  }
};

export const part1 = async () => {
  const { paper, instructions } = await getPaper();
  fold(paper, instructions[0]);

  return paper.reduce(
    (dots, row) => dots + row.filter((tile) => tile === Tile.DOT).length,
    0,
  );
};

export const part2 = async () => {
  const { paper, instructions } = await getPaper();
  for (const instruction of instructions) {
    fold(paper, instruction);
  }
  const canvas = await createRenderer();
  await prepCanvas(canvas);
  await drawPaper(paper, canvas);
  await canvas.render();

  console.log("");
  return "Press 'q' to quit";
};

export const render = async () => {
  const { paper, instructions } = await getPaper();
  const canvas = await createRenderer();
  await prepCanvas(canvas);
  await drawPaper(paper, canvas, true);
  for (const instruction of instructions) {
    await drawPaper(paper, canvas);
    await drawFold(instruction, canvas, true);

    fold(paper, instruction);
  }
  await drawPaper(paper, canvas);
  await canvas.render();

  console.log("");
  return "Press 'q' to quit";
};
