const makeProgram = require('./read-incode');

const input = require('../utils').readArray('./day-15/input.txt');

const toPixel = node =>
    `<span style="--x:${node.x};--x-end:${node.x + 1};--y:${
    node.y
    };--y-end:${node.y + 1};--color:${
    node.color
    };--color-text:${node.color}"></span>`;
const toHtmlDocument = (pixels, width, height) => `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta http-equiv="X-UA-Compatible" content="ie=edge">
    <title>Document</title>
    <style>
        body {
            background-color: rebeccapurple;
            height: 100vh;
            width: 100vw;
            display: grid;
            grid-template-columns: repeat(${width}, 10px);
            grid-template-rows: repeat(${height}, 10px);
        }
        span {
            display: block;
            grid-column: var(--x);
            grid-row: var(--y);
			background-color: var(--color);
			color: var(--color-text);
        }
    </style>
</head>
<body>
    ${pixels}
</body>
</html>`;

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

const getMapKey = ({ x, y }) => `(${x},${y})`;
const getNextDir = (map, direction, position) => {
    const potentialNorth = posFromDirection(position, DIRECTIONS.NORTH);
    const potentialSouth = posFromDirection(position, DIRECTIONS.SOUTH);
    const potentialEast = posFromDirection(position, DIRECTIONS.EAST);
    const potentialWest = posFromDirection(position, DIRECTIONS.WEST);
    switch (direction) {
        case DIRECTIONS.NORTH:
            if (map[getMapKey(potentialWest)] !== '#') {
                return DIRECTIONS.WEST;
            }
            if (map[getMapKey(potentialNorth)] !== '#') {
                return DIRECTIONS.NORTH;
            }
            if (map[getMapKey(potentialEast)] !== '#') {
                return DIRECTIONS.EAST;
            }
            return DIRECTIONS.SOUTH;
        case DIRECTIONS.SOUTH:
            if (map[getMapKey(potentialEast)] !== '#') {
                return DIRECTIONS.EAST;
            }
            if (map[getMapKey(potentialSouth)] !== '#') {
                return DIRECTIONS.SOUTH;
            }
            if (map[getMapKey(potentialWest)] !== '#') {
                return DIRECTIONS.WEST;
            }
            return DIRECTIONS.NORTH;
        case DIRECTIONS.EAST:
            if (map[getMapKey(potentialNorth)] !== '#') {
                return DIRECTIONS.NORTH;
            }
            if (map[getMapKey(potentialEast)] !== '#') {
                return DIRECTIONS.EAST;
            }
            if (map[getMapKey(potentialSouth)] !== '#') {
                return DIRECTIONS.SOUTH;
            }
            return DIRECTIONS.WEST;
        case DIRECTIONS.WEST:
            if (map[getMapKey(potentialSouth)] !== '#') {
                return DIRECTIONS.SOUTH;
            }
            if (map[getMapKey(potentialWest)] !== '#') {
                return DIRECTIONS.WEST;
            }
            if (map[getMapKey(potentialNorth)] !== '#') {
                return DIRECTIONS.NORTH;
            }
            return DIRECTIONS.EAST;
    }
}

const theMap = {}
let output;
let currentDir = DIRECTIONS.EAST;
let currentPos = {
    x: 0,
    y: 0,
}


const program = makeProgram(input);
theMap[getMapKey(currentPos)] = 'S';
while (output !== 2) {
    output = program.run(currentDir);
    output = output | 0;

    const nextPosition = posFromDirection(currentPos, currentDir);
    if (output === 0) {
        theMap[getMapKey(nextPosition)] = '#';
    } else if (output === 1) {
        if(theMap[getMapKey(nextPosition)] !== 'S') {
            theMap[getMapKey(nextPosition)] = '.'
        };
        currentPos = nextPosition;
    }
    currentDir = getNextDir(theMap, currentDir, currentPos);
}
theMap[getMapKey(currentPos)] = 'W';


const mapNodes = Object.keys(theMap).map(key => {
    // Destruct key to position
    const [x, y] = key.replace(/[\(\)]*/g, '').split(',');
    const value = theMap[key];

    const node = {
        x: x | 0,
        y: y | 0,
        color: value === '#' ? 'black' : value === 'W' ? 'green' : value === 'S' ? 'blue' : 'white',
        value,
        neighbours:[]
    }

    return node;
});
const minWidth = Math.min(...mapNodes.map(({ x }) => x));
const minHeight = Math.min(...mapNodes.map(({ y }) => y));

const mapWidth = Math.max(...mapNodes.map(({ x }) => x)) - minWidth;
const mapHeight = Math.max(...mapNodes.map(({ y }) => y)) - minHeight;

const pixels = mapNodes
    .map(node => ({
        ...node,
        x: node.x + Math.abs(minWidth) + 1,
        y: node.y + Math.abs(minHeight) + 1
    }))
    .map(toPixel)
    .join('');

const htmlDoc = toHtmlDocument(pixels, mapWidth + 1, mapHeight + 1);
require('fs').writeFileSync('./day-15/map.html', htmlDoc);