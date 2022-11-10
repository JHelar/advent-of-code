// INPUT URL: https://adventofcode.com/2021/day/8/input
const enum DIGIT_NAMES {
  ZERO = "zero",
  ONE = "one",
  TWO = "two",
  THREE = "three",
  FOUR = "four",
  FIVE = "five",
  SIX = "six",
  SEVEN = "seven",
  EIGHT = "eight",
  NINE = "nine",
}

const sortDigit = (digit: Digit) => digit.sort((a, b) => a.localeCompare(b));

const parseDigits = (digits: string) =>
  digits.trim().split(" ").map((digit) => digit.split("") as Digit);

const parseEntry = (entry: string) => {
  const [patterns, output] = entry.split(" | ");
  return [
    parseDigits(patterns).sort((a, b) =>
      isUniqueDigit(a) ? 1 : isUniqueDigit(b) ? -1 : 0
    ),
    parseDigits(output),
  ] as Entry;
};

const createIsDigit = (count: number) =>
  (digit: Digit) => digit.length === count;
const isUniqueDigit = (digit: Digit) =>
  [DIGIT_NAMES.ONE, DIGIT_NAMES.FOUR, DIGIT_NAMES.SEVEN, DIGIT_NAMES.EIGHT]
    .some((digitName) => createIsDigit(SEGMENT_COUNTS[digitName])(digit));

const SEGMENTS: Record<DIGIT_NAMES, Digit> = {
  [DIGIT_NAMES.ZERO]: sortDigit(["a", "c", "b", "e", "f", "g"]),
  [DIGIT_NAMES.ONE]: sortDigit(["c", "f"]),
  [DIGIT_NAMES.TWO]: sortDigit(["a", "c", "d", "e", "g"]),
  [DIGIT_NAMES.THREE]: sortDigit(["a", "c", "d", "f", "g"]),
  [DIGIT_NAMES.FOUR]: sortDigit(["c", "b", "d", "f"]),
  [DIGIT_NAMES.FIVE]: sortDigit(["a", "b", "d", "f", "g"]),
  [DIGIT_NAMES.SIX]: sortDigit(["a", "b", "d", "e", "f", "g"]),
  [DIGIT_NAMES.SEVEN]: sortDigit(["a", "c", "f"]),
  [DIGIT_NAMES.EIGHT]: sortDigit(["a", "b", "c", "d", "e", "f", "g"]),
  [DIGIT_NAMES.NINE]: sortDigit(["a", "b", "c", "d", "f", "g"]),
};

const SEGMENT_COUNTS: Record<DIGIT_NAMES, number> = {
  [DIGIT_NAMES.ZERO]: 6,
  [DIGIT_NAMES.ONE]: 2, // Unique
  [DIGIT_NAMES.TWO]: 5,
  [DIGIT_NAMES.THREE]: 5,
  [DIGIT_NAMES.FOUR]: 4, // Unique
  [DIGIT_NAMES.FIVE]: 5,
  [DIGIT_NAMES.SIX]: 6,
  [DIGIT_NAMES.SEVEN]: 3, // Unique
  [DIGIT_NAMES.EIGHT]: 7, // Unique
  [DIGIT_NAMES.NINE]: 6,
};

const NAME_TO_VALUE: Record<DIGIT_NAMES, string> = {
  [DIGIT_NAMES.ZERO]: "0",
  [DIGIT_NAMES.ONE]: "1", // Unique
  [DIGIT_NAMES.TWO]: "2",
  [DIGIT_NAMES.THREE]: "3",
  [DIGIT_NAMES.FOUR]: "4", // Unique
  [DIGIT_NAMES.FIVE]: "5",
  [DIGIT_NAMES.SIX]: "6",
  [DIGIT_NAMES.SEVEN]: "7", // Unique
  [DIGIT_NAMES.EIGHT]: "8", // Unique
  [DIGIT_NAMES.NINE]: "9",
};

type Segment = "a" | "b" | "c" | "d" | "e" | "f" | "g";
type Digit = Array<Segment>;
type Entry = [Array<Digit>, Array<Digit>];
type Entries = Array<Entry>;
type SegmentWires = Record<Segment, Segment>;

const getEntries = async () => {
  const inputString = await Deno.readTextFile("./day-8/input.txt");
  const entries = inputString.split("\n").map(parseEntry);
  return entries;
};

const createMapDigit = (correctDigit: Digit) => {
  return (digit: Digit, entry: Entry, wires: SegmentWires) => {
    const wireSegments = digit.map((segment) => wires[segment]).filter(Boolean);

    const isAllSegmentsCorrect = wireSegments.every((segment) =>
      correctDigit.includes(segment)
    );

    if (!isAllSegmentsCorrect) return null;

    // If map contains correct count of segments
    if (wireSegments.length === correctDigit.length) return wires;

    const segmentNotSet = digit.find((segment) => wires[segment] === undefined);
    if (!segmentNotSet) return null;

    const correctSegmentsNotSet = correctDigit.filter((segment) =>
      !wireSegments.includes(segment)
    );

    for (const correctSegment of correctSegmentsNotSet) {
      if (Object.values(wires).includes(correctSegment)) continue;
      const newWires = mapEntry(entry, {
        ...wires,
        [segmentNotSet]: correctSegment,
      });
      if (newWires) {
        return newWires;
      }
    }

    return null;
  };
};

const DIGIT_MAPS = [
  DIGIT_NAMES.ZERO,
  DIGIT_NAMES.ONE,
  DIGIT_NAMES.TWO,
  DIGIT_NAMES.THREE,
  DIGIT_NAMES.FOUR,
  DIGIT_NAMES.FIVE,
  DIGIT_NAMES.SIX,
  DIGIT_NAMES.SEVEN,
  DIGIT_NAMES.EIGHT,
  DIGIT_NAMES.NINE,
].map((name) => ({
  isDigit: createIsDigit(SEGMENT_COUNTS[name]),
  map: createMapDigit(SEGMENTS[name]),
}));

const mapEntry = (entry: Entry, wires: SegmentWires): SegmentWires | null => {
  const [patterns] = entry;
  let digitSuccess = false;

  for (const digit of patterns) {
    digitSuccess = false;
    const digitHandlers = DIGIT_MAPS.filter(({ isDigit }) => isDigit(digit));
    for (const handler of digitHandlers) {
      const newWires = handler.map(digit, entry, wires);
      if (newWires) {
        wires = newWires;
        digitSuccess = true;
        break;
      }
    }
    if (!digitSuccess) return null;
  }

  return wires;
};

export const part1 = async () => {
  const entries = await getEntries();

  let result = 0;
  for (const entry of entries) {
    const [, output] = entry;
    result += output.reduce(
      (sum, digit) => isUniqueDigit(digit) ? sum + 1 : sum,
      0,
    );
  }

  return result;
};

const decodeEntry = (entry: Entry) => {
  const wires = mapEntry(entry, {} as SegmentWires);
  if (!wires) {
    throw new Error("FAILED");
  }

  const output = entry[1];

  const segments = Object.entries(SEGMENTS);

  const decoded = output.map(
    (
      digit,
    ) => {
      const sorted = sortDigit(digit.map((segment) => wires[segment])).join("");
      const maped = segments.find(([, segment]) => segment.join("") === sorted)
        ?.[0];
      return {
        digit: digit.join(""),
        sorted,
        maped,
        value: maped ? NAME_TO_VALUE[maped as DIGIT_NAMES] : "-1",
      };
    },
  );
  return decoded;
};

export const part2 = async () => {
  const entries = await getEntries();
  let sum = 0;
  for (const entry of entries) {
    const decoded = decodeEntry(entry);
    const value = Number(decoded.reduce((num, { value }) => num + value, ""));
    sum += value;
  }

  return sum;
};
