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
    x: number
    y: number
}

const parseCommand = (input: string) => ({
    action: input.slice(0, 1),
    units: parseInt(input.slice(1))
} as Command)

export default () => {
    const commands = readFileWithSeparator('day-12/input.txt', '\n').map(parseCommand)

    let shipPosition: Position = {
        x: 0,
        y: 0
    }

    let waypointPosition: Position = {
        x: 10,
        y: 1
    }

    commands.forEach(({ action, units }) => {
        let nwx, nwy;
        switch(action) {
            case 'N':
                waypointPosition.y += units
                break
            case 'S':
                waypointPosition.y -= units
                break
            case 'E':
                waypointPosition.x += units
                break
            case 'W':
                waypointPosition.x -= units
                break
            case 'L':
                nwx = waypointPosition.x * Math.cos(units * Math.PI / 180) - waypointPosition.y * Math.sin(units * Math.PI / 180);
                nwy = waypointPosition.y * Math.cos(units * Math.PI / 180) + waypointPosition.x * Math.sin(units * Math.PI / 180);
                waypointPosition.x = nwx
                waypointPosition.y = nwy
                break
            case 'R':
                nwx = waypointPosition.x * Math.cos(units * Math.PI / 180) + waypointPosition.y * Math.sin(units * Math.PI / 180);
                nwy = waypointPosition.y * Math.cos(units * Math.PI / 180) - waypointPosition.x * Math.sin(units * Math.PI / 180);
                waypointPosition.x = nwx
                waypointPosition.y = nwy
                break
            case 'F':
                const moveX = units * waypointPosition.x
                const moveY = units * waypointPosition.y
                shipPosition.x += moveX
                shipPosition.y += moveY
                break

        }
    })


    return Math.abs(shipPosition.x) + Math.abs(shipPosition.y)
}