// @ts-nocheck
import { rotate90, hflip, vflip } from '2d-array-rotation'

export default class Tile {
    id: number
    top?: number
    right?: number
    bottom?: number
    left?: number
    grid: string[][]

    constructor(id: number, grid: string[][]) {
        this.grid = grid
        this.id = id
    }

    isBlocked(x, y) {
        return this.grid[x][y] === '#' || this.grid[x][y] === 'O'
    }

    markMonster(x, y) {
        this.grid[x][y] = 'O'
    }

    removeBorder() {
        this.grid = this.grid.slice(1, this.grid.length - 1).map(col => col.slice(1, col.length - 1))
    }

    getAllBorders() {
        const borders: string[] = []

        borders.push(this.grid[0].join(''))
        borders.push(this.grid.map(row => row[row.length - 1]).join(''))
        borders.push([...this.grid[this.grid.length - 1]].reverse().join(''))
        borders.push(this.grid.map(col => col[0]).reverse().join(''))

        return borders
    }

    rotateRight(times: number = 1) {
        for (let index = 0; index < times; index++) {
            this.grid = rotate90(this.grid)
        }
    }

    flipHorizontal() {
        this.grid = hflip(this.grid)
    }

    flipVertical() {
        this.grid = vflip(this.grid)
    }

    setNeighbour(borderIndex: number, tileId: number) {
        switch(borderIndex) {
            case 0:
                this.top = tileId
                break
            case 1:
                this.right = tileId
                break
            case 2:
                this.bottom = tileId
                break
            case 3:
                this.left = tileId
                break
        }
    }

    getNeighbour(borderIndex: number) {
        switch(borderIndex) {
            case 0:
                return this.top
            case 1:
                return this.right
            case 2:
                return this.bottom
            case 3:
                return this.left
        }
    }

    toString() {
        return this.grid.map(row => row.join('')).join('\n')
    }

    toHtml() {
        return `
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Sea monsters!</title>
            <style>
        body {
            background-color: #333;
            display: flex;
            width: 100%;
            flex-flow: column;
        }
        .row {
            display: flex;
            width: 100%;
            flex-flow: row nowrap;
        }
        .node {
            display: block;
            width: 10px;
            height: 10px;
        }
        .node-sea {
            background-color: #001720;
        }
        .node-rock {
            background-color: #001118;
        }
        .node-monster {
            background-color: #001720;
            position: relative;
        }
        .node-monster::after {
            position: absolute;
            content: '';
            background-color: seagreen;
            border-radius: 50%;
            display: block;
            top: 0;
            bottom: 0;
            left: 0;
            right: 0;
        }
    </style>
        </head>
        <body>
            ${this.grid.map(row => `<div class="row">${row.map(c => `<span class="node node-${c === '.' ? 'sea' : c === '#' ? 'rock' : 'monster'}"></span>`).join('')}</div>`).join('\n')}
        </body>
        </html>
        `
    }
}