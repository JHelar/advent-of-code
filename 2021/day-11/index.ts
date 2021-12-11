// INPUT URL: https://adventofcode.com/2021/day/11/input
import {
  Image,
  TerminalCanvas,
} from "https://deno.land/x/terminal@0.1.0-dev.3/src/mod.ts";
import {
  createRenderer,
  makeColor,
  PALETTE,
  setBackground,
  sleep,
} from "../renderer/render.ts";
type Tile = number;
type Position = [number, number];
type Grid = Tile[][];
// Row = Octopus energy level
// Column = Grid position
type Ocopuses = Record<string, Position>[];

const getAdjacentPositions = ([x, y]: Position) =>
  [
    [x + 1, y],
    [x - 1, y],
    [x, y + 1],
    [x, y - 1],
    [x + 1, y + 1],
    [x - 1, y + 1],
    [x + 1, y - 1],
    [x - 1, y - 1],
  ] as Position[];

const getGridTile = ([x, y]: Position, grid: Grid): Tile | null =>
  grid[y]?.[x] ?? null;
const setGridTile = ([x, y]: Position, tile: Tile, grid: Grid): Tile | null =>
  grid[y][x] = tile;
const getGridKey = ([x, y]: Position) => `(${x},${y})`;

const runStep = (octopuses: Ocopuses, grid: Grid): number => {
  const exploding: Position[] = Object.values(octopuses[9]);
  const allExploding = [...exploding];
  let flashes = exploding.length;
  for (
    let engergyLevel = 9;
    engergyLevel >= 0;
    engergyLevel--
  ) {
    const positions = octopuses[engergyLevel];
    for (const pos of Object.values(positions)) {
      const tile = getGridTile(pos, grid)!;
      const newTile = tile + 1;
      setGridTile(pos, newTile, grid);
    }
    if (engergyLevel === 9) continue; // Skip moving exploding octopuses
    octopuses[engergyLevel + 1] = octopuses[engergyLevel];
  }

  while (exploding.length) {
    for (const explodePosition of exploding.splice(0)) {
      const adjacent = getAdjacentPositions(explodePosition);

      for (const adjacentPosition of adjacent) {
        const tile = getGridTile(adjacentPosition, grid);
        const key = getGridKey(adjacentPosition);

        if (tile === null) continue;
        if (tile > 9) continue; // Allready exploded

        delete octopuses[tile][key]; // Move octopus

        const newTile = tile + 1;
        setGridTile(adjacentPosition, newTile, grid);

        // Exploding
        if (newTile > 9) {
          exploding.push(adjacentPosition);
          allExploding.push(adjacentPosition);
          flashes++;
        } else {
          octopuses[newTile][key] = adjacentPosition;
        }
      }
    }
  }

  // Reset all exploding octopuses
  const fresh: Record<string, Position> = {};
  for (const position of allExploding) {
    setGridTile(position, 0, grid);
    fresh[getGridKey(position)] = position;
  }
  octopuses[0] = fresh;
  return flashes;
};

const getOcopusGrid = async () => {
  const inputString = await Deno.readTextFile("./day-11/input.txt");
  const grid = inputString.split("\n").map((line) =>
    line.trim().split("").map(Number)
  ) as Grid;

  const octopuses: Ocopuses = Array(10).fill(0).map(() => ({}));

  for (let y = 0; y < grid.length; y++) {
    for (let x = 0; x < grid[y].length; x++) {
      const position: Position = [x, y];
      const key = getGridKey(position);
      const tile = getGridTile(position, grid)!;

      octopuses[tile][key] = position;
    }
  }

  return {
    grid,
    octopuses,
  };
};

const createOctopusImage = (energyLevel: Tile): Image => {
  const { r, g, b } = PALETTE.YELLOW;
  const x = energyLevel / 10;
  const color = makeColor(
    Math.round(r * x),
    Math.round(g * x),
    Math.round(b * x),
  );
  return {
    width: 7,
    height: 7,
    pixels: [
      // top padding
      Array(7).fill(PALETTE.BLUE_DARK),
      // top contour
      [
        ...Array(2).fill(PALETTE.BLUE_DARK),
        ...Array(3).fill(PALETTE.BLACK),
        ...Array(2).fill(PALETTE.BLUE_DARK),
      ],
      // middle
      [
        ...Array(1).fill(PALETTE.BLUE_DARK),
        PALETTE.BLACK,
        ...Array(3).fill(color),
        PALETTE.BLACK,
        ...Array(1).fill(PALETTE.BLUE_DARK),
      ],
      // Bottom contour
      [
        ...Array(2).fill(PALETTE.BLUE_DARK),
        ...Array(3).fill(PALETTE.BLACK),
        ...Array(2).fill(PALETTE.BLUE_DARK),
      ],
      // Legs
      [
        PALETTE.BLUE_DARK,
        PALETTE.BLACK,
        PALETTE.BLUE_DARK,
        PALETTE.BLACK,
        PALETTE.BLUE_DARK,
        PALETTE.BLACK,
        PALETTE.BLUE_DARK,
      ],
      [
        PALETTE.BLACK,
        PALETTE.BLUE_DARK,
        PALETTE.BLUE_DARK,
        PALETTE.BLACK,
        PALETTE.BLUE_DARK,
        PALETTE.BLUE_DARK,
        PALETTE.BLACK,
      ],
      // padding bottom
      Array(7).fill(PALETTE.BLUE_DARK),
    ],
  };
};

const renderGrid = async (grid: Grid, canvas: TerminalCanvas) => {
  for (let y = 0; y < grid.length; y++) {
    for (let x = 0; x < grid[y].length; x++) {
      const energyLevel = getGridTile([x, y], grid)!;
      let octopus = createOctopusImage(energyLevel);
      if (energyLevel === 0) {
        octopus = createOctopusImage(10);
      }
      canvas.drawImage(
        x + octopus.width * x,
        y + octopus.height * y,
        octopus,
      );
    }
  }
  await canvas.render();
};

export const part1 = async () => {
  const { grid, octopuses } = await getOcopusGrid();
  const steps = 100;
  let flashes = 0;
  for (let step = 0; step < steps; step++) {
    flashes += runStep(octopuses, grid);
  }
  return flashes;
};

export const part2 = async () => {
  const { grid, octopuses } = await getOcopusGrid();
  const goalFlashes = grid.length * grid[0].length;
  let steps = 1;
  while (goalFlashes !== runStep(octopuses, grid)) {
    steps++;
  }
  return steps;
};

export const render = async () => {
  const { grid, octopuses } = await getOcopusGrid();
  const canvas = await createRenderer();
  setBackground(PALETTE.BLUE_DARK);
  canvas.clear();
  const goalFlashes = grid.length * grid[0].length;
  while (goalFlashes !== runStep(octopuses, grid)) {
    await renderGrid(grid, canvas);
    await sleep(150);
  }
  await renderGrid(grid, canvas);
  await sleep(150);
  runStep(octopuses, grid);
  await renderGrid(grid, canvas);
  return 0;
};
