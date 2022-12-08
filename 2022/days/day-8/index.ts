import fs from "fs/promises";
import path from "path";

type Tree = number;
type TreeRow = Tree[];
type Trees = TreeRow[];

const readInput = async () => {
  const content = await fs.readFile(path.resolve(__dirname, "input.txt"));
  return content.toString("utf8");
};

const getTrees = async (): Promise<Trees> => {
  const content = await readInput();
  return content
    .split("\n")
    .filter(Boolean)
    .map((line) => line.trim().split("").map(Number));
};

const getColumn = (trees: Trees, columnPos: number): TreeRow => {
  const column: TreeRow = [];
  trees.forEach((row) => {
    column.push(row[columnPos]);
  });
  return column;
};

const getScenicScore = ({
  row,
  tree,
}: {
  row: TreeRow;
  tree: Tree;
}) => {
  let score = 0;

  for (const otherTree of row) {
    if (otherTree >= tree) {
      return score + 1;
    }
    score++;
  }

  return score;
};

const part2 = async () => {
  const trees = await getTrees();
  const scores: number[] = [];
  for (let y = 0; y < trees.length; y++) {
    const row = trees[y];
    for (let x = 0; x < row.length; x++) {
      const tree = row[x];
      const column = getColumn(trees, x);

      const scenicScores = [
        column.slice(0, y).reverse(),
        column.slice(y + 1),
        row.slice(0, x).reverse(),
        row.slice(x + 1),
      ].map((r) => getScenicScore({ row: r, tree }));

      const score = scenicScores
        .reduce((sum, score) => sum * score, 1);

      scores.push(score);
    }
  }

  return `Best score: ${Math.max(...scores)}`;
};

const part1 = async () => {
  const trees = await getTrees();
  let visibles = 0;
  for (let y = 0; y < trees.length; y++) {
    const row = trees[y];
    for (let x = 0; x < row.length; x++) {
      const tree = row[x];
      const column = getColumn(trees, x);

      if (y === 0 || y === trees.length - 1) {
        visibles++;
        continue;
      }

      if (x === 0 || x === trees.length - 1) {
        visibles++;
        continue;
      }

      if (Math.max(...column.slice(0, y)) < tree) {
        visibles++;
        continue;
      }

      if (Math.max(...column.slice(y + 1)) < tree) {
        visibles++;
        continue;
      }

      if (Math.max(...row.slice(0, x)) < tree) {
        visibles++;
        continue;
      }

      if (Math.max(...row.slice(x + 1)) < tree) {
        visibles++;
        continue;
      }
    }
  }
  return `Visible trees: ${visibles}`;
};

// Generated code to run on cli
(async () => {
  const [, , part] = process.argv;
  if (part === "1") {
    const result = await part1();
    console.log(result);
  }
  if (part === "2") {
    const result = await part2();
    console.log(result);
  }
})();
