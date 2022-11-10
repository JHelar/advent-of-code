// INPUT URL: https://adventofcode.com/2021/day/3/input
type BinArray = Array<string>;

const readBin = async (): Promise<BinArray> => {
  const inputString = await Deno.readTextFile("./day-3/input.txt");
  const binArray = inputString.split("\n").map((bin) => bin.replace("\r", ""));
  return binArray;
};

const calcBits = (binArray: BinArray) => {
  const bitCalc = Array(binArray[0].length).fill(0).map(() => ({
    ones: 0,
    zeroes: 0,
  }));

  for (const bin of binArray) {
    for (let i = 0; i < bin.length; i++) {
      const bit = bin[i];
      if (bit === "1") bitCalc[i].ones++;
      else bitCalc[i].zeroes++;
    }
  }

  return bitCalc;
};

export const part1 = async () => {
  const binArray = await readBin();
  const bitCalc = calcBits(binArray);

  const { epsilonRate, gammaRate } = bitCalc.reduce(
    ({ gammaRate, epsilonRate }, { ones, zeroes }) => {
      return {
        gammaRate: `${gammaRate}${Math.max(ones, zeroes) === ones ? "1" : "0"}`,
        epsilonRate: `${epsilonRate}${
          Math.min(ones, zeroes) === ones ? "1" : "0"
        }`,
      };
    },
    { gammaRate: "", epsilonRate: "" },
  );

  const epsilonRateDec = parseInt(epsilonRate, 2);
  const gammaRateDec = parseInt(gammaRate, 2);

  return epsilonRateDec * gammaRateDec;
};

export const part2 = async () => {
  const binArray = await readBin();

  const trie = createTree(binArray);
  const ozygen = calcOzygenWithTrie(trie);
  const scrubber = calcScrubberRateWithTrie(trie);

  return ozygen * scrubber;
};

type Trie = {
  value: number;
  leaf?: string;
  children: Record<string, Trie>;
};
const insert = (bin: string, root: Trie) => {
  let lookAt = root;
  for (const bit of bin) {
    if (bit in lookAt.children) {
      lookAt.children[bit].value++;
      lookAt = lookAt.children[bit];
    } else {
      const trie: Trie = {
        value: 1,
        children: {},
      };
      lookAt.children[bit] = trie;
      lookAt = trie;
    }
  }
  lookAt.leaf = bin;
  return root;
};

const findAll = (pattern: string, root: Trie): Array<string> => {
  if (root.leaf) return [root.leaf];
  const part = pattern[0];
  if (part === "*") {
    return Object.values(root.children).reduce(
      (arr, child) => [...arr, ...findAll(pattern.slice(1), child)],
      [] as string[],
    );
  }
  if (part in root.children) {
    return findAll(pattern.slice(1), root.children[part]);
  }
  return [];
};

const createTree = (binArray: BinArray) => {
  const root = {
    value: -1,
    children: {},
  };
  for (const bin of binArray) {
    insert(bin, root);
  }

  return root;
};

const calcRate = (comp: (ones: number, zeroes: number) => boolean) =>
  (root: Trie) => {
    let lookAt = root;
    while (lookAt.leaf === undefined) {
      const ones = lookAt.children["1"]?.value;
      const zeroes = lookAt.children["0"]?.value;

      if (zeroes === undefined || comp(ones, zeroes)) {
        lookAt = lookAt.children["1"];
      } else {
        lookAt = lookAt.children["0"];
      }
    }

    return parseInt(lookAt.leaf, 2);
  };

const calcScrubberRateWithTrie = calcRate((ones, zeroes) => ones < zeroes);
const calcOzygenWithTrie = calcRate((ones, zeroes) => ones >= zeroes);
