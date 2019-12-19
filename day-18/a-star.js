const clonedeep = require('lodash.clonedeep')
const { getMapKey } = require('./get-map');
const { DIRECTIONS, posFromDirection } = require('./direction');

const getDistance = (onePoint, anotherPoint) => Math.sqrt(Math.pow(anotherPoint.x - onePoint.x, 2) + Math.pow(anotherPoint.y - onePoint.y, 2));
const sortByCost = (a, b) => (a.g + a.node.h) - (b.g + b.node.h);
const getSteps = (startPath, startNode) => {
    let count = 1;
    let lookAt = startPath.prev;
    while (getMapKey(lookAt.node) !== getMapKey(startNode)) {
        count++
        lookAt = lookAt.prev;
    }

    return count;
}

const getNeighBours = (node, theMap) => {
    const potentialNorth = posFromDirection(node, DIRECTIONS.NORTH);
    const potentialSouth = posFromDirection(node, DIRECTIONS.SOUTH);
    const potentialEast = posFromDirection(node, DIRECTIONS.EAST);
    const potentialWest = posFromDirection(node, DIRECTIONS.WEST);

    return [potentialNorth, potentialSouth, potentialEast, potentialWest].map(pos => theMap[getMapKey(pos)]).filter(n => n ? n.value !== '#' : false);
}

const aStar = (nodes, playerNode, finishNode, theMap) => {
    // Set g and h costs
    nodes.forEach(node => {
        node.h = getDistance(node, finishNode);
    })
    const visited = [];
    let priority = [{
        prev: undefined,
        node: playerNode,
        g: 0
    }];
    while (priority.length > 0) {
        const lookUp = priority.shift();

        if (lookUp.node.value === finishNode.value) {
            return {
                steps: getSteps(lookUp, playerNode),
                node: lookUp.node
            }
        }
        const neighbours = getNeighBours(lookUp.node, theMap);
        neighbours.forEach(n => {
            if (!n.isDoor || playerNode.keys.includes(n.value)) {
                const prio = priority.find(pn => pn === n);
                const g = getDistance(lookUp.node, n) + lookUp.g;
                if (prio) {
                    if (prio.g > g) {
                        prio.prev = lookUp;
                        prio.g = g;
                    }
                } else if (!visited.some(vn => vn.node === n)) {
                    priority.push({
                        node: n,
                        prev: lookUp,
                        g
                    })
                }
            }
        })
        priority.sort(sortByCost);
        visited.push(lookUp)
    }
    return null;
}
const getAvailableKeys = (nodes, playerNode, theMap) => {
    const nodeCopy = clonedeep(nodes);
    const potentialKeyNodes = nodeCopy.filter(node => node.isKey && !playerNode.keys.includes(node.value));
    return potentialKeyNodes.map(key => aStar(nodeCopy, playerNode, key, theMap)).filter(result => result ? true : false);
}

module.exports = {
    aStar,
    getAvailableKeys
}