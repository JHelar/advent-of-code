const COLORS = {
    '0': 'BLACK',
    '1': 'WHITE',
    BLACK: '0',
    WHITE: '1',
}
const DIRECTION = {
    UP: { x: 0, y: -1, name: 'UP' },
    RIGHT: { x: 1, y: 0, name: 'RIGHT' },
    DOWN: { x: 0, y: 1, name: 'DOWN' },
    LEFT: { x: -1, y: 0, name: 'LEFT' },
}

const ROBOT_DIRECTION_COMMAND = {
    '0': currentDirection => getNextDirection(currentDirection, 1),
    '1': currentDirection => getNextDirection(currentDirection, -1)
}

const getNextDirection = (fromDirection, dir) => {
    const directionKeys = Object.keys(DIRECTION);
    const myDirectionIndex = directionKeys.findIndex(k => k === fromDirection);
    let nextDirectionIndex = myDirectionIndex + dir;
    if (nextDirectionIndex < 0) {
        nextDirectionIndex = directionKeys.length + nextDirectionIndex;
    } else if (nextDirectionIndex >= directionKeys.length) {
        nextDirectionIndex = nextDirectionIndex % directionKeys.length;
    }
    const toDirection = directionKeys[nextDirectionIndex];
    return DIRECTION[toDirection];
}

const getMapKey = (x, y) => x * 100 + y;

const robotMap = (read, write) => new Promise(res => {
    let currentDirection = DIRECTION.UP;
    let positionX = 50;
    let positionY = 50;
    let map = {}
    let command = 0;
    let paintColor = COLORS['0']
    let commandColor = COLORS['0'];

    write.push('0\n');

    const writeToRobot = () => {
        // First color
        const node = map[getMapKey(positionX, positionY)];
        if (node) {
            commandColor = node.color;
            node.color = paintColor;
        } else {
            map[getMapKey(positionX, positionY)] = {
                x: positionX,
                y: positionY,
                color: paintColor
            };
        }
        // Then move
        positionX = positionX + currentDirection.x;
        positionY = positionY + currentDirection.y;

        write.push(`${COLORS[commandColor]}\n`);
    }

    read.on('data', robotData => {
        const code = robotData.toString();
        if (code === 'EXIT') {
            res(map);
            return;
        }
        if (command === 0) { // Set color to map
            paintColor = COLORS[code];
            command = 1;
        } else {
            currentDirection = ROBOT_DIRECTION_COMMAND[code](currentDirection.name);
            command = 0;

            // Write back to robot
            writeToRobot();
        }
    })
})

module.exports = robotMap;