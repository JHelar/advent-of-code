import {
  Color,
  Image,
  TerminalCanvas,
} from "https://deno.land/x/terminal@0.1.0-dev.3/src/mod.ts";
import {
  createRenderer,
  makeColor,
  PALETTE,
  setBackground,
  sleep,
} from "../renderer/render.ts";
import {
  BIG_CAVE,
  END_CAVE,
  SMALL_CAVE,
  SPACING,
  START_CAVE,
} from "./caves.ts";

// INPUT URL: https://adventofcode.com/2021/day/12/input
const enum GOALS {
  START = "start",
  END = "end",
}
type Tile = string | GOALS;
type TileType = "big" | "small" | GOALS;
type Node = {
  tile: Tile;
  type: TileType;
  connections: Tile[];
};
type Position = [number, number];
type CaveMap = Record<Tile, Node>;
type CavePositions = Record<Tile, Position>;

const isTileGoal = (tile: Tile) => tile === GOALS.START || tile === GOALS.END;
const isTileSmall = (tile: Tile) => !isTileGoal(tile) && !isTileBig(tile);
const isTileBig = (tile: Tile) => {
  if (isTileGoal(tile)) return false;
  const firstCharCode = tile.charCodeAt(0);
  return firstCharCode >= 65 && firstCharCode <= 90;
};
const getTileType = (tile: Tile): TileType => {
  if (isTileGoal(tile)) return tile as GOALS;
  if (isTileSmall(tile)) return "small";
  return "big";
};

const getCaveMap = async () => {
  const inputString = await Deno.readTextFile("./day-12/input.txt");
  const lines = inputString.split("\n");
  const tiles = lines.map((line) => line.trim().split("-")) as Tile[][];
  const caveMap = tiles.reduce((acc, [tile, connectedTo]) => {
    if (!(connectedTo in acc)) {
      acc[connectedTo] = {
        tile: connectedTo,
        type: getTileType(connectedTo),
        connections: [],
      };
    }
    if (!(tile in acc)) {
      acc[tile] = {
        tile,
        type: getTileType(tile),
        connections: [],
      };
    }

    acc[tile].connections.push(connectedTo);
    acc[connectedTo].connections.push(tile);

    return acc;
  }, {} as CaveMap);
  return caveMap;
};

const findPaths = (
  node: Node,
  caveMap: CaveMap,
  visited: Record<Tile, number>,
  maxVisits?: number,
) => {
  const result: Tile[][] = [];

  const newVisited = { ...visited };
  if (node.type === "small") {
    newVisited[node.tile] = ++newVisited[node.tile] || 1;
  }

  if (
    maxVisits !== undefined &&
    Object.values(newVisited).filter((count) => count >= maxVisits).length > 1
  ) {
    return result;
  }

  for (let i = 0; i < node.connections.length; i++) {
    const connection = node.connections[i];
    const connectionNode = caveMap[connection];
    const visited = newVisited[connection] ?? 0;
    if (
      connectionNode.type === "small" &&
        (maxVisits === undefined && visited === 1) ||
      (maxVisits !== undefined && visited >= maxVisits)
    ) {
      continue;
    }
    if (connection === GOALS.START) continue;
    if (connection === GOALS.END) {
      result.push([node.tile, connection]);
      continue;
    }

    const subPaths = findPaths(
      connectionNode,
      caveMap,
      newVisited,
      maxVisits,
    );
    subPaths.forEach((path) => {
      result.push([node.tile, ...path]);
    });
  }

  return result;
};

export const part1 = async () => {
  const caveMap = await getCaveMap();
  const startNode = caveMap[GOALS.START];
  const paths = findPaths(startNode, caveMap, {});
  return paths.length;
};

export const part2 = async () => {
  const caveMap = await getCaveMap();
  const startNode = caveMap[GOALS.START];
  const paths = findPaths(startNode, caveMap, {}, 2);
  return paths.length;
};

const getCaveImage = (type: TileType, color?: Color): Image =>
  type === "small"
    ? SMALL_CAVE(color)
    : type === "big"
    ? BIG_CAVE(color)
    : type === GOALS.START
    ? START_CAVE
    : END_CAVE;

const getTreeSize = (
  caveMap: CaveMap,
) => {
  const startNode = caveMap[GOALS.START];

  let maxWidth = 0;
  let depth = 0;
  const q: Node[] = [startNode];
  const visited: Tile[] = [];

  while (q.length > 0) {
    let count = q.length;
    depth++;
    maxWidth = Math.max(maxWidth, count);
    while (count > 0) {
      const temp = q.pop();
      if (temp) {
        visited.push(temp.tile);
        q.unshift(
          ...temp.connections.filter((tile) =>
            !visited.includes(tile) && !q.includes(caveMap[tile])
          ).map((
            tile,
          ) => caveMap[tile]),
        );
      }
      count--;
    }
  }

  return {
    width: maxWidth,
    depth,
  };
};

