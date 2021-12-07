// INPUT URL: https://adventofcode.com/2021/day/7/input
type Position = number;
type Positions = Position[];

const incrementor = () => {
  const mem: Record<string, number> = {};
  return (count: number) => {
    if (count in mem) return mem[count];
    const res = Array(count).fill(0).reduce(
      (sum, _, index) => sum + index + 1,
      0,
    );
    mem[count] = res;
    return res;
  };
};

type IncrementFn = (diff: number) => number;
const calculateFuelConsumption = (
  min: number,
  positions: Positions,
  increment: IncrementFn,
) => {
  return positions.reduce(
    (sum, position) => sum + increment(Math.abs(position - min)),
    0,
  );
};

const getPositions = async (): Promise<Positions> => {
  const inputString = await Deno.readTextFile("./day-7/input.txt");
  const positions = inputString.split(",").map(Number).sort((a, b) => a - b);
  return positions;
};

export const part1 = async () => {
  const positions = await getPositions();
  const max = positions[positions.length - 1];

  let prevFuel = Infinity;

  for (let i = 0; i < max; i++) {
    const fuel = calculateFuelConsumption(i, positions, (n) => n);
    if (fuel > prevFuel) {
      return prevFuel;
    }
    prevFuel = fuel;
  }
};

export const part2 = async () => {
  const positions = await getPositions();
  const max = positions[positions.length - 1];

  let prevFuel = Infinity;

  for (let i = 0; i < max; i++) {
    const fuel = calculateFuelConsumption(i, positions, incrementor());
    if (fuel > prevFuel) {
      return prevFuel;
    }
    prevFuel = fuel;
  }
};
