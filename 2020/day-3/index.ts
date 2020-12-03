import { readFileWithSeparator } from "../utils"

const TREE_TILE = '#'
const SQUARE_TILE = '.'

const createMapTraverser = (map: string[][]) => {
    const mapWidth = map[0].length
    const mapHeight = map.length

    const convertToMapCoordinate = (x, y) => {
        const mapX = x % mapWidth
        const mapY = y % mapHeight

        return [mapX, mapY]
    }

    const getMapTileAt = (x, y) => {
        const [mapX, mapY] = convertToMapCoordinate(x, y)
        return map[mapY][mapX]
    }

    return ([stepX, stepY ]: number[]) => {
        let trees = 0
        let x = 0
        let y = 0
        while(true) {
            x += stepX
            y += stepY

            if(y >= mapHeight) {
                break
            }

            const tile = getMapTileAt(x, y)
            if(tile === TREE_TILE) {
                trees++
            }
        }
        return trees
    }
}

export default () => {
    const map = readFileWithSeparator('day-3/input.txt', '\n').map(row => row.split(''))
    const traverseMap = createMapTraverser(map)

    return [[1, 1],[3, 1],[5, 1],[7, 1],[1, 2]].map(traverseMap).reduce((sum, result) => sum * result, 1)
}