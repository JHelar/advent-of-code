// INPUT URL: https://adventofcode.com/2021/day/22/input
import { sumOf } from "https://deno.land/std@0.117.0/collections/mod.ts";
type Range = [from: number, to: number];
type Range3D = [x: Range, y: Range, z: Range];
type RebootStep = {
  state: State;
  range: Range3D;
};

const enum State {
  ON = "on",
  OFF = "off",
}

class Cuboid {
  xMin: number;
  xMax: number;
  yMin: number;
  yMax: number;
  zMin: number;
  zMax: number;
  state: State;
  constructor([xRange, yRange, zRange]: Range3D, state: State) {
    this.xMin = xRange[0];
    this.xMax = xRange[1];
    this.yMin = yRange[0];
    this.yMax = yRange[1];
    this.zMin = zRange[0];
    this.zMax = zRange[1];
    this.state = state;
  }

  public volume() {
    return ((1 + this.xMax - this.xMin) * (1 + this.yMax - this.yMin) *
      (1 + this.zMax - this.zMin));
  }

  public isOverlapping(otherCuboid: Cuboid) {
    return otherCuboid.xMin <= this.xMax && this.xMin <= otherCuboid.xMax &&
      otherCuboid.yMin <= this.yMax && this.yMin <= otherCuboid.yMax &&
      otherCuboid.zMin <= this.zMax && this.zMin <= otherCuboid.zMax;
  }

  public split(cuttingCuboid: Cuboid) {
    const splitCuboids: Cuboid[] = [];
    if (cuttingCuboid.xMin > this.xMin) {
      const range: Range3D = [[this.xMin, cuttingCuboid.xMin - 1], [
        this.yMin,
        this.yMax,
      ], [this.zMin, this.zMax]];
      splitCuboids.push(
        new Cuboid(range, this.state),
      );
    }
    if (cuttingCuboid.xMax < this.xMax) {
      const range: Range3D = [[cuttingCuboid.xMax + 1, this.xMax], [
        this.yMin,
        this.yMax,
      ], [this.zMin, this.zMax]];
      splitCuboids.push(
        new Cuboid(
          range,
          this.state,
        ),
      );
    }

    const middleXRange: Range = [
      Math.max(this.xMin, cuttingCuboid.xMin),
      Math.min(this.xMax, cuttingCuboid.xMax),
    ];
    if (cuttingCuboid.yMin > this.yMin) {
      const range: Range3D = [
        middleXRange,
        [this.yMin, cuttingCuboid.yMin - 1],
        [
          this.zMin,
          this.zMax,
        ],
      ];
      splitCuboids.push(
        new Cuboid(range, this.state),
      );
    }
    if (cuttingCuboid.yMax < this.yMax) {
      const range: Range3D = [
        middleXRange,
        [cuttingCuboid.yMax + 1, this.yMax],
        [
          this.zMin,
          this.zMax,
        ],
      ];
      splitCuboids.push(
        new Cuboid(range, this.state),
      );
    }

    const middleYRange: Range = [
      Math.max(this.yMin, cuttingCuboid.yMin),
      Math.min(this.yMax, cuttingCuboid.yMax),
    ];
    if (cuttingCuboid.zMin > this.zMin) {
      const range: Range3D = [middleXRange, middleYRange, [
        this.zMin,
        cuttingCuboid.zMin - 1,
      ]];
      splitCuboids.push(
        new Cuboid(range, this.state),
      );
    }
    if (cuttingCuboid.zMax < this.zMax) {
      const range: Range3D = [middleXRange, middleYRange, [
        cuttingCuboid.zMax + 1,
        this.zMax,
      ]];
      splitCuboids.push(
        new Cuboid(range, this.state),
      );
    }

    return splitCuboids;
  }
}

const parseRange = (rangeStr: string): Range =>
  rangeStr.split("..").map(Number) as Range;

const parse3DRange = (rangeStr: string): Range3D =>
  rangeStr.split(",").map((range) =>
    parseRange(range.split("=")[1])
  ) as Range3D;

const createCuboids = (steps: RebootStep[]) =>
  steps.map<Cuboid>(({ range, state }) => new Cuboid(range, state));

const executeSteps = (steps: Cuboid[]) => {
  let lit: Cuboid[] = [];

  steps.forEach((step) => {
    const newLitCuboids: Cuboid[] = [];
    lit.forEach((cuboid) => {
      if (cuboid.isOverlapping(step)) {
        const cutCuboids = cuboid.split(step);
        newLitCuboids.push(...cutCuboids);
      } else {
        newLitCuboids.push(cuboid);
      }
    });
    if (step.state === State.ON) {
      newLitCuboids.push(step);
    }
    lit = newLitCuboids;
  });

  return lit;
};

const getSteps = async () => {
  const inputString = await Deno.readTextFile("./day-22/input.txt");
  const rebootSteps = inputString.split("\n").map<RebootStep>((line) => {
    const [state, coordinateRanges] = line.trim().split(" ");
    return {
      state: state as State,
      range: parse3DRange(coordinateRanges),
    };
  });
  return rebootSteps;
};

export const part1 = async () => {
  const rebootSteps = await getSteps();
  let cubes = createCuboids(rebootSteps).filter((step) =>
    ![step.xMax, step.xMin, step.yMax, step.yMin, step.zMax, step.zMin].some(
      (el) => Math.abs(el) > 50,
    )
  );
  cubes = executeSteps(cubes);

  return sumOf(cubes, (cuboid) => cuboid.volume());
};

export const part2 = async () => {
  const rebootSteps = await getSteps();
  let cubes = createCuboids(rebootSteps);
  cubes = executeSteps(cubes);

  return sumOf(cubes, (cuboid) => cuboid.volume());
};
