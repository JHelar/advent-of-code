import fs from 'fs'
import { Portal, Node, getNodeKey } from './node'
import { aStar } from './aStar'

const labelPerRow: Record<number, { name: string[], rowIndex: number, colIndex: number }[]> = {}
const labelPerColumn: Record<number, { name: string[], rowIndex: number, colIndex: number }[]> = {}
const stringMap = fs.readFileSync('day-20/test.txt').toString().split('\n').map((row, rowIndex) => {
    const column: string[] = []
    row.split('').forEach((c, colIndex) => {
        if(c === '.' || c === '#') {
            column.push(c)
        } else if(c !== ' ') {
            if(!(colIndex in labelPerColumn)) {
                labelPerColumn[colIndex] = []
            }
            
            const colLabel = labelPerColumn[colIndex].find(c => c.rowIndex + 1 === rowIndex)
            if(colLabel) {
                colLabel.name.push(c)
            } else {
                labelPerColumn[colIndex].push({
                    name: [c],
                    rowIndex,
                    colIndex,
                })
            }

            if(!(rowIndex in labelPerRow)) {
                labelPerRow[rowIndex] = []
            }
            const rowLabel = labelPerRow[rowIndex].find(r => r.colIndex + 1 === colIndex)
            if(rowLabel) {
                rowLabel.name.push(c)
            } else {
                labelPerRow[rowIndex].push({
                    name: [c],
                    colIndex,
                    rowIndex
                })
            }
        }
    })
    return column
}).filter(row => row.length)

const labels = [...Object.values(labelPerColumn).flatMap((labels) => labels.map(({ name, colIndex, rowIndex }) => ({
    name: name.join(''),
    x: colIndex - 2,
    y: rowIndex + (rowIndex + 2 < stringMap.length && stringMap[rowIndex + 2][colIndex - 2] ? 2 : rowIndex - 1 > 0 && stringMap[rowIndex - 1][colIndex - 2] ? -1 : 0)
}))),...Object.values(labelPerRow).flatMap((labels) => labels.map(({ name, colIndex, rowIndex }) => ({
    name: name.join(''),
    x: colIndex + (colIndex + 2 < stringMap[0].length && stringMap[rowIndex - 2][colIndex + 2] ? 2 : colIndex - 1 > 0 && stringMap[rowIndex - 2][colIndex - 1] ? -1 : 0),
    y: rowIndex - 2
})))].filter(({ name }) => name.length === 2)

const map = stringMap.reduce((map, row, rowIndex) => {
    row.forEach((col, columnIndex) => {
        const label = labels.find((l) => l.x === columnIndex && l.y === rowIndex)
        if(label) {
            const node = new Portal(columnIndex, rowIndex, label.name)
            map[node.getKey()] = node
        } else {
            const node = new Node(columnIndex, rowIndex, col)
            map[node.getKey()] = node
        }
    })
    return map
}, {} as Record<string, Node>)

const nodes = Object.values(map).map((node, ni, nodes) => {
    node.neighbours = [
        [0, 1],
        [0, -1],
        [1, 0],
        [-1, 0]
    ].map(([x, y]) => map[getNodeKey(node.x + x, node.y + y)]).filter(n => n && n.road)
    if(node instanceof Portal) {
        const portalEnd = nodes.find(n => n instanceof Portal && n.name === node.name && n.getKey() !== node.getKey())
        if(portalEnd) {
            [
                [portalEnd.x, portalEnd.y + 1],
                [portalEnd.x, portalEnd.y - 1],
                [portalEnd.x + 1, portalEnd.y],
                [portalEnd.x - 1, portalEnd.y]
            ]
            .map(([x, y]) => map[getNodeKey(node.x + x, node.y + y)])
            .filter(n => n && n.road)
            .forEach(n => node.neighbours.push(n))
        }
    }
    return node
}).filter(node => node.road)

const path = aStar(nodes)
let count = 0
if(path) {
    let lookAt = path.prev
    while(lookAt) {
        count++
        lookAt = lookAt.prev
    }
}
// console.log({
//     count
// })
nodes.forEach(n => {
    if(n instanceof Portal) {
        console.log(n.toString())
    }
})