// INPUT URL: https://adventofcode.com/2021/day/9/input
import {
  createRenderer,
  drawPixel,
  makeColor,
  PALETTE,
  renderToScreen,
  sleep,
} from "../renderer/render.ts";
type Tile = {
  value: number;
  position: Position;
  filled?: number;
};
type Row = Array<Tile>;
type Map = Array<Row>;
type Position = [number, number];
type Basin = {
  tile: Tile;
  size: number;
};

const getMapTile = ([x, y]: Position, map: Map): Tile | undefined =>
  map[y] !== undefined ? map[y][x] : undefined;

const getMap = async () => {
  const inputString = await Deno.readTextFile("./day-9/input.txt");
  const map = inputString.split("\n").map((row, y) => {
    const trimmed = row.trim();
    const splitted = trimmed.split("");
    const casted = splitted.map((num, x) => ({
      value: parseInt(num, 10),
      position: [x, y],
    }));
    return casted as Row;
  }) as Map;
  return map;
};

const getBasins = (map: Map) => {
  const columnCount = map[0].length;
  const rowCount = map.length;

  const basins: Array<Basin> = [];

  for (let y = 0; y < rowCount; y++) {
    for (let x = 0; x < columnCount; x++) {
      const { value } = getMapTile([x, y], map)!;

      const topTile = getMapTile([x, y - 1], map)?.value ?? Infinity;
      const bottomTile = getMapTile([x, y + 1], map)?.value ?? Infinity;
      const leftTile = getMapTile([x - 1, y], map)?.value ?? Infinity;
      const rightTile = getMapTile([x + 1, y], map)?.value ?? Infinity;

      if (
        value < topTile &&
        value < bottomTile &&
        value < leftTile &&
        value < rightTile
      ) {
        basins.push({
          tile: {
            value,
            position: [x, y],
          },
          size: 1,
        });
      }
    }
  }

  return basins;
};

const floodFillFrom = (
  [x, y]: Position,
  map: Map,
  fillValue: number,
  order: Array<Tile> = [],
  basinFirst = false,
): number => {
  if (basinFirst) {
    map[y][x].filled = -1;
  } else {
    map[y][x].filled = fillValue;
  }
  order.push(map[y][x]);

  const topPos: Position = [x, y - 1];
  const bottomPos: Position = [x, y + 1];
  const rightPos: Position = [x + 1, y];
  const leftPos: Position = [x - 1, y];

  let filled = 1;
  for (const pos of [topPos, bottomPos, rightPos, leftPos]) {
    const tile = getMapTile(pos, map);
    if (
      tile !== undefined && tile.value < 9 && tile.filled === undefined
    ) {
      filled += floodFillFrom(pos, map, fillValue, order);
    }
  }
  return filled;
};

export const part1 = async () => {
  const map = await getMap();
  const basins = getBasins(map);

  return basins.reduce((sum, { tile: { value } }) => sum + value + 1, 0);
};

export const part2 = async () => {
  const map = await getMap();
  const basins = getBasins(map);

  const sizes: number[] = [];
  for (let i = 0; i < basins.length; i++) {
    const basin = basins[i];
    const size = floodFillFrom(basin.tile.position, map, i);
    sizes.push(size);
  }

  const topThree = sizes.sort((a, b) => b - a).slice(0, 3);
  return topThree.reduce((sum, size) => sum * size, 1);
};

export const render = async () => {
  const map = await getMap();
  const basins = getBasins(map);

  await createRenderer(100, 100);
  for (const row of map) {
    for (const tile of row) {
      const [x, y] = tile.position;
      const color = makeColor(
        93 * (1 + (tile.value / 9)),
        93 * (1 + (tile.value / 9)),
        93 * (1 + (tile.value / 9)),
      );
      drawPixel(x, y, color);
    }
  }

  await renderToScreen();
  await sleep(50);
  for (const { tile: { position: [x, y] } } of basins) {
    // renderer.setCell(x, y, "X", PALETTE.YELLOW, PALETTE.BLUE_DARK);
    drawPixel(x, y, PALETTE.YELLOW);
    await renderToScreen();
    await sleep(5);
  }

  for (const { tile: { position } } of basins) {
    // renderer.setCell(x, y, "X", PALETTE.YELLOW, PALETTE.BLUE_DARK);
    const order: Array<Tile> = [];
    floodFillFrom(position, map, 0, order, true);
    for (const tile of order) {
      if (tile.filled !== undefined) {
        const [x, y] = tile.position;
        if (tile.filled === -1) {
          drawPixel(x, y, PALETTE.YELLOW);
        } else {
          drawPixel(x, y, PALETTE.BLUE_LIGHT);
        }
        await renderToScreen();
        await sleep(5);
      }
    }
  }
};
