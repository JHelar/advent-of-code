// INPUT URL: https://adventofcode.com/2021/day/10/input
const OPEN_TOKENS = ["(", "[", "{", "<"] as const;
const CLOSE_TOKENS = [")", "]", "}", ">"] as const;
type Token = typeof OPEN_TOKENS[number] | typeof CLOSE_TOKENS[number] | "ROOT";
type Node = {
  token: Token;
  children: Node[];
  parent?: Node;
};

const SYNTAX_CHECK_POINTS: Record<typeof CLOSE_TOKENS[number], number> = {
  ")": 3,
  "]": 57,
  "}": 1197,
  ">": 25137,
};

const AUTO_COMPLETE_POINTS: Record<typeof CLOSE_TOKENS[number], number> = {
  ")": 1,
  "]": 2,
  "}": 3,
  ">": 4,
};

class SyntaxError extends Error {
  public line: number;
  public column: number;
  public token: string;

  constructor(line: number, column: number, token: string) {
    super(
      `Syntax error: invalid syntax "${token}" at line: ${line}, col: ${column}`,
    );
    this.line = line;
    this.column = column;
    this.token = token;
  }
}

const autoComplete = (node: Node) => {
  let lookAt = node;
  const tokensAdded: Token[] = [];
  while (lookAt.parent !== undefined) {
    const closeTokenIndex = OPEN_TOKENS.findIndex((ot) => ot === lookAt.token);
    const closeToken = CLOSE_TOKENS[closeTokenIndex];
    tokensAdded.push(closeToken);

    lookAt = lookAt.parent;
  }

  return tokensAdded;
};

const parseLine = (line: string, lineNo: number) => {
  const root: Node = {
    token: "ROOT",
    children: [],
  };
  let lookAt = root;
  for (let i = 0; i < line.length; i++) {
    const token = line[i] as Token;
    if (OPEN_TOKENS.some((ot) => ot === token)) {
      const newNode: Node = {
        token,
        children: [],
        parent: lookAt,
      };
      lookAt.children.push(newNode);
      lookAt = newNode;
      continue;
    }
    const closeTokenIndex = CLOSE_TOKENS.findIndex((ct) => ct === token);
    if (closeTokenIndex === -1) {
      throw new SyntaxError(lineNo, i + 1, token);
    }

    if (
      closeTokenIndex ===
        OPEN_TOKENS.findIndex((ot) => ot === lookAt.token as any)
    ) {
      if (lookAt.parent === root) {
        return {
          root,
          tokensAdded: [],
        };
      }
      lookAt = lookAt.parent!;
    } else {
      throw new SyntaxError(lineNo, i + 1, token);
    }
  }

  // Uncompleted line, auto complete
  const tokensAdded = autoComplete(lookAt);
  return {
    root,
    tokensAdded,
  };
};

export const part1 = async () => {
  const inputString = await Deno.readTextFile("./day-10/input.txt");
  const lines = inputString.split("\n").map((line) => line.trim());

  let sum = 0;
  for (let i = 0; i < lines.length; i++) {
    try {
      parseLine(lines[i], i + 1);
    } catch (error) {
      if (error instanceof SyntaxError) {
        if (error.token in SYNTAX_CHECK_POINTS) {
          sum += (SYNTAX_CHECK_POINTS as any)[error.token];
        } else {
          throw error;
        }
      } else {
        throw error;
      }
    }
  }

  return sum;
};

export const part2 = async () => {
  const inputString = await Deno.readTextFile("./day-10/input.txt");
  const lines = inputString.split("\n").map((line) => line.trim());

  const scores: number[] = [];
  for (let i = 0; i < lines.length; i++) {
    try {
      const { tokensAdded } = parseLine(lines[i], i + 1);
      if (tokensAdded.length) {
        const sum = tokensAdded.reduce(
          (sum, token) => sum * 5 + (AUTO_COMPLETE_POINTS as any)[token],
          0,
        );
        scores.push(sum);
      }
    } catch (error) {
      if (!(error instanceof SyntaxError)) {
        throw error;
      }
    }
  }

  return scores.sort((a, b) => a - b)[Math.floor(scores.length / 2)];
};