const getCavePositions = (caveMap: CaveMap) => {
  const { width } = getTreeSize(caveMap);
  let y = 0;

  const startNode = caveMap[GOALS.START];
  const q: Node[] = [startNode];
  const visited: Tile[] = [];
  const cavePositions: CavePositions = {};
  const caveOrder: Node[] = [];

  while (q.length > 0) {
    let count = q.length;
    const spacingSteps = Math.round((width * SPACING) / count + 1);
    y += SPACING;
    let x = SPACING + spacingSteps;
    while (count > 0) {
      const temp = q.pop();
      if (temp) {
        caveOrder.push(temp);
        const caveImage = getCaveImage(temp.type);
        cavePositions[temp.tile] = [
          x,
          y,
        ];
        visited.push(temp.tile);
        x += caveImage.width + spacingSteps;
        q.unshift(
          ...temp.connections.filter((tile) =>
            !visited.includes(tile) && !q.includes(caveMap[tile])
          ).map((
            tile,
          ) => caveMap[tile]),
        );
      }
      count--;
    }
  }

  return { cavePositions, caveOrder };
};

const drawTunnel = (
  from: Node,
  to: Node,
  cavePositions: CavePositions,
  canvas: TerminalCanvas,
  color = PALETTE.GRAY_DARK,
) => {
  const [x, y] = cavePositions[from.tile];
  const { width: fromWidth, height: fromHeight } = getCaveImage(from.type);
  const startX = x + Math.floor(fromWidth / 2);
  const startY = y + Math.floor(fromHeight / 2);

  const [cx, cy] = cavePositions[to.tile];
  const { width: toWidth, height: toHeight } = getCaveImage(to.type);
  const endX = cx + Math.floor(toWidth / 2);
  const endY = cy + Math.floor(toHeight / 2);

  if (startX - endX < 0) {
    canvas.drawLine(
      endX,
      endY,
      startX,
      startY,
      color,
    );
  } else {
    canvas.drawLine(
      startX,
      startY,
      endX,
      endY,
      color,
    );
  }
};

const drawTunnels = (
  caveMap: CaveMap,
  cavePositions: CavePositions,
  caveOrder: Node[],
  canvas: TerminalCanvas,
) => {
  const rendered: string[] = [];
  for (const node of caveOrder) {
    for (const connectedTile of node.connections) {
      const renderKey = (connectedTile + node.tile).split("").sort().join("");
      if (rendered.includes(renderKey)) continue;
      rendered.push(renderKey);

      drawTunnel(node, caveMap[connectedTile], cavePositions, canvas);
    }
  }
};

const drawCave = (
  tile: Tile,
  position: Position,
  canvas: TerminalCanvas,
  color: Color = PALETTE.WHITE,
) => {
  const [startX, startY] = position;
  const caveImage = getCaveImage(getTileType(tile), color);
  canvas.drawImage(startX, startY, caveImage);
};

const drawCaves = (cavePositions: CavePositions, canvas: TerminalCanvas) => {
  for (const tile of Object.keys(cavePositions)) {
    drawCave(tile, cavePositions[tile], canvas);
  }
};

export const render = async () => {
  const caveMap = await getCaveMap();
  const canvas = await createRenderer();
  setBackground(PALETTE.BLACK);
  canvas.clear();
  const { cavePositions, caveOrder } = getCavePositions(caveMap);
  drawTunnels(caveMap, cavePositions, caveOrder, canvas);
  drawCaves(cavePositions, canvas);

  await canvas.render();
  await sleep(150);

  for (const path of findPaths(caveMap[GOALS.START], caveMap, {})) {
    drawTunnels(caveMap, cavePositions, caveOrder, canvas);
    drawCaves(cavePositions, canvas);
    for (let i = path.length - 1; i > 0; i--) {
      const fromTile = path[i - 1];
      const toTile = path[i];
      drawTunnel(
        caveMap[fromTile],
        caveMap[toTile],
        cavePositions,
        canvas,
        PALETTE.GREEN_LIGHT,
      );
      drawCave(toTile, cavePositions[toTile], canvas, PALETTE.RED);
      drawCave(fromTile, cavePositions[fromTile], canvas, PALETTE.RED);
    }
    await canvas.render();
    await sleep(150);
  }
};
