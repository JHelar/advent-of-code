// INPUT URL: https://adventofcode.com/2021/day/23/input
const enum Amphipod {
  A = 1,
  B = 10,
  C = 100,
  D = 1000,
}

type NOP = null;
type Maybe<T> = null | T;
type Vacant<T> = undefined | T;
type VacantAmphipod = Vacant<Amphipod>;
type Room = {
  spots: [VacantAmphipod, VacantAmphipod];
  roomIndex: number;
};
type HallwaySpot = VacantAmphipod;
type Hallway = [
  HallwaySpot,
  HallwaySpot,
  NOP,
  HallwaySpot,
  NOP,
  HallwaySpot,
  NOP,
  HallwaySpot,
  NOP,
  HallwaySpot,
  HallwaySpot,
];
type Rooms = [NOP, NOP, Room, NOP, Room, NOP, Room, NOP, Room, NOP, NOP];
type VisitNode = {
  amphipod: Amphipod;
  isCorrect: boolean;
  spotNumber: Maybe<number>;
  roomIndex: Maybe<number>;
  hallwayIndex: Maybe<number>;
};

type State = {
  hallway: Hallway;
  rooms: Rooms;
  energy: number;
};

const charToAmphipod = (char: string): Amphipod => {
  if (char === "A") return Amphipod.A;
  if (char === "B") return Amphipod.B;
  if (char === "C") return Amphipod.C;
  if (char === "D") return Amphipod.D;
  throw Error(`Unknown amphipod: "${char}"`);
};

const parseInput = async (): Promise<State> => {
  const inputString = await Deno.readTextFile("./day-23/input.txt");
  const [, , rooms1String, rooms2String] = inputString.split("\n");
  const hallway: Hallway = [
    undefined,
    undefined,
    null,
    undefined,
    null,
    undefined,
    null,
    undefined,
    null,
    undefined,
    undefined,
  ];
  const [, , , a1, , b1, , c1, , d1] = rooms1String.trim();
  const [, a2, , b2, , c2, , d2] = rooms2String.trim();

  const roomA: Room = {
    spots: [charToAmphipod(a1), charToAmphipod(a2)],
    roomIndex: 2,
  };
  const roomB: Room = {
    spots: [charToAmphipod(b1), charToAmphipod(b2)],
    roomIndex: 4,
  };
  const roomC: Room = {
    spots: [charToAmphipod(c1), charToAmphipod(c2)],
    roomIndex: 6,
  };
  const roomD: Room = {
    spots: [charToAmphipod(d1), charToAmphipod(d2)],
    roomIndex: 8,
  };
  const rooms: Rooms = [
    null,
    null,
    roomA,
    null,
    roomB,
    null,
    roomC,
    null,
    roomD,
    null,
    null,
  ];

  return {
    hallway,
    rooms,
    energy: 0,
  };
};

const getAmphipodRoomIndex = (amphipod: Amphipod) =>
  2 + ((amphipod.toString().length - 1) * 2);
const getAmphipodRoom = (amphipod: Amphipod, rooms: Rooms) =>
  rooms[getAmphipodRoomIndex(amphipod)];

export const part1 = async () => {
};

export const part2 = async () => {
};
