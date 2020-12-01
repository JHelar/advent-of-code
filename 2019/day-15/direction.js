const DIRECTIONS = {
    1: 'NORTH',
    2: 'SOUTH',
    3: 'WEST',
    4: 'EAST',
    NORTH: 1,
    SOUTH: 2,
    WEST: 3,
    EAST: 4
}

const posFromDirection = (position, direction) => {
    switch (direction) {
        case DIRECTIONS.NORTH:
            return {
                x: position.x,
                y: position.y - 1
            }
        case DIRECTIONS.SOUTH:
            return {
                x: position.x,
                y: position.y + 1
            }
        case DIRECTIONS.EAST:
            return {
                x: position.x + 1,
                y: position.y
            }
        case DIRECTIONS.WEST:
            return {
                x: position.x - 1,
                y: position.y
            }
    }
}

module.exports = {
    DIRECTIONS,
    posFromDirection
}