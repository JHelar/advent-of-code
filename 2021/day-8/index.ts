// INPUT URL: https://adventofcode.com/2021/day/8/input
/**
 *
   0:      1:      2:      3:      4:
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

  5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg
 */

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

const SEGMENTS: Record<DIGIT_NAMES, Digit> = {
  [DIGIT_NAMES.ZERO]: ["a", "c", "b", "e", "f", "g"],
  [DIGIT_NAMES.ONE]: ["c", "f"],
  [DIGIT_NAMES.TWO]: ["a", "c", "d", "e", "g"],
  [DIGIT_NAMES.THREE]: ["a", "c", "d", "f", "g"],
  [DIGIT_NAMES.FOUR]: ["c", "b", "d", "f"],
  [DIGIT_NAMES.FIVE]: ["a", "b", "d", "f", "g"],
  [DIGIT_NAMES.SIX]: ["a", "b", "d", "e", "f", "g"],
  [DIGIT_NAMES.SEVEN]: ["a", "c", "f"],
  [DIGIT_NAMES.EIGHT]: ["a", "b", "c", "d", "e", "f", "g"],
  [DIGIT_NAMES.NINE]: ["a", "b", "c", "d", "f", "g"],
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

const parseDigits = (digits: string) =>
  digits.trim().split(" ").map((digit) => digit.split("") as Digit);

const parseEntry = (entry: string) => {
  const [patterns, output] = entry.split(" | ");
  return [parseDigits(patterns), parseDigits(output)] as Entry;
};

const createIsDigit = (count: number) =>
  (digit: Digit) => digit.length === count;
const isUniqueDigit = (digit: Digit) =>
  [DIGIT_NAMES.ONE, DIGIT_NAMES.FOUR, DIGIT_NAMES.SEVEN, DIGIT_NAMES.EIGHT]
    .some((digitName) => createIsDigit(SEGMENT_COUNTS[digitName])(digit));

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
  //   const entries = await getEntries();

  let result = 0;
  //   for (const entry of entries) {
  //     const [, output] = entry;
  //     result += output.reduce(
  //       (sum, digit) => isUniqueDigit(digit) ? sum + 1 : sum,
  //       0,
  //     );
  //   }

  return result;
};

const decodeEntry = (entry: Entry) => {
  const wires = mapEntry(entry, {} as SegmentWires);
  if (!wires) {
    console.log("FAIL");
    return null;
  }

  const output = entry[1];

  const segments = Object.entries(SEGMENTS);

  const decoded = output.map(
    (
      digit,
    ) => {
      const decodedDigit = segments.find(([, segments]) =>
        digit.every((segment) => segments.includes(wires[segment]))
      );
      if (decodedDigit) {
        return NAME_TO_VALUE[decodedDigit[0] as DIGIT_NAMES];
      }
      return "-1";
    },
  ).join("");
  return decoded;
};

export const part2 = async () => {
  const entries = await getEntries();
  let sum = 0;
  for (const entry of entries) {
    const decoded = decodeEntry(entry);
    console.log(decoded);
    sum += Number(decoded);
  }

  return sum;
};
