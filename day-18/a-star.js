const NodeCache = require('node-cache');

const { getMapKey } = require('./get-map');
const { DIRECTIONS, posFromDirection } = require('./direction');

const resultCache = new NodeCache({
	stdTTL: 0,
	checkperiod: 0
})

const getDistance = (onePoint, anotherPoint) => Math.sqrt(Math.pow(anotherPoint.x - onePoint.x, 2) + Math.pow(anotherPoint.y - onePoint.y, 2));
const sortByCost = finishNode => (a, b) => (a.g + getDistance(a.node, finishNode)) - (b.g + getDistance(b.node, finishNode));
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

const getPotentialResult = (playerNode, finishNode) => {
	const startKey = getMapKey(playerNode);
	const finishKey = getMapKey(finishNode);
	const keys = playerNode.keys.reduce((keySum, k) => keySum + k, 0)
	let result = resultCache.get(startKey+finishKey+keys);
	if(!result) {
		result = resultCache.get(finishKey+startKey+keys);
	}
	return result;
}

const setPotentialResult = (playerNode, finishNode, value) => {
	const startKey = getMapKey(playerNode);
	const finishKey = getMapKey(finishNode);
	const keys = playerNode.keys.reduce((keySum, k) => keySum + k.value, 0)
	
	let result = resultCache.set(startKey+finishKey+keys, value);
	return result;
}

const aStar = (playerNode, finishNode, theMap) => {
	const result = getPotentialResult(playerNode, finishNode);
	if(result) {
		return result;
	}
    // Set g and h cost
    const visited = [];
    let priority = [{
        prev: undefined,
        node: playerNode,
        g: 0
	}];
	
    while (priority.length > 0) {
        const lookUp = priority.shift();

        if (lookUp.node.value === finishNode.value) {
            const searchResult = {
                steps: getSteps(lookUp, playerNode),
                node: lookUp.node
			};
			setPotentialResult(playerNode, finishNode, searchResult);
			return searchResult;
        }
        const neighbours = getNeighBours(lookUp.node, theMap);
        neighbours.forEach(n => {
            if (
				(!n.isKey || (n.value === finishNode.value || playerNode.keys.includes(n.value))) && // If key let it be this key or an allready taken key
				(!n.isDoor || playerNode.keys.includes(n.value))) // If door make sure we have the key
			{
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
        priority.sort(sortByCost(finishNode));
        visited.push(lookUp)
    }
    return null;
}
const getAvailableKeys = (nodes, playerNode, theMap) => {
    const potentialKeyNodes = nodes.reduce((keys, node) => {
		if(node.isKey && !playerNode.keys.includes(node.value)) {
			const result = aStar(playerNode, node, theMap);
			if(result) {
				keys.push(result);
			}
		}
		return keys;
	}, []);
    return potentialKeyNodes;
}

module.exports = {
    aStar,
    getAvailableKeys
}