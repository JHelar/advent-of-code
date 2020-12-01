const { DIRECTIONS, posFromDirection } = require('./direction')
const getMapKey = ({ x, y }) => `(${x},${y})`;
const flood = theMap => {
    const nodes = Object.values(theMap);
    const startNode = nodes.find(n => n.value === 'W');

    // const walkLength = nodes.filter(n => n.value !== '#').length;
    const visited = [];
    const queue = [{
        node: startNode,
        steps: 0
    }];
    let lookUp;
    while (queue.length > 0) {
        lookUp = queue.splice(0, 1)[0];

        const potentialNorth = posFromDirection(lookUp.node, DIRECTIONS.NORTH);
        const potentialSouth = posFromDirection(lookUp.node, DIRECTIONS.SOUTH);
        const potentialEast = posFromDirection(lookUp.node, DIRECTIONS.EAST);
        const potentialWest = posFromDirection(lookUp.node, DIRECTIONS.WEST);

        [potentialNorth, potentialSouth, potentialEast, potentialWest]
            .map(p => theMap[getMapKey(p)])
            .forEach(node => {
                if(node && node.value !== '#' && !visited.includes(node)) {
                    Object.assign(node, {
                        color: 'lightblue',
                        steps: lookUp.steps + 1
                    })
                    queue.push({
                        node,
                        steps: lookUp.steps + 1
                    });
                }
            });
        visited.push(lookUp.node);
    }
    const maxSteps = Math.max(...nodes.map(n => n.steps || 0));
    return maxSteps
}

module.exports = flood;