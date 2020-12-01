const getDistance = (onePoint, anotherPoint) => Math.sqrt(Math.pow(anotherPoint.x - onePoint.x, 2) + Math.pow(anotherPoint.y - onePoint.y, 2));
const sortByCost = (a, b) => (a.g + a.node.h) - (b.g + b.node.h);
const aStar = nodes => {
    const startNode = nodes.find(node => node.value === 'S');
    const finishNode = nodes.find(node => node.value === 'W');

    // Set g and h costs
    nodes.forEach(node => {
        node.h = getDistance(node, finishNode);
    })
    
    const visited = [];
    let priority = [{
        prev: undefined,
        node: startNode,
        g: 0
    }];

    
    while(true) {
        const lookUp = priority.splice(0, 1)[0];
        if(lookUp.node.value === 'W') {
            return lookUp
        }
        lookUp.node.neighbours.forEach(n => {
            const prio = priority.find(pn => pn === n);
            const g = getDistance(lookUp.node, n) + lookUp.g;
            if(prio) {
                if(prio.g > g) {
                    prio.prev = lookUp;
                    prio.g = g;
                }
            } else if(!visited.some(vn => vn.node === n)) {
                priority.push({
                    node: n,
                    prev: lookUp,
                    g
                })
            }

        })
        priority.sort(sortByCost);
        visited.push(lookUp)
    }
}

module.exports = aStar