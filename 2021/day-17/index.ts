// INPUT URL: https://adventofcode.com/2021/day/17/input
import {
  Color,
  TerminalCanvas,
} from "https://deno.land/x/terminal@0.1.0-dev.3/src/mod.ts";
import {
  createRenderer,
  drawPixel,
  mult,
  PALETTE,
  renderToScreen,
  setBackground,
  sleep,
} from "../renderer/render.ts";
type Vector2 = [number, number];
type Position = Vector2;
type Velocity = Vector2;
type Range = Vector2;
type Target = {
  xRange: Range;
  yRange: Range;
};

const isInTarget = (
  [x, y]: Vector2,
  { xRange: [xStart, xEnd], yRange: [yStart, yEnd] }: Target,
) => x >= xStart && x <= xEnd && y <= yStart && y >= yEnd;
const isOvershot = ([, y]: Vector2, { yRange: [, yEnd] }: Target) => y < yEnd;

const add = (one: Vector2, two: Vector2): Vector2 => [
  one[0] + two[0],
  one[1] + two[1],
];
const addDrag = (vel: Vector2): Vector2 => {
  if (vel[0] > 0) return add(vel, [-1, 0]);
  if (vel[0] < 0) return add(vel, [1, 0]);
  return add(vel, [0, 0]);
};
const addGravity = (vel: Vector2): Vector2 => add(vel, [0, -1]);

const getMaxPosition = ([x, y]: Vector2): Vector2 => {
  const maxX = Array(Math.abs(x))
    .fill(0)
    .reduce((sum, _, i) => sum + (x - (x - i)), x);
  const maxY = Array(y)
    .fill(0)
    .reduce((sum, _, i) => sum + (y - (y - i)), y);

  return [maxX, maxY];
};

const simulateThrow = (
  initialPosition: Vector2,
  target: Target,
  initialVector: Vector2 = [0, 0],
): boolean => {
  // Simulate from highest point
  let projectile: Position = initialPosition;
  let velocity: Velocity = initialVector;
  while (!isOvershot(projectile, target)) {
    projectile = add(projectile, velocity);
    velocity = addGravity(addDrag(velocity));
    if (isInTarget(projectile, target)) {
      return true;
    }
  }

  return false;
};

const getTargetInput = async (): Promise<Target> => {
  const inputString = await Deno.readTextFile("./day-17/input.txt");
  const [, areas] = inputString.trim().split("target area: ");
  const [xRange, yRange] = areas
    .split(", ")
    .map((area) => area.split("=")[1].split("..").map(Number) as Range);

  yRange.reverse();
  const target: Target = {
    xRange,
    yRange,
  };

  return target;
};

export const part1 = async () => {
  const target = await getTargetInput();
  const { xRange, yRange } = target;
  let bestY = -Infinity;
  for (let x = 1; x <= xRange[1]; x++) {
    for (
      let y = Math.abs(yRange[0]);
      y <= Math.abs(10 * (yRange[1] - yRange[0]));
      y++
    ) {
      const [maxX, maxY] = getMaxPosition([x, y]);
      if (maxY <= bestY) continue;
      if (simulateThrow([maxX, maxY], target)) {
        bestY = maxY;
      }
    }
  }

  return bestY;
};

export const part2 = async () => {
  const target = await getTargetInput();
  const { xRange, yRange } = target;
  let hits = 0;
  for (let x = 1; x <= xRange[1]; x++) {
    for (let y = yRange[1]; y <= Math.abs(10 * (yRange[1] - yRange[0])); y++) {
      if (simulateThrow([0, 0], target, [x, y])) {
        hits++;
      }
    }
  }

  return hits;
};

export const render = async () => {
  const target = await getTargetInput();
  const { xRange, yRange } = target;
  const canvas = await createRenderer();

  const maxY = await part1();
  const minY = yRange[1];

  const maxX = xRange[1];
  const minX = 0;

  const height = Math.abs(maxY - minY);

  const scale = (1 - ((height - canvas.height) / canvas.height));

  const velocities: Velocity[] = [];
  // for (let x = 1; x <= xRange[1]; x++) {
  //   for (let y = yRange[1]; y <= Math.abs(10 * (yRange[1] - yRange[0])); y++) {
  //     if (simulateThrow([0, 0], target, [x, y])) {
  //       velocities.push([x, y]);
  //     }
  //   }
  // }
  console.log({
    scale,
    height,
    canvas: canvas.height,
  });
};
