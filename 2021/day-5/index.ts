// INPUT URL: https://adventofcode.com/2021/day/5/input
import { renderFile } from "https://deno.land/x/mustache/mod.ts";

type Point = [number, number];
type Range = [Point, Point];
type Ranges = Array<Range>;
type Tile = number;
type SeaMap = Record<string, Tile>;

const pointToKey = ([x, y]: Point) => `(${x},${y})`;

const isVerticalLine = ([[x1], [x2]]: Range) => x1 === x2;
const isHorizontalLine = ([[, y1], [, y2]]: Range) => y1 === y2;

const getIsOnLine = (range: Range) => {
  const [[x1, y1], [x2, y2]] = range;
  if (isHorizontalLine(range)) return ([, y]: Point) => y === y1;
  if (isVerticalLine(range)) return ([x]: Point) => x === x1;

  const a = (x2 - x1) !== 0 ? (y2 - y1) / (x2 - x1) : 0;
  const m = y1 - a * x1;
  return ([x, y]: Point) => y === a * x + m;
};

const setRangeToMap = (map: SeaMap, range: Range) => {
  const [[x1, y1], [x2, y2]] = range;
  const isOnLine = getIsOnLine(range);

  const xStart = Math.min(x1, x2);
  const yStart = Math.min(y1, y2);

  const xEnd = Math.max(x1, x2);
  const yEnd = Math.max(y1, y2);

  for (let y = yStart; y <= yEnd; y++) {
    for (let x = xStart; x <= xEnd; x++) {
      const point: Point = [x, y];

      const mapKey = pointToKey(point);
      if (isOnLine(point)) {
        map[mapKey] = ++map[mapKey] || 1;
      }
    }
  }
};

const getRanges = async () => {
  const inputString = await Deno.readTextFile("./day-5/input.txt");
  const ranges = inputString.split("\n").reduce((points, line) => {
    const range = line.split(" -> ").map((pointStr) =>
      pointStr.split(",").map(Number) as Point
    );
    return [...points, range as Range];
  }, [] as Ranges);

  return ranges;
};

const printMap = async (map: SeaMap, maxX: number, maxY: number) => {
  const outputContent = await renderFile("./day-5/output.mustache", {
    map: JSON.stringify(map),
    maxX,
    maxY,
    maxValue: Object.values(map).reduce(
      (max, value) => value > max ? value : max,
      0,
    ),
  });
  await Deno.writeTextFile("./day-5/output.html", outputContent);
};

export const part1 = async () => {
  const ranges = await getRanges();
  const map: SeaMap = {};

  for (const range of ranges) {
    if (isHorizontalLine(range) || isVerticalLine(range)) {
      setRangeToMap(map, range);
    }
  }

  const maxCount = Object.values(map).reduce(
    (sum, value) => value > 1 ? sum + 1 : sum,
    0,
  );

  return maxCount;
};

export const part2 = async () => {
  const ranges = await getRanges();
  const map: SeaMap = {};

  let maxX = -Infinity;
  let maxY = -Infinity;

  for (const range of ranges) {
    const [[x1, y1], [x2, y2]] = range;
    const localXMax = Math.max(x1, x2);
    const localYMax = Math.max(y1, y2);

    if (localXMax > maxX) maxX = localXMax;
    if (localYMax > maxY) maxY = localYMax;

    setRangeToMap(map, range);
  }

  const maxCount = Object.values(map).reduce(
    (sum, value) => value > 1 ? sum + 1 : sum,
    0,
  );

  printMap(map, maxX, maxY);

  return maxCount;
};
