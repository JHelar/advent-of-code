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
type CaveMap = Record<Tile, Node>;

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
