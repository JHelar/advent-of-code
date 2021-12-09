// INPUT URL: https://adventofcode.com/2021/day/9/input
type Tile = {
  value: number;
  filled?: number;
};
type Row = Array<Tile>;
type Map = Array<Row>;
type Position = [number, number];
type Basin = {
  tile: Tile;
  position: Position;
  size: number;
};

const getMapTile = ([x, y]: Position, map: Map): Tile | undefined =>
  map[y] !== undefined ? map[y][x] : undefined;

const getMap = async () => {
  const inputString = await Deno.readTextFile("./day-9/input.txt");
  const map = inputString.split("\n").map((row) => {
    const trimmed = row.trim();
    const splitted = trimmed.split("");
    const casted = splitted.map((num) => ({
      value: parseInt(num, 10),
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
          },
          position: [x, y],
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
): number => {
  map[y][x].filled = fillValue;

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
      filled += floodFillFrom(pos, map, fillValue);
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
    const size = floodFillFrom(basin.position, map, i);
    sizes.push(size);
  }

  const topThree = sizes.sort((a, b) => b - a).slice(0, 3);
  return topThree.reduce((sum, size) => sum * size, 1);
};
