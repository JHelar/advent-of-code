import { PRIORITY_BELOW_NORMAL } from 'constants'
import { verify } from 'crypto'
import { connect } from 'http2'
import { runInThisContext } from 'vm'
import { readFileWithSeparator } from '../utils'

type Action = 'N' | 'S' | 'E' | 'W' | 'L' | 'R' | 'F'

interface Command {
    action: Action,
    units: number
}

interface Position {
    horizontal: number
    vertical: number
}

const parseCommand = (input: string) => ({
    action: input.slice(0, 1),
    units: parseInt(input.slice(1))
} as Command)

const rotate = ({ horizontal, vertical }: Position, B: Position, angle: number): Position => {
    const rad = (Math.PI / 180) * angle
    const newX = Math.cos(rad) * horizontal - vertical * Math.sin(rad)
    const newY = Math.sin(rad) * horizontal - vertical * Math.cos(rad)
    return {
        horizontal: Math.round(newX),
        vertical: Math.round(newY),
    }
}

export default () => {
    const commands = readFileWithSeparator('day-12/input.txt', '\n').map(parseCommand)

    let shipPosition: Position = {
        horizontal: 0,
        vertical: 0
    }

    let waypointPosition: Position = {
        horizontal: 10,
        vertical: 1
    }

    commands.forEach(({ action, units }) => {
        console.log({
            waypointPosition,
            shipPosition,
            action
        })
        switch(action) {
            case 'N':
                waypointPosition.vertical += units
                break
            case 'S':
                waypointPosition.vertical -= units
                break
            case 'E':
                waypointPosition.horizontal += units
                break
            case 'W':
                waypointPosition.horizontal -= units
                break
            case 'L':
                waypointPosition = rotate(waypointPosition, shipPosition, units)
                break
            case 'R':
                waypointPosition = rotate(waypointPosition, shipPosition, units * -1)
                break
            case 'F':
            default:
                shipPosition.horizontal += units * waypointPosition.horizontal
                shipPosition.vertical += units * waypointPosition.vertical
                break

        }
    })


    return Math.abs(shipPosition.horizontal) + Math.abs(shipPosition.vertical)
}