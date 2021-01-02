import fs from 'fs'
import { toHtmlDocument, toPixel } from './toHtml'


const getMapKey = (x: number, y: number, z: number) => `${x},${y},${z}`
const getMapCoordinate = (key: string) => key.split(',').map(Number)

let map: Map<string, boolean> = fs.readFileSync('day-24/input.txt').toString().split('\n').flatMap((row, rowIndex) => row.split('').map((column, columnIndex) => ({
    x: columnIndex,
    y: rowIndex,
    bug: column === '#'
}))).reduce((acc, tile) => {
    const key = getMapKey(tile.x, tile.y, 0)
    acc.set(key, tile.bug)
    return acc
}, new Map())

const getbBiodiversityRating = (map: Map<string, boolean>) => {
    let tilePoint = 1
    let sum = 0
    for (const [key, bug] of map) {
        if(bug) {
            sum += tilePoint
        }
        tilePoint *= 2
    }
    return sum
}

const getBugCount = (map: Map<string, boolean>) => {
    let sum = 0
    for (const [key, bug] of map) {
        sum += bug ? 1 : 0
    }
    return sum
}

const getAdjacent = ([x, y, z]: number[]) => {
    const isEdge = y === 0 || y === 4 || x === 0 || x === 4
    const isInner = y === 1 && x === 2 || y === 3 && x === 2 || y === 2 && x === 1 || y === 2 && x === 3

    let adjacent = new Set<number[]>()
    if(isEdge) {
        if(y === 0) {
            adjacent.add([x + 0, y + 1, z + 0])
            adjacent.add([2, 1, z - 1])
        } else if(y === 4) {
            adjacent.add([x + 0, y - 1, z + 0])
            adjacent.add([2, 3, z - 1])
        } else {
            adjacent.add([x + 0, y - 1, z + 0])
            adjacent.add([x + 0, y + 1, z + 0])     
        }

        if(x === 0) {
            adjacent.add([x + 1, y + 0, z + 0])
            adjacent.add([1, 2, z - 1])
        } else if(x === 4) {
            adjacent.add([x - 1, y + 0, z + 0])
            adjacent.add([3, 2, z - 1])
        } else {
            adjacent.add([x - 1, y + 0, z + 0])
            adjacent.add([x + 1, y + 0, z + 0])
        }

    } else if(isInner) {
        if(y === 1 && x === 2) {
            adjacent.add([0, 0, z + 1])
            adjacent.add([1, 0, z + 1])
            adjacent.add([2, 0, z + 1])
            adjacent.add([3, 0, z + 1])
            adjacent.add([4, 0, z + 1])

            adjacent.add([x - 1, y + 0, z + 0])
            adjacent.add([x + 1, y + 0, z + 0])
            adjacent.add([x + 0, y - 1, z + 0])
        } else if(y === 2 && x === 1) {
            adjacent.add([0, 0, z + 1])
            adjacent.add([0, 1, z + 1])
            adjacent.add([0, 2, z + 1])
            adjacent.add([0, 3, z + 1])
            adjacent.add([0, 4, z + 1])

            adjacent.add([x - 1, y + 0, z + 0])
            adjacent.add([x + 0, y - 1, z + 0])
            adjacent.add([x + 0, y + 1, z + 0])
        } else if(y === 3 && x === 2) {
            adjacent.add([0, 4, z + 1])
            adjacent.add([1, 4, z + 1])
            adjacent.add([2, 4, z + 1])
            adjacent.add([3, 4, z + 1])
            adjacent.add([4, 4, z + 1])

            adjacent.add([x - 1, y + 0, z + 0])
            adjacent.add([x + 1, y + 0, z + 0])
            adjacent.add([x + 0, y + 1, z + 0])
        } else if(y === 2 && x === 3) {
            adjacent.add([4, 0, z + 1])
            adjacent.add([4, 1, z + 1])
            adjacent.add([4, 2, z + 1])
            adjacent.add([4, 3, z + 1])
            adjacent.add([4, 4, z + 1])

            adjacent.add([x + 1, y + 0, z + 0])
            adjacent.add([x + 0, y - 1, z + 0])
            adjacent.add([x + 0, y + 1, z + 0])
        }
    } else {
        adjacent.add([x + 0, y - 1, z + 0])
        adjacent.add([x + 0, y + 1, z + 0])
        adjacent.add([x - 1, y + 0, z + 0])
        adjacent.add([x + 1, y + 0, z + 0])
    }

    return Array(...adjacent.values())
}

const setMapAtLevel = (map: Map<string, boolean>, refMap: Map<string, boolean>, level: number) => {
    for (let x = 0; x < 5; x++) {
        for (let y = 0; y < 5; y++) {
            const key = getMapKey(x, y, level)
            if(!refMap.has(key)) {
                map.set(key, false)
            } else {
                map.set(key, refMap.get(key)!)
            }
        }
    }
}

const runs = 200

for (let z = 1; z <= runs / 2; z++) {
    setMapAtLevel(map, map, z)
    setMapAtLevel(map, map, z * -1)
}

let run = 0
while(run < runs) {
    let newMap: Map<string, boolean> = new Map()
    for (const [key, bug] of map) {
        const position = getMapCoordinate(key)
        if(position[0] === 2 && position[1] === 2) {
            newMap.set(key, false)
            continue
        }

        const adjacent = getAdjacent(position)
        const neighbours = adjacent.map(([x, y, z]) => {
            const aKey = getMapKey(x, y, z)
            return map.get(aKey)
        }).filter(Boolean)

        if(bug) {
            if(neighbours.length === 1) {
                newMap.set(key, true)
            } else {
                newMap.set(key, false)
            }
        } else {
            if(neighbours.length === 1 || neighbours.length === 2) {
                newMap.set(key, true)
            } else {
                newMap.set(key, false)
            }
        }
    }
    map = newMap
    run++
}

const bugCount = getBugCount(map)
let newMap: Map<string, boolean> = new Map()
const pixels = Object.entries(Array(...map.entries()).reduce((acc, [key, bug]) => {
    const pos = getMapCoordinate(key)
    if(!(pos[2] in acc)){
        acc[pos[2]] = []
    }
    acc[pos[2]].push(toPixel(pos, bug))
    return acc
},{} as Record<number, string[]>)).sort((a, b) => Number(a[0]) - Number([0])).map(([_, v]) => v)

const document = toHtmlDocument(pixels, 600, 800)
fs.writeFileSync('day-24/map.html', document)
console.log({
    bugCount
})
