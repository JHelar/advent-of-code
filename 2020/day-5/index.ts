import { readFileWithSeparator } from "../utils"

const createFinder = (lowerId: string, upperStart: number) => {
    const finder = (id: string[], upper: number, lower: number) => {
        const lookup = id.shift()
        if(!id.length) {
            if(lookup === lowerId) return lower
            return upper
        }
    
        const halfPoint = (upper - lower) / 2
        if(lookup === lowerId) return finder(id, Math.floor(upper - halfPoint), lower)
        return finder(id, upper, Math.ceil(lower + halfPoint))
    }

    return (input: string[]) => finder(input, upperStart, 0)
}

const getSeatId = ({ row, column }: { row:number, column: number }) => row * 8 + column

const part1 = () => {
    const findRow = createFinder('F', 127)
    const findColumn = createFinder('L', 7)
    
    return readFileWithSeparator('day-5/input.txt', '\n').map(input => ({
        row: findRow(input.slice(0, 7).split('')),
        column: findColumn(input.slice(7).split(''))
    })).reduce((highest, curr) => {
        const seatId = getSeatId(curr)
        if(highest < seatId) return seatId
        return highest
    }, 0)
}

export default () => {
    const findRow = createFinder('F', 127)
    const findColumn = createFinder('L', 7)
    
    const ids = readFileWithSeparator('day-5/input.txt', '\n').map(input => ({
        row: findRow(input.slice(0, 7).split('')),
        column: findColumn(input.slice(7).split(''))
    })).map(getSeatId).sort()

    for (let i = 0; i < ids.length; i++) {
        const prevId = ids[i - 1]
        const currentId = ids[i]
        const nextId = ids[i + 1]
      
        if(prevId && currentId - 1 !== prevId) {
            return currentId -1
        }
        if(nextId && currentId + 1 !== nextId) {
            return currentId + 1
        }
    }
    return -1
}