// INPUT URL: https://adventofcode.com/2021/day/1/input
const getDepths = async () => {
  const inputString = await Deno.readTextFile("./day-1/input.txt");
  const depths = inputString.split("\n").map(Number);
  return depths;
};

export const part1 = async () => {
  const depths = await getDepths();
  let previous = Infinity;
  let count = 0;

  for (const depth of depths) {
    if (depth > previous) count++;
    previous = depth;
  }

  return count;
};

export const part2 = async () => {
  const depths = await getDepths();
  let previous = Infinity;
  let count = 0;

  for (let i = 0; i < depths.length; i++) {
    const sum = depths[i] + depths[i + 1] + depths[i + 2];
    if (sum > previous) count++;
    previous = sum;
  }

  return count;
};
