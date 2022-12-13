import fs from "fs/promises";
import path from "path";

const readInput = async () => {
  const content = await fs.readFile(path.resolve(__dirname, "input.txt"));
  return content.toString("utf8");
};

type Node = {
  x: number;
  y: number;
  visited: boolean;
  value: "S" | "E" | number;
  distance: number;
  parent?: Node;
  f: number;
  g: number;
  h: number;
};
type Map = Record<string, Node>;

const getKey = ({ x, y }: { x: number; y: number }) => `(${x},${y})`;

const parseMap = async () => {
  const content = await readInput();
  return content
    .split("\n")
    .flatMap((line) => line.trim())
    .filter(Boolean)
    .flatMap((row, y) =>
      row.split("").map((tile, x) => ({
        x,
        y,
        value: tile === "S" || tile === "E" ? tile : tile.charCodeAt(0),
        distance: tile === "S" ? 0 : Infinity,
        visited: false,
        f: 0,
        g: 0,
        h: 0,
      }))
    )
    .reduce((map, node) => {
      map[getKey(node)] = node;
      return map;
    }, {}) as Map;
};

const getNeighbours = (node: Node, map: Map) =>
  [
    [0, -1],
    [0, 1],
    [-1, 0],
    [1, 0],
  ]
    .map(([x, y]) => map[getKey({ x: node.x + x, y: node.y + y })])
    .filter(Boolean)
    .filter(({ visited }) => !visited)
    .filter((neighbour) => {
      if (neighbour.value === "E") {
        return node.value === "z".charCodeAt(0);
      }
      if (neighbour.value === "S") return false;
      if (typeof node.value === "string") {
        return true
      }

      if (neighbour.value <= node.value) return true;
      if (neighbour.value === (node.value + 1)) return true;

      return false;
    });

const getDistance = (node: Node, end: Node) =>
  Math.abs(node.x - end.x) + Math.abs(node.y - end.y);

const findPath = (map: Map) => {
  const startNode = Object.values(map).find(({ value }) => value === "S")!;
  const endNode = Object.values(map).find(({ value }) => value === "E")!;

  const queue = [startNode];
  const visited: Record<string, boolean> = {}

  while (queue.length > 0) {
    const currentNode = queue.pop();
    if (!currentNode) return;

    const neighbours = getNeighbours(currentNode, map);

    for (const neighbour of neighbours) {
        if(visited[getKey(neighbour)]) {
            throw new Error('Oh dear')
        }
      neighbour.parent = currentNode;
      if (neighbour.value === "E") {
        return neighbour;
      }

      neighbour.h = getDistance(neighbour, endNode);
      neighbour.g =
        currentNode.g +
        (typeof neighbour.value === "string" ? 0 : neighbour.value);
      neighbour.f = neighbour.h + neighbour.g;
      queue.push(neighbour);
    }
    queue.sort((a, b) => b.f - a.f);
    currentNode.visited = true;
    visited[getKey(currentNode)] = true
  }
};

const part1 = async () => {
  const map = await parseMap();
  let currentNode = findPath(map);
  let steps = -1;
  while (currentNode) {
    currentNode = currentNode.parent;
    steps++;
  }
  return `Result: ${steps}`;
};

const part2 = async () => {
  return null;
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
