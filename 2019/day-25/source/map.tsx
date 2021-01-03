import React, { FC, useRef, useEffect, useState } from 'react'
import { Box } from 'ink'

interface Room {
    x: number
    y: number
    active: boolean
    north: boolean
    south: boolean
    east: boolean
    west: boolean
    visited: boolean
}

interface Node {
    x: number
    y: number
    door: number
    active: boolean
    visited: boolean
}

const getPointFromVector = (point: Room, [x, y]: [number, number]): [ x: number, y: number ] => ([ point.x + x, point.y + y ])
const getMapKey = ([x, y]: [number, number]) => `${x},${y}`

export const MapView: FC<{ output: string[], command: string }> = ({ output, command }) => {
    const currentRoom = useRef<Room>({ x: 0, y: 0, north: false, south: false, east: false, west: false, active: true, visited: true })
    const map: Record<string, Room> = useRef({}).current
    const [pov, setPov] = useState<Node[][]>([])

    let relativeY = -1
    let relativeX = -1

    const setMapPov = () => {
        let newPov: Node[][] = Array(9).fill(0).map(() => Array(9).fill(0).map(() => ({
            active: false,
            door: 0,
            visited: false,
            x: 0,
            y: 0
        })))
        for (let y = 0; y < 3; y++) {
            for (let x = 0; x < 3; x++) {
                let room = map[getMapKey(getPointFromVector(currentRoom.current, [relativeX, relativeY]))]
                for (let innerY = 0; innerY < 3; innerY++) {
                    const roomY = (y * 3) + innerY
                    for (let innerX = 0; innerX < 3; innerX++) {
                        const roomX = (x * 3) + innerX
                        let node = newPov[roomY]![roomX]!
                        if((relativeX === 0 && relativeY === -1 || relativeX === -1 && relativeY === 0 || relativeX === 0 && relativeY === 0 || relativeX === 1 && relativeY === 0 || relativeX === 0 && relativeY === 1) && room) {
                            if(innerY === 0 && innerX === 1) {
                                node.door = room.visited ? room.north ? 1 : -1 : 0
                            } else if(innerY === 1 && innerX === 0) {
                                node.door = room.visited ? room.west ? 1 : -1 : 0
                            } else if(innerY === 1 && innerX === 2) {
                                node.door = room.visited ? room.east ? 1 : -1 : 0
                            } else if(innerY === 2 && innerX === 1) {
                                node.door = room.visited ? room.south ? 1 : -1 : 0
                            } else if(innerY === 1 && innerX === 1) {
                                node.visited = room.visited
                                node.door = 1
                                node.active = relativeY === 0 && relativeX === 0
                            } else {
                                node.door = -1
                            }
                        }
                    }
                }
                relativeX++
                if(relativeX > 1) {
                    relativeX = -1
                }
            }
            relativeY++
        }
        setPov(newPov)
    }

    useEffect(() => {
        map[getMapKey([currentRoom.current.x, currentRoom.current.y])] = currentRoom.current
    })

    useEffect(() => {
        if(command) {
            let newRoom: string | undefined = undefined
            switch (command) {
                case 'north':
                    newRoom = getMapKey(getPointFromVector(currentRoom.current, [0, -1]))
                    break;
                case 'south':
                    newRoom = getMapKey(getPointFromVector(currentRoom.current, [0, 1]))
                    break
                case 'east':
                    newRoom = getMapKey(getPointFromVector(currentRoom.current, [1, 0]))
                    break
                case 'west':
                    newRoom = getMapKey(getPointFromVector(currentRoom.current, [-1, 0]))
                    break
                default:
                    break;
            }
            if(newRoom) {
                if(newRoom in map) {
                    map[getMapKey([currentRoom.current.x, currentRoom.current.y])]!.active = false
                    map[getMapKey([currentRoom.current.x, currentRoom.current.y])]!.visited = true
                    map[newRoom]!.active = true
                    map[newRoom]!.visited = true
                    currentRoom.current = map[newRoom]!
                }
            }
        }

        const doorsLeadIndex = output.findIndex(o => o.startsWith('Doors here lead:'))
        if(doorsLeadIndex > -1) {
            const spaceIndex = output.indexOf(' ', doorsLeadIndex)
            const positions = output.slice(doorsLeadIndex + 1, spaceIndex - 2)
            currentRoom.current.north = false
            currentRoom.current.south = false
            currentRoom.current.east = false
            currentRoom.current.west = false
            positions.map(p => p.slice(2)).forEach(pos => {
                let newRoom: [number, number] | undefined = undefined

                switch (pos) {
                    case 'north':
                        newRoom = getPointFromVector(currentRoom.current, [0, -1])
                        currentRoom.current.north = true
                        break;
                    case 'south':
                        newRoom = getPointFromVector(currentRoom.current, [0, 1])
                        currentRoom.current.south = true
                        break
                    case 'east':
                        newRoom = getPointFromVector(currentRoom.current, [1, 0])
                        currentRoom.current.east = true
                        break
                    case 'west':
                        newRoom = getPointFromVector(currentRoom.current, [-1, 0])
                        currentRoom.current.west = true
                        break
                    default:
                        break;
                }
                if(newRoom) {
                    const newRoomKey = getMapKey(newRoom)
                    if(!(newRoomKey in map)) {
                        map[newRoomKey] = {
                            north: false,
                            south: false,
                            east: false,
                            west: false,
                            active: false,
                            visited: false,
                            x: newRoom[0],
                            y: newRoom[1]
                        }
                    }
                }
            })
        }
        setMapPov()
    }, [output, command])


    return <Box flexDirection="column" borderColor="cyanBright" borderStyle="classic">
        { pov.map((row, rowIndex) => <Box key={`row${rowIndex}`} flexDirection="row">{row.map((t, tileIndex) => <Box key={`row${rowIndex}-tile${tileIndex}`} width={1} height={1} borderColor={t.active ? 'greenBright' : t.visited ? 'cyan' : t.door === 1 ? 'white' : t.door === -1 ? 'red' : 'black'} borderStyle="double"></Box>)}</Box>) }
    </Box>
}