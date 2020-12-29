const isString = (n) => typeof(n) === 'string'

export const makeArray = (ySize, xSize, fill) =>
  Array(ySize)
    .fill()
    .map(() => (xSize ? Array(xSize).fill(fill) : fill));

export const output2dArray = arr => arr.map(line => line.join('')).join('\n');

export const fastMax = arr => arr.reduce((max, v) => (max >= v ? max : v), -Infinity);

export const fastMin = arr => arr.reduce((min, v) => (min <= v ? min : v), Infinity);

export class InfiniteGrid {
  constructor(fill) {
    this.fill = fill;
    this.grid = new Map();
  }

  get cells() {
    return [...this.grid.entries()]
      .map(([pos, value]) => [...pos.split(',').map(Number), value])
      .map(([x, y, value]) => ({ x, y, value }));
  }

  get bounds() {
    const cells = this.cells;
    return {
      min: {
        x: fastMin(cells.map(({ x }) => x)),
        y: fastMin(cells.map(({ y }) => y))
      },
      max: {
        x: fastMax(cells.map(({ x }) => x)),
        y: fastMax(cells.map(({ y }) => y))
      }
    };
  }

  toArray(min, max) {
    const bounds = this.bounds;
    if (min == null) min = bounds.min;
    if (max == null) max = bounds.max;
    const array = makeArray(max.y - min.y + 1, max.x - min.x + 1, this.fill);
    for (let y = min.y; y <= max.y; y++) {
      for (let x = min.x; x <= max.x; x++) {
        array[y - min.y][x - min.x] = this.get(x, y);
      }
    }
    return array;
  }

  key(x, y) {
    return `${x},${y}`;
  }

  set(x, y, value) {
    this.grid.set(this.key(x, y), value);
  }

  get(x, y) {
    return this.grid.has(this.key(x, y)) ? this.grid.get(this.key(x, y)) : this.fill;
  }

  clone() {
    const newGrid = new InfiniteGrid(this.fill);
    newGrid.grid = new Map(this.grid);
    return newGrid;
  }
}

export const sum = (a, b) => a + b;

export const sortNum = (a, b) => a - b;

export const nTimes = (n, cb) => {
  for (let i = 0; i < n; i++) cb();
};

export const range = (start, stop) => {
  const result = [];
  const numOrCharCode = n => (isString(n) ? n.charCodeAt(0) : n);
  for (let i = numOrCharCode(start); i <= numOrCharCode(stop); i++) {
    result.push(isString(start) ? String.fromCharCode(i) : i);
  }
  return result;
};

export const maxBy = cb => (a, b) => (cb(b) > cb(a) ? b : a);

export const minBy = cb => (a, b) => (cb(b) < cb(a) ? b : a);

export const dijkstra = (graph, source, dest, cbNeighbors) => {
  const allKeys = new Set([source]);
  const nodes = new Set([source]);
  const dist = new Map();
  const prev = new Map();

  const getDist = key => (dist.has(key) ? dist.get(key) : Infinity);
  dist.set(source, 0);

  while (nodes.size) {
    const closest = [...nodes].reduce(minBy(n => getDist(n)));
    if (dest && closest === dest) {
      return [dist.get(dest), toPath(prev, source, dest)];
    }
    nodes.delete(closest);
    const neighbors = cbNeighbors ? cbNeighbors(graph, closest) : graph[closest];
    neighbors.forEach(neighbor => {
      if (!allKeys.has(neighbor)) {
        allKeys.add(neighbor);
        nodes.add(neighbor);
      }
      const alt = getDist(closest) + 1;
      if (alt < getDist(neighbor)) {
        dist.set(neighbor, alt);
        prev.set(neighbor, closest);
      }
    });
  }

  return dest ? [] : [dist, prev];
};

export const toPath = (prev, source, dest) => {
  const path = [];
  let current;
  do {
    current = current ? prev.get(current) : dest;
    path.unshift(current);
  } while (current !== source);
  return path;
};

export const chunk = (arr, size = 1) => {
  const chunks = [];
  for (let i = 0; i < arr.length; i += size) {
    chunks.push(arr.slice(i, i + size));
  }
  return chunks;
};

export const gcd = (a, b) => (a ? gcd(b % a, a) : b);

export const lcm = (a, b) => (a * b) / gcd(a, b);

export const sortBy = (...cbs) => (a, b) => {
  for (const cb of cbs) {
    const aa = cb(a);
    const bb = cb(b);
    const diff = cb.desc
      ? isString(aa)
        ? bb.localeCompare(aa)
        : bb - aa
      : isString(aa)
      ? aa.localeCompare(bb)
      : aa - bb;
    if (diff !== 0) return diff;
  }
  return 0;
};
export const desc = cb => ((cb.desc = true), cb);

export const getPermutations = arr => {
  const result = [];

  const p = (arr, temp = []) => {
    let i, x;
    if (!arr.length) result.push(temp);

    for (i = 0; i < arr.length; i++) {
      x = arr.splice(i, 1)[0];
      p(arr, temp.concat(x));
      arr.splice(i, 0, x);
    }
  };

  p(arr);
  return result;
};

export const getAllSubsets = arr =>
  arr.reduce((subsets, value) => subsets.concat(subsets.map(set => [value, ...set])), [[]]);

export const nestedLoop = function*(n, min, max, filter) {
  const getMin = i => (Array.isArray(min) ? min[i] : min);
  const getMax = i => (Array.isArray(max) ? max[i] : max);

  const arr = [...Array(n)].map((_, i) => getMin(i));
  let i = 0;
  while (true) {
    if (!filter || filter(arr)) yield arr.slice();

    arr[0]++;
    while (arr[i] === getMax(i) + 1) {
      arr[i] = getMin(i);
      i++;
      if (i === n) return;
      arr[i]++;
      if (arr[i] !== getMax(i) + 1) i = 0;
    }
  }
};
