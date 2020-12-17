import { readFileWithSeparator } from '../utils'

const Vec4 = (x: number, y: number, z: number, w: number) => {
  const add = ([nx, ny, nz, nw]: number[]) => {
    return Vec4(x + nx, y + ny, z + nz, w + nw)
  }

  const toArray = () => [x, y, z, w]

  const toString = () => `(${x},${y},${z},${w})`

  return {
    add,
    toArray,
    toString,
    x,
    y,
    z,
    w
  }
}

type Vec4 = ReturnType<typeof Vec4>

const offsets: Array<Vec4> = []

const ACTIVE = '#'
const INACTIVE = '.'

for(let x = -1; x <= 1; x++) {
  for(let y = -1; y <= 1; y++) {
    for(let z = -1; z <= 1; z++) {
      for(let w = -1; w <= 1; w++) {
        if(x !== 0 || y !== 0 || z !== 0 || w !== 0) {
          offsets.push(Vec4(x, y, z, w))
        }
      }
    }
  }
}

const getNeighbours = (point: Vec4) => offsets.map(offset => Vec4(point.x, point.y, point.z, point.w).add(offset.toArray()))

const mutateMap = (point: Point, map: Map<string, Point>) => {
  point.neighbours.forEach(p => {
    if(!map.has(p.toString())) {
      map.set(p.toString(), {
        neighbours: getNeighbours(p),
        pos: p,
        state: INACTIVE
      })
    }
  })
}

const checkState = (point: Point, map: Map<string, Point>): {point: Point, didChange: boolean} => {
  let activeNeighbours = 0
  point.neighbours.forEach(p => {
    const neighbour = map.get(p.toString())
    if(neighbour && neighbour.state === ACTIVE) activeNeighbours++
  })

  if(point.state === ACTIVE) {
    if(activeNeighbours === 2 || activeNeighbours === 3) {
      return {
        didChange: false,
        point
      }
    }
    else {
      return {
        point: {
          pos: point.pos,
          neighbours: point.neighbours,
          state: INACTIVE
        },
        didChange: true
      }
    }
  } else {
    if(activeNeighbours === 3) {
      return {
        point: {
          neighbours: point.neighbours,
          pos: point.pos,
          state: ACTIVE
        },
        didChange: true
      }
    }
  }
  return {
    point,
    didChange: false
  }
}

interface Point {
  state: string
  pos: Vec4
  neighbours: Vec4[]
}

export default async () => {
  const map = readFileWithSeparator('day-17/input.txt', '\n').reduce((acc, row, rowIndex) => {
    return [...acc, ...row.split('').map((point, colIndex) => ({ pos: Vec4(colIndex, rowIndex, 0, 0), state: point, neighbours: [] }) as Point)]
  }, [] as Point[]).reduce((map, p) => {
    p.neighbours = getNeighbours(p.pos)
    map.set(p.pos.toString(), p)
    return map
  }, new Map<string, Point>())

  const stopAt = 6
  let mutations: Array<Point> = []
  for (let run = 0; run < stopAt; run++) {
      let lookupPoints = [...map.values()]
      for (const point of lookupPoints) {
        mutateMap(point, map)
      }
      lookupPoints = [...map.values()]
      for (const point of lookupPoints) {
        const { didChange, point: newPoint } = checkState(point, map)
        if(didChange) {
          mutations.push(newPoint)
        }
      }
      mutations.forEach(point => map.set(point.pos.toString(), point))
      mutations = []
  }
  return [...map.values()].reduce((sum, { state }) => sum + (state === ACTIVE ? 1: 0), 0) 
}