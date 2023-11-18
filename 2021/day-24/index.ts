// INPUT URL: https://adventofcode.com/2021/day/24/input
type Operator = "inp" | "mul" | "add" | "div" | "mod" | "eql";
type Variable = "x" | "y" | "z" | "w";
type Operation = [
  op: Operator,
  arg1: Variable,
  arg2: Variable | number | undefined
];
type Program = { left: number; right: number; value: number }[];
type StackLine = { stack: { value: number; i: number }[]; lines: Program };

const toOperation = (line: string): Operation => {
  const [op, arg1, arg2] = line.trim().split(" ");
  const arg2Sanitized = isNaN(parseInt(arg2)) ? arg2 : parseInt(arg2);
  return [op, arg1, arg2Sanitized] as Operation;
};

const getProgram = async () => {
  const operations = await (
    await Deno.readTextFile("./day-24/input.txt")
  )
    .split("inp w")
    .filter(Boolean)
    .map((line) => [
      ["inp", "w", undefined],
      ...line.trim().split("\n").map(toOperation),
    ])
    .map((step) => {
      const zTruncate = step[4][2] === 26;
      const xIncrement = step[5][2];
      const yIncrement = step[15][2];

      return {
        stackOp: zTruncate ? "pop" : "push",
        value: zTruncate ? xIncrement : yIncrement,
      };
    })
    .reduce(
      (vars, { stackOp, value }, i) => {
        const { stack, lines } = vars;
        if (typeof value !== "number")
          throw new Error("Panic value is not a number");

        if (stackOp === "push") {
          stack.push({ value: value as number, i });
        } else {
          const head = stack.pop();
          lines.push({
            left: i,
            right: head!.i,
            value: head!.value + value,
          });
        }

        return vars;
      },
      { stack: [], lines: [] } as StackLine
    ).lines;

  return operations;
};

const isValidDigit = (digit: number) => digit >= 1 && digit <= 9;

const runProgram = (
  program: Program,
  op: Extract<keyof Math, "max" | "min">
) => {
  const result = Array(14).fill(0);
  const digits = Array(9)
    .fill(0)
    .map((_, i) => i + 1);

  for (const restraint of program) {
    const right_input = Math[op](
      ...digits.filter((d) => isValidDigit(restraint.value + d))
    );
    const left_input = right_input + restraint.value;
    result[restraint.left] = left_input;
    result[restraint.right] = right_input;
  }

  return result.join("");
};

export const part1 = async () => {
  const program = await getProgram();
  return runProgram(program, "max");
};

export const part2 = async () => {
  const program = await getProgram();
  return runProgram(program, "min");
};
