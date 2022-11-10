// INPUT URL: https://adventofcode.com/2021/day/18/input
type SnailNumber = {
  depth: number;
  value: number;
}[];

const toSnail = (
  line: string,
): SnailNumber => {
  const numbers: SnailNumber = [];
  let depth = 0;

  for (const token of line) {
    if (token === "[") {
      depth++;
      continue;
    }
    if (token === "]") {
      depth--;
      continue;
    }
    const value = parseInt(token);
    if (!isNaN(value)) {
      numbers.push({
        depth,
        value,
      });
      continue;
    }
  }

  return numbers;
};

const add = (
  one: SnailNumber,
  two: SnailNumber,
): SnailNumber => {
  const number: SnailNumber = [];
  number.push(...two);
  number.unshift(...one);

  for (const num of number) {
    num.depth++;
  }
  return number;
};

const explode = (number: SnailNumber) => {
  for (let i = 0; i < number.length; i++) {
    const { value, depth } = number[i];

    if (depth > 4) {
      if (i > 0) {
        number[i - 1].value += value;
      }
      if (i < number.length - 2) {
        number[i + 2].value += number[i + 1].value;
      }
      number.splice(i, 2, { depth: depth - 1, value: 0 });
      return true;
    }
  }
  return false;
};

const split = (number: SnailNumber) => {
  for (let i = 0; i < number.length; i++) {
    const { value, depth } = number[i];
    if (value >= 10) {
      number.splice(i, 1, { depth: depth + 1, value: Math.floor(value / 2) }, {
        depth: depth + 1,
        value: Math.ceil(value / 2),
      });
      return true;
    }
  }
  return false;
};

const reduceSum = (sum: SnailNumber, number: SnailNumber) => {
  const total = add(sum, number);
  while (explode(total) || split(total));
  return total;
};

const calculateMagnitude = (number: SnailNumber) => {
  const total = [...number];
  let i = 0;
  while (total.length > 1) {
    const one = total[i];
    const two = total[i + 1];

    if (one.depth === two.depth) {
      total.splice(i, 2, {
        depth: one.depth - 1,
        value: 3 * one.value + 2 * two.value,
      });
      i = 0;
    } else {
      i = (i + 1) % total.length;
    }
  }

  return total[0].value;
};

const getSnailInput = async (): Promise<SnailNumber[]> => {
  const inputString = await Deno.readTextFile("./day-18/input.txt");
  const numbers = inputString.split("\n").map((line) => toSnail(line.trim()));
  return numbers;
};

export const part1 = async () => {
  const numbers = await getSnailInput();
  const result = numbers.reduce(reduceSum);
  const magnitude = calculateMagnitude(result);
  return magnitude;
};

export const part2 = async () => {
  const numbers = await getSnailInput();
  let bestMagnitude = -Infinity;
  const calcs: Record<string, number> = {};

  for (let i = 0; i < numbers.length; i++) {
    for (let j = 0; j < numbers.length; j++) {
      let one = numbers[i].map((v) => ({ ...v }));
      let two = numbers[j].map((v) => ({ ...v }));

      const key1 = `${i}-${j}`;
      const key2 = `${j}-${i}`;
      if (key1 in calcs || key2 in calcs) continue;
      if (i === j) continue;
      let magnitude = calculateMagnitude(reduceSum(one, two));
      if (magnitude > bestMagnitude) {
        calcs[key1] = magnitude;
        bestMagnitude = magnitude;
      }

      one = numbers[i].map((v) => ({ ...v }));
      two = numbers[j].map((v) => ({ ...v }));
      magnitude = calculateMagnitude(reduceSum(two, one));
      if (magnitude > bestMagnitude) {
        calcs[key2] = magnitude;
        bestMagnitude = magnitude;
      }
    }
  }

  return bestMagnitude;
};
