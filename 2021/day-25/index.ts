// INPUT URL: https://adventofcode.com/2021/day/25/input
type CucumberType = ">" | "v";
type SeaNode = CucumberType | ".";
type Cucumber = {
  type: CucumberType;
  posX: number;
  posY: number;
};

type SeaMap = SeaNode[][];
type Cucumbers = Record<CucumberType, Cucumber[]>;

const parseMap = async (): Promise<[map: SeaMap, cucumbers: Cucumbers]> => {
  const inputString = await Deno.readTextFile("./day-25/input.txt");
  const cucumbers: Cucumbers = {
    ">": [],
    v: [],
  };
  const map: SeaMap = inputString.split("\n").map((line, posY) =>
    line
      .trim()
      .split("")
      .map((node, posX) => {
        if (node === ".") return ".";
        const cucumber: Cucumber = {
          type: node as CucumberType,
          posY,
          posX,
        };
        cucumbers[cucumber.type].push(cucumber);
        return node as SeaNode;
      })
  );

  return [map, cucumbers];
};

const getNeighbour = (cucumber: Cucumber, map: SeaMap) => {
  if (cucumber.type === ">") {
    let nextTile = map[cucumber.posY][cucumber.posX + 1];
    if (nextTile !== undefined)
      return {
        node: nextTile,
        posY: cucumber.posY,
        posX: cucumber.posX + 1,
      };

    nextTile = map[cucumber.posY][0];
    return {
      node: nextTile,
      posY: cucumber.posY,
      posX: 0,
    };
  }

  let nextTile = map[cucumber.posY + 1]?.[cucumber.posX];
  if (nextTile !== undefined)
    return {
      node: nextTile,
      posY: cucumber.posY + 1,
      posX: cucumber.posX,
    };

  nextTile = map[0][cucumber.posX];
  return {
    node: nextTile,
    posY: 0,
    posX: cucumber.posX,
  };
};

const step = (
  cucumbers: Cucumber[],
  map: SeaMap
): [moved: boolean, map: SeaMap] => {
  let moved = false;
  const newMap = [...map.map(row => [...row])]

  for (const cucumber of cucumbers) {
    const neighbour = getNeighbour(cucumber, map);
    if (neighbour.node === ".") {
      newMap[neighbour.posY][neighbour.posX] = cucumber.type;
      newMap[cucumber.posY][cucumber.posX] = ".";

      cucumber.posX = neighbour.posX;
      cucumber.posY = neighbour.posY;

      moved = true;
    }
  }

  return [moved, newMap];
};

const printMap = (map: SeaMap) => {
  console.log(map.map((row) => row.join("")).join("\n"));
};

export const part1 = async () => {
  let [map, cucumbers] = await parseMap();
  let movedEast = true
  let movedSouth = true
  let moved = true
  let stepCount = 0
  for (stepCount = 0; moved; stepCount++) {
    [movedEast, map] = step(cucumbers[">"], map);
    [movedSouth, map] = step(cucumbers["v"], map);
    moved = movedEast || movedSouth
  }

  printMap(map)
  return stepCount  
};

export const part2 = async () => {};
