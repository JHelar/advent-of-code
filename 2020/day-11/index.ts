import * as fs from 'fs'
import { readFileWithSeparator } from '../utils'

const SEAT_OCCUPIED = '#'
const SEAT_EMPTY = 'L'
const SEAT_FLOOR = '.'

type SEAT_STATE = typeof SEAT_OCCUPIED | typeof SEAT_EMPTY | typeof SEAT_FLOOR

interface Seat {
    state: SEAT_STATE
    row: number
    col: number,
    adjacent: Seat[]
}

const getSeatAt = (map: Seat[][], row: number, col: number, rowVectorX: number, colVectorY: number): Seat | undefined => {
    let mapRow = row
    let mapCol = col
    while(true) {
        const r = map[mapRow]
        if(r) {
            const seat = r[mapCol]
            if(!seat) return undefined
            if(seat.state === SEAT_OCCUPIED || seat.state === SEAT_EMPTY) return seat
        } else {
            return undefined
        }

        mapRow += rowVectorX
        mapCol += colVectorY
    }
    
}

const getAdjacent = (map: Seat[][], { row, col }: Seat) => {
    return [
        [row - 1, col, -1, 0], 
        [row + 1, col, 1, 0], 
        [row, col + 1, 0, 1], 
        [row, col - 1, 0, -1], 
        [row - 1, col - 1, -1, -1], 
        [row - 1, col + 1, -1, 1], 
        [row + 1, col - 1, 1, -1],
        [row + 1, col + 1, 1, 1]
    ].reduce((acc, [sR, sC, xV, yV]) => {
        const seat = getSeatAt(map, sR, sC, xV, yV)
        if(seat) {
            acc.push(seat)
        }
        return acc
    },[] as Array<Seat>)
}

const getSeatState = (map: Seat[][], seat: Seat): SEAT_STATE => {
    const adjacent = getAdjacent(map, seat)

    const isEmpty = seat.state === SEAT_EMPTY && !adjacent.some(({ state }) => state === SEAT_OCCUPIED)
    if(isEmpty) return SEAT_OCCUPIED

    const isOccupied = seat.state === SEAT_OCCUPIED && adjacent.filter(({ state }) => state === SEAT_OCCUPIED).length >= 5
    if(isOccupied) return SEAT_EMPTY

    return seat.state
} 

export default () => {
    const map = readFileWithSeparator('day-11/input.txt', '\n').map((row, rowIndex) => row.split('').map((state, columnIndex) => ({
        row: rowIndex,
        col: columnIndex,
        state,
    } as Seat)))

    let lookupMap = map
    let run = 0
    let changeCount = 0
    let mapStates = []
    while(true) {
        mapStates.push(lookupMap)
        let newMap: Seat[][] = []
        for (let rowIndex = 0; rowIndex < map.length; rowIndex++) {
            const row = lookupMap[rowIndex];
            let newRow = []
            for (let columnIndex = 0; columnIndex < row.length; columnIndex++) {
                const seat = row[columnIndex];
                const newSeatState = getSeatState(lookupMap, seat)
               
                if(seat.state !== newSeatState) {
                    changeCount++
                }
                newRow.push({
                    ...seat,
                    state: newSeatState
                })
            }
            newMap.push(newRow)
            newRow = []
        }
        lookupMap = newMap
        newMap = []
        console.log({
            run,
            changeCount
        })
        if(changeCount === 0) {
            break;
        }
        changeCount = 0
        run++
    }
    mapStates.push(lookupMap)

    fs.writeFileSync('./day-11/mapStates.json', JSON.stringify(mapStates))
    return lookupMap.reduce((sum, row) => sum + row.filter(({ state }) => state === SEAT_OCCUPIED).length, 0)
}