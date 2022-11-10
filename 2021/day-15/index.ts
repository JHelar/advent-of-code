// INPUT URL: https://adventofcode.com/2021/day/15/input
import {
  Color,
  TerminalCanvas,
} from "https://deno.land/x/terminal@0.1.0-dev.3/src/mod.ts";
import {
  createRenderer,
  drawPixel,
  mult,
  PALETTE,
  renderToScreen,
  setBackground,
  sleep,
} from "../renderer/render.ts";

type Tile = {
  value: number;
  position: Position;
};
interface Node extends Tile {
  parent?: Node;
  g: number;
  h: number;
  f: number;
}
type CaveMap = Tile[][];
type Position = [number, number];

const calcH = (node: Tile, end: Tile) =>
  Math.abs(node.position[0] - end.position[0]) +
  Math.abs(node.position[1] - end.position[1]);
const getTile = ([x, y]: Position, map: CaveMap) => map[y]?.[x];
const getPositionKey = ([x, y]: Position) => `(${x},${y})`;
const getNeighbourTiles = ([x, y]: Position, map: CaveMap): Tile[] =>
  [
    [x + 1, y],
    [x - 1, y],
    [x, y + 1],
    [x, y - 1],
  ]
    .map((pos) => getTile(pos as Position, map))
    .filter(Boolean);
const isEqual = <T extends Tile>({ position: a }: T, { position: b }: T) =>
  a[0] === b[0] && a[1] === b[0];
const parseMap = async () => {
  const inputString = await Deno.readTextFile("./day-15/input.txt");
  const map = inputString.split("\n").map((row: string, y: number) =>
    row
      .trim()
      .split("")
      .map<Tile>((value: string, x: number) => ({
        value: Number(value),
        position: [x, y],
      }))
  ) as CaveMap;

  return map;
};

function* findPath(map: CaveMap, render = false) {
  const start = getTile([0, 0], map)!;
  const startNode: Node = {
    ...start,
    f: 0,
    g: 0,
    h: 0,
  };

  const end = getTile([map[0].length - 1, map.length - 1], map)!;
  const open: Node[] = [startNode];
  const closed: Node[] = [];

  const nodes: Record<string, Node[]> = {
    [getPositionKey(start.position)]: [startNode],
  };
  while (open.length > 0) {
    const q = open.pop();
    if (!q) throw new Error("Shhoooot");
    if (render) yield q;
    const neighbours = getNeighbourTiles(q.position, map);

    for (const neighbour of neighbours) {
      const neighbourNode: Node = {
        ...neighbour,
        h: calcH(neighbour, end),
        g: q.g + neighbour.value,
        f: 0,
        parent: q,
      };
      neighbourNode.f = neighbourNode.h + neighbourNode.g;

      if (isEqual(neighbour, end)) yield neighbourNode;

      const nodesKey = getPositionKey(neighbour.position);
      const allreadySeenNodes = nodes[nodesKey] || [];
      if (allreadySeenNodes.some((n) => n.f <= neighbourNode.f)) continue;
      open.push(neighbourNode);

      nodes[nodesKey] = [...(nodes[nodesKey] || []), neighbourNode];
    }
    open.sort(({ f: a }, { f: b }) => b - a);
    closed.push(q);
  }
}

export const part1 = async () => {
  const map = await parseMap();
  const result = findPath(map).next().value;
  if (result) {
    return result.f;
  }
};

export const part2 = async () => {
  const map = await parseMap();
  const maxHeight = map.length;
  const maxWidth = map[0].length;

  const biggerMap: CaveMap = Array(maxHeight * 5)
    .fill(0)
    .map(() => []);

  for (let y = 0; y < 5; y++) {
    for (let x = 0; x < 5; x++) {
      for (let mapY = 0; mapY < map.length; mapY++) {
        const row: Tile[] = [];
        for (let mapX = 0; mapX < map[mapY].length; mapX++) {
          const { value } = map[mapY][mapX];
          let newValue = value + x + y;
          if (newValue > 9) newValue = newValue % 9;

          row.push({
            position: [mapX + x * maxWidth, mapY + y * maxHeight],
            value: newValue,
          });
        }
        biggerMap[mapY + y * maxHeight].push(...row);
      }
    }
  }

  const result = findPath(biggerMap).next().value;
  if (result) {
    return result.f;
  }
};

const drawPath = (path: Node) => {
  let lookAt = path;

  while (lookAt !== undefined) {
    const [x, y] = lookAt.position;
    if (lookAt.parent === undefined) {
      drawPixel(x, y, PALETTE.YELLOW);
      break;
    } else if (lookAt === path) {
      drawPixel(x, y, PALETTE.RED);
    } else {
      drawPixel(x, y, PALETTE.GREEN_LIGHT);
    }
    lookAt = lookAt.parent;
  }
};

const renderResult = async (path: Node, canvas: TerminalCanvas) => {
  let lookAt = path;

  const coords: [Position, Color][] = [];
  while (lookAt !== undefined) {
    const [x, y] = lookAt.position;
    if (lookAt.parent === undefined) {
      coords.push([[x, y], PALETTE.YELLOW]);
    } else if (lookAt === path) {
      coords.push([[x, y], PALETTE.RED]);
    } else {
      coords.push([[x, y], PALETTE.GREEN_LIGHT]);
    }
    lookAt = lookAt.parent!;
    coords.forEach(([pos, color]) => {
      canvas.drawPixel(pos[0], pos[1], color);
    });
    await canvas.render();
    await sleep(10);
  }
};

export const render = async () => {
  const map = await parseMap();
  const maxHeight = map.length;
  const maxWidth = map[0].length;

  const biggerMap: CaveMap = Array(maxHeight * 5)
    .fill(0)
    .map(() => []);

  for (let y = 0; y < 5; y++) {
    for (let x = 0; x < 5; x++) {
      for (let mapY = 0; mapY < map.length; mapY++) {
        const row: Tile[] = [];
        for (let mapX = 0; mapX < map[mapY].length; mapX++) {
          const { value } = map[mapY][mapX];
          let newValue = value + x + y;
          if (newValue > 9) newValue = newValue % 9;

          row.push({
            position: [mapX + x * maxWidth, mapY + y * maxHeight],
            value: newValue,
          });
        }
        biggerMap[mapY + y * maxHeight].push(...row);
      }
    }
  }

  const canvas = await createRenderer();
  setBackground(PALETTE.BLUE_DARK);
  await canvas.render();

  let result: Node;
  for (const node of findPath(map, true)) {
    drawPath(node);
    await renderToScreen();
    await sleep(10);
    result = node;
  }
  await renderResult(result!, canvas);
};
