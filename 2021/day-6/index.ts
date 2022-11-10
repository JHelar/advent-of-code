// INPUT URL: https://adventofcode.com/2021/day/6/input
type Fish = number;
type Fishes = Fish[];

const simulateFishes = async (days: number) => {
  const inputString = await Deno.readTextFile("./day-6/input.txt");
  const input = inputString.split(",").map(Number) as Fishes;

  const fish: number[] = Array(9).fill(0);
  for (const i of input) {
    fish[i] += 1;
  }

  for (let day = 0; day < days; day++) {
    const amtNew = fish[0];

    for (let i = 1; i <= 8; i++) {
      fish[i - 1] = fish[i];
    }

    fish[6] += amtNew;
    fish[8] = amtNew;
  }

  return fish.reduce((sum, fish) => sum + fish, 0);
};

export const part1 = async () => {
  const res = await simulateFishes(80);
  return res;
};

export const part2 = async () => {
  const res = await simulateFishes(256);
  return res;
};
