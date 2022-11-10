// INPUT URL: https://adventofcode.com/2021/day/19/input
import {
  intersect,
  permutations,
  sumOf,
  zip,
} from "https://deno.land/std@0.117.0/collections/mod.ts";

type Point3D = [x: number, y: number, z: number];
type Scanner = {
  beacons: Point3D[];
  distances: number[][];
};
type Axis = 0 | 1 | 2;
type Transfrom = {
  axis: Axis;
  sign: -1 | 1;
};
type Transform3D = [Transfrom, Transfrom, Transfrom];
type Offset = {
  axes: Transform3D;
  offset: Point3D;
};
type VisitEntry = [from: number, to: number];
type Overlap = [number, number];

const distance = (p1: Point3D, p2: Point3D): number =>
  Math.sqrt(sumOf(zip(p1, p2), ([a, b]) => (a - b) * (a - b)));

const transformMatrixes = permutations<Axis>([0, 1, 2]).flatMap<
  Transform3D
>((
  [x, y, z],
) => [
  [
    { axis: x, sign: 1 },
    { axis: y, sign: 1 },
    { axis: z, sign: 1 },
  ],
  [
    { axis: x, sign: 1 },
    { axis: y, sign: 1 },
    { axis: z, sign: -1 },
  ],
  [
    { axis: x, sign: 1 },
    { axis: y, sign: -1 },
    { axis: z, sign: 1 },
  ],
  [
    { axis: x, sign: 1 },
    { axis: y, sign: -1 },
    { axis: z, sign: -1 },
  ],
  [
    { axis: x, sign: -1 },
    { axis: y, sign: 1 },
    { axis: z, sign: 1 },
  ],
  [
    { axis: x, sign: -1 },
    { axis: y, sign: 1 },
    { axis: z, sign: -1 },
  ],
  [
    { axis: x, sign: -1 },
    { axis: y, sign: -1 },
    { axis: z, sign: 1 },
  ],
  [
    { axis: x, sign: -1 },
    { axis: y, sign: -1 },
    { axis: z, sign: -1 },
  ],
]);

const getOffsets = (scannerCount: number) => {
  const offsets: (Offset | undefined)[] = new Array(scannerCount).fill(
    undefined,
  );
  offsets[0] = {
    axes: transformMatrixes[0],
    offset: [0, 0, 0],
  };

  return offsets;
};

const getOverlaps = (
  { distances: d1 }: Scanner,
  { distances: d2 }: Scanner,
): Overlap[] => {
  const found: Overlap[] = [];
  for (let i1 = 0; i1 < d1.length; i1++) {
    const a1 = d1[i1];
    for (let i2 = 0; i2 < d2.length; i2++) {
      const a2 = d2[i2];
      if (intersect(a1, a2).length >= 12) {
        found.push([i1, i2]);
      }
    }
  }
  return found;
};

const getAllOverlaps = (scanners: Scanner[]) =>
  scanners.map((s1) =>
    scanners.map((s2) => s1 === s2 ? [] : getOverlaps(s1, s2))
  );

const getNeighbours = (overlaps: Overlap[][][]) =>
  overlaps.map((o) =>
    o.map((os, indx) => os.length >= 12 ? indx : undefined).filter<number>((
      i,
    ): i is number => i !== undefined)
  );

const transformPoint = (transform: Transform3D, point: Point3D): Point3D =>
  transform.map(({ sign, axis }) => point[axis] * sign) as Point3D;

const visitScanners = (scanners: Scanner[]) => {
  const allOverlaps = getAllOverlaps(scanners);
  const neighbours = getNeighbours(allOverlaps);
  const offsets = getOffsets(scanners.length);
  const toVisit: VisitEntry[] = neighbours[0].map((
    neighbour,
  ) => [0, neighbour]);
  while (toVisit.length > 0) {
    const [from, to] = toVisit.shift()!;
    if (offsets[to] !== undefined) continue;

    const fromTransform = offsets[from]!.axes;
    const overlaps = allOverlaps[from][to];
    const candidates: [number, Point3D][] = transformMatrixes.map((tm) => {
      const ds = overlaps.map(([fromL, toL]) => {
        const b1 = transformPoint(fromTransform, scanners[from].beacons[fromL]);
        const b2 = transformPoint(tm, scanners[to].beacons[toL]);
        return b1.map((n, i) => n - b2[i]) as Point3D;
      });
      return [
        ds.filter(
          (elt) =>
            elt[0] === ds[0][0] && elt[1] === ds[0][1] && elt[2] === ds[0][2],
        ).length,
        ds[0],
      ];
    });
    const candidate = candidates.findIndex(([b]) => b === overlaps.length);
    if (candidate === -1) {
      throw new Error("Should not happen");
    }
    offsets[to] = {
      axes: transformMatrixes[candidate],
      offset: candidates[candidate][1].map(
        (n, idx) => n + offsets[from]!.offset[idx],
      ) as Point3D,
    };
    toVisit.push(
      ...neighbours[to].map<VisitEntry>((n) => [to, n]),
    );
  }
  return offsets;
};

const getManhatanDistance = ([x1, y1, z1]: Point3D, [x2, y2, z2]: Point3D) =>
  Math.abs(x1 - x2) + Math.abs(y1 - y2) + Math.abs(z1 - z2);

const getScanners = async () => {
  const inputString = await Deno.readTextFile("./day-19/input.txt");
  const scannerLines = inputString.trim().split("\n\r\n");
  const scanners = scannerLines.map<Scanner>((scannerLine) => {
    const [, ...beaconLines] = scannerLine.trim().split("\n");
    const beacons = beaconLines.map((beaconLine) =>
      beaconLine.trim().split(",").map(Number) as Point3D
    );
    return {
      beacons,
      distances: beacons.map((b1) => beacons.map((b2) => distance(b1, b2))),
    };
  });
  return scanners;
};

export const part1 = async () => {
  const scanners = await getScanners();
  const offsets = visitScanners(scanners);
  const allBeacons: Set<string> = new Set();
  scanners.forEach(({ beacons }, i) => {
    const { offset, axes } = offsets[i]!;
    beacons.forEach((b) => {
      allBeacons.add(
        zip(transformPoint(axes, b), offset)
          .map(([i, j]) => i + j)
          .join(","),
      );
    });
  });

  return allBeacons.size;
};

export const part2 = async () => {
  const scanners = await getScanners();
  const offsets = visitScanners(scanners);

  const allDistances = offsets.flatMap((b1) =>
    offsets.map((b2) => getManhatanDistance(b1!.offset, b2!.offset))
  );

  return Math.max(...allDistances);
};
