const DIRECTIONS = {
    NORTH: 94,
    SOUTH: 118,
    WEST: 60,
    EAST: 62,
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

const getMapKey = ({ x, y }) => `(${x},${y})`;
const doWalk = mapValue => {
    if(mapValue && mapValue.value !== '.') {
        return true;
    }
    return false
};
const nextDirection = (from, toDir) => {
    switch(from) {
        case DIRECTIONS.NORTH:
            if(toDir === DIRECTIONS.WEST) {
                return DIRECTIONS.WEST;
            } else {
                return DIRECTIONS.EAST;
            }
        case DIRECTIONS.SOUTH:
            if(toDir === DIRECTIONS.WEST) {
                return DIRECTIONS.EAST;
            } else {
                return DIRECTIONS.WEST;
            }
        case DIRECTIONS.EAST:
            if(toDir === DIRECTIONS.EAST) {
                return DIRECTIONS.SOUTH;
            } else {
                return DIRECTIONS.NORTH;
            }
        case DIRECTIONS.WEST:
            if(toDir === DIRECTIONS.EAST) {
                return DIRECTIONS.NORTH;
            } else {
                return DIRECTIONS.SOUTH;
            }
    }
}

const navigateMap = theMap => {
    const playerDirections = ['^','v','<','>'].map(c => c.charCodeAt(0));
    const roadNodes = Object.values(theMap).filter(node => node.value !== '.');
    const playerNode = roadNodes.find(node => playerDirections.some(pd => pd === node.code));
    let position = playerNode.pos;
    let direction = playerNode.code;
    let steps = 0;
    const movement = [];
    
    while (true) {
        const nextPosition = posFromDirection(position, direction);
        Object.assign(theMap[getMapKey(position)], {
            visited: true
        })
        if(!doWalk(theMap[getMapKey(nextPosition)])) {
            movement.push(steps.toString())
            steps = 0;
            const left = posFromDirection(position, nextDirection(direction, DIRECTIONS.WEST));
            const right = posFromDirection(position, nextDirection(direction, DIRECTIONS.EAST));
       
            if(doWalk(theMap[getMapKey(left)])) {
                direction = nextDirection(direction, DIRECTIONS.WEST)
                position = posFromDirection(position, direction);
                steps++;
                movement.push('L');
            } else if(doWalk(theMap[getMapKey(right)])) {
                direction = nextDirection(direction, DIRECTIONS.EAST)
                position = posFromDirection(position, direction);
                steps++;
                movement.push('R');
            } else {
                movement.push(steps.toString());
                break;
            }
        } else {
            steps++;
            position = posFromDirection(position, direction);
        }
    }
    movement.shift();
    movement.pop();
    return movement.join(',');
}

module.exports = navigateMap;