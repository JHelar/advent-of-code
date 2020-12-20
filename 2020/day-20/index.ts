import Tile from './tile'
import { readFileByLine, readFileWithSeparator } from '../utils'
import { basename } from 'path'


const reverseString = (str: string) => str.split('').reverse().join('')

const oppositeBorder = (borderIndex: number) => {
  switch(borderIndex) {
    case 0:
      return 2
    case 1:
      return 3
    case 2:
      return 0
    case 3:
      return 1
  }
  throw new Error('O=H noes')
}

const arrangeTiles = (tiles: Tile[]): Tile[] => {
  const allTiles = tiles.reduce((acc, tile) => {
    acc[tile.id] = tile
    return acc
  }, {} as Record<number, Tile>)
  const checkedTiles: Tile[] = []
  let remainingTiles: Tile[] = [...tiles]

  const tileStack: Tile[] = []
  tileStack.push(tiles[0])

  while(tileStack.length) {
    const currentTile = tileStack.pop()
    if(currentTile) {
      remainingTiles = remainingTiles.filter(t => t.id !== currentTile.id)
      const currentBorders = currentTile.getAllBorders()
      remainingTiles.forEach(tile => {
        tile.getAllBorders().forEach((border, borderIndex) => {
          if (currentBorders.includes(border)) {
            const iCurrent = currentBorders.indexOf(border)
            const rotations = ((iCurrent + 4) - oppositeBorder(borderIndex)) % 4
            
            if (currentTile.getNeighbour(iCurrent) === undefined) {
                allTiles[tile.id]!!.rotateRight(rotations)
                if (iCurrent == 0 || iCurrent == 2) allTiles[tile.id]!!.flipHorizontal()
                else allTiles[tile.id]!!.flipVertical()

                currentTile.setNeighbour(iCurrent, tile.id)
                tile.setNeighbour(oppositeBorder(iCurrent), currentTile.id)
                tileStack.push(tile)
              }
            } else if (currentBorders.includes(reverseString(border))) {
              const iCurrent = currentBorders.indexOf(reverseString(border))
              const rotations = ((iCurrent + 4) - oppositeBorder(borderIndex)) % 4
              
              if (currentTile.getNeighbour(iCurrent) === undefined) {
                  allTiles[tile.id]!!.rotateRight(rotations)
                  currentTile.setNeighbour(iCurrent, tile.id)
                  tile.setNeighbour(oppositeBorder(iCurrent), currentTile.id)
                  tileStack.push(tile)
              }
          }
        })
      })

      checkedTiles.push(currentTile)
    }
  }

  return tiles
}

const generateImage = (tiles: Tile[]) => {
  const allTiles = tiles.reduce((acc, tile) => {
    acc[tile.id] = tile
    return acc
  }, {} as Record<number, Tile>)

  const size = Math.sqrt(tiles.length) | 0

  let left = tiles.find(tile => !tile.left && !tile.top)

  const imageTile = new Tile(0, [])
  let rowStart = 0
  if(left) {
    for(let i = 0; i < size; i++) {
      let right = left!!
      right.removeBorder()
      rowStart = imageTile.grid.length
      imageTile.grid = [...imageTile.grid, ...right.grid]
      right = allTiles[right?.right!]
      
      for(let j = 1; j < size; j++) {
        right.removeBorder()
        imageTile.grid = imageTile.grid.map((row, rowIndex) => {
          if(rowIndex >= rowStart){
            return [...row, ...right.grid[rowIndex - rowStart]]
          }
          return row
        })
        right = allTiles[right?.right!]
      }
      left = allTiles[left.bottom!]
    }
    
    return imageTile
  } else {
    console.log('NO LKEFT')
  }

  return undefined
}

const isSeaMonster = (image: Tile, x: number, y: number): boolean => {
  if(image.isBlocked(x, y)) {
    const isMonster = (
      image.isBlocked(x + 18, y - 1) &&
      image.isBlocked(x + 5, y) &&
      image.isBlocked(x + 6, y) &&
      image.isBlocked(x + 11, y) &&
      image.isBlocked(x + 12, y) &&
      image.isBlocked(x + 17, y) &&
      image.isBlocked(x + 18, y) &&
      image.isBlocked(x + 19, y) &&
      image.isBlocked(x + 1, y + 1) &&
      image.isBlocked(x + 4, y + 1) &&
      image.isBlocked(x + 7, y + 1) &&
      image.isBlocked(x + 10, y + 1) &&
      image.isBlocked(x + 13, y + 1) &&
      image.isBlocked(x + 16, y + 1)
    )

    if(isMonster) {
      image.markMonster(x, y)
      image.markMonster(x + 18, y - 1)
      image.markMonster(x + 5, y)
      image.markMonster(x + 6, y)
      image.markMonster(x + 11, y)
      image.markMonster(x + 12, y)
      image.markMonster(x + 17, y)
      image.markMonster(x + 18, y)
      image.markMonster(x + 19, y)
      image.markMonster(x + 1, y + 1)
      image.markMonster(x + 4, y + 1)
      image.markMonster(x + 7, y + 1)
      image.markMonster(x + 10, y + 1)
      image.markMonster(x + 13, y + 1)
      image.markMonster(x + 16, y + 1)
    }
    return isMonster
  }
  return false
}

const countSeaMonsters = (image: Tile): number => {
  let count = 0
  for(let y = 1; y <= image.grid.length - 1; y++) {
    for (let x = 0; x <= image.grid.length - 19; x++) {
      if(isSeaMonster(image, x, y)) count++
    }
  }
  return count
}

const findAllSeaMonsters = (image: Tile): number => {
  for (let index = 0; index < 4; index++) {
    let count = countSeaMonsters(image)
    if(count) return count

    image.flipVertical()
    count = countSeaMonsters(image)
    if(count) return count

    image.flipHorizontal()
    count = countSeaMonsters(image)
    if(count) return count

    image.flipVertical()
    count = countSeaMonsters(image)
    if(count) return count

    image.flipHorizontal()
    image.rotateRight()
  }
  return 0
}

export default async () => {
  const tiles: Array<Tile> = []
  let tileNo = ''
  let rows: Array<string> = []

  await readFileByLine('day-20/input.txt', line => {
    if(!line) {
      const grid = rows.map(row => row.split(''))
      tiles.push(new Tile(parseInt(tileNo), grid))
      rows = []
    } else {
      if(line.includes('Tile')) {
        tileNo = line.substring(5, line.length - 1)
      } else {
        rows.push(line)
      }
    }
  })
  const grid = rows.map(row => row.split(''))
  tiles.push(new Tile(parseInt(tileNo), grid))


  arrangeTiles(tiles)
  const image = generateImage(tiles)
  if(image) {
    const monsters = findAllSeaMonsters(image)
    require('fs').writeFileSync('day-20/image.txt', image.toString())
    if(monsters) {
      image.rotateRight()
      require('fs').writeFileSync('day-20/image.html', image.toHtml())
      return image.grid.reduce((sum, row) => sum + row.filter(c => c === '#').length, 0)
    }
  }
  return 0
}
