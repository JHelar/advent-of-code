const { DIRECTIONS, posFromDirection } = require('./direction')
const makeProgram = require('./read-incode');
const aStar = require('./a-star');
const flood = require('./flood');

const input = require('../utils').readArray('./day-15/input.txt');

const toPixel = node =>
    `<span style="--x:${node.x};--x-end:${node.x + 1};--y:${
    node.y
    };--y-end:${node.y + 1};--color:${
    node.color
    };--color-text:${node.color}">${node.steps ? node.steps: ''}</span>`;
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
            grid-template-columns: repeat(${width}, 20px);
            grid-template-rows: repeat(${height}, 20px);
        }
        span {
            display: block;
            grid-column: var(--x);
            grid-row: var(--y);
			background-color: var(--color);
        }
    </style>
</head>
<body>
    ${pixels}
</body>
</html>`;

const getMapKey = ({ x, y }) => `(${x},${y})`;
let findStart = false;

const doWalk = mapValue => mapValue !== '#';
const getNextDir = (map, direction, position) => {
    const potentialNorth = posFromDirection(position, DIRECTIONS.NORTH);
    const potentialSouth = posFromDirection(position, DIRECTIONS.SOUTH);
    const potentialEast = posFromDirection(position, DIRECTIONS.EAST);
    const potentialWest = posFromDirection(position, DIRECTIONS.WEST);

    const northValue = map[getMapKey(potentialNorth)];
    if(northValue === undefined) return DIRECTIONS.NORTH;

    const southValue = map[getMapKey(potentialSouth)];
    if(southValue === undefined) return DIRECTIONS.SOUTH;

    const eastValue = map[getMapKey(potentialEast)];
    if(eastValue === undefined) return DIRECTIONS.EAST;

    const westValue = map[getMapKey(potentialWest)];
    if(westValue === undefined) return DIRECTIONS.WEST;


    switch (direction) {
        case DIRECTIONS.NORTH:
            if (doWalk(map[getMapKey(potentialWest)])) {
                return DIRECTIONS.WEST;
            }
            if (doWalk(map[getMapKey(potentialNorth)])) {
                return DIRECTIONS.NORTH;
            }
            if (doWalk(map[getMapKey(potentialEast)])) {
                return DIRECTIONS.EAST;
            }
            return DIRECTIONS.SOUTH;
        case DIRECTIONS.SOUTH:
            if (doWalk(map[getMapKey(potentialEast)])) {
                return DIRECTIONS.EAST;
            }
            if (doWalk(map[getMapKey(potentialSouth)])) {
                return DIRECTIONS.SOUTH;
            }
            if (doWalk(map[getMapKey(potentialWest)])) {
                return DIRECTIONS.WEST;
            }
            return DIRECTIONS.NORTH;
        case DIRECTIONS.EAST:
            if (doWalk(map[getMapKey(potentialNorth)])) {
                return DIRECTIONS.NORTH;
            }
            if (doWalk(map[getMapKey(potentialEast)])) {
                return DIRECTIONS.EAST;
            }
            if (doWalk(map[getMapKey(potentialSouth)])) {
                return DIRECTIONS.SOUTH;
            }
            return DIRECTIONS.WEST;
        case DIRECTIONS.WEST:
            if (doWalk(map[getMapKey(potentialSouth)])) {
                return DIRECTIONS.SOUTH;
            }
            if (doWalk(map[getMapKey(potentialWest)])) {
                return DIRECTIONS.WEST;
            }
            if (doWalk(map[getMapKey(potentialNorth)])) {
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
while (true) {
    output = program.run(currentDir);
    output = output | 0;

    const nextPosition = posFromDirection(currentPos, currentDir);
    if(findStart && theMap[getMapKey(currentPos)] === 'S') {
        break;
    }
    if (output === 0) {
        theMap[getMapKey(nextPosition)] = '#';
    } else if (output === 1) {
        if(theMap[getMapKey(nextPosition)] !== 'S') {
            theMap[getMapKey(nextPosition)] = '.'
        }
        currentPos = nextPosition;
    } else if (output === 2) {
        theMap[getMapKey(nextPosition)] = 'W'
        currentPos = nextPosition;
        findStart = true;
    }
    currentDir = getNextDir(theMap, currentDir, currentPos);
}


let mapNodes = Object.keys(theMap).map(key => {
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

// Create graph
const mapNodesWithKeys = mapNodes.reduce((m, node) => {
    m[getMapKey(node)] = node;
    return m
}, {});

// Const set node neighbours
mapNodes.forEach(node => {
    const potentialNorth = posFromDirection(node, DIRECTIONS.NORTH);
    const potentialSouth = posFromDirection(node, DIRECTIONS.SOUTH);
    const potentialEast = posFromDirection(node, DIRECTIONS.EAST);
    const potentialWest = posFromDirection(node, DIRECTIONS.WEST);

    const neighbours = [potentialNorth, potentialSouth, potentialEast, potentialWest].map(pos => mapNodesWithKeys[getMapKey(pos)]).filter(n => n ? n.value !== '#' : false);

    Object.assign(node, {
        neighbours
    })
})

const startPath = aStar(mapNodes.filter(n => n.value !== '#'));
let count = 0;
let lookAt = startPath.prev;
while(lookAt.node.value !== 'S') {
    count++
    lookAt.node.color = 'red';
    lookAt = lookAt.prev;
}
console.log({
    count
})

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

const steps = flood(mapNodesWithKeys);

const floodPixels = mapNodes
    .map(node => ({
        ...node,
        x: node.x + Math.abs(minWidth) + 1,
        y: node.y + Math.abs(minHeight) + 1
    }))
    .map(toPixel)
    .join('');

const floodDoc = toHtmlDocument(floodPixels, mapWidth + 1, mapHeight + 1);
require('fs').writeFileSync('./day-15/map-flood.html', floodDoc);

console.log({
    steps
})