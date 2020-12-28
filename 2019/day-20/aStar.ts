import { Node } from './node'

type SearchNode = {
    prev?: SearchNode
    node: Node
    g: number
}

const getDistance = (onePoint: Node, anotherPoint: Node) => Math.sqrt(Math.pow(anotherPoint.x - onePoint.x, 2) + Math.pow(anotherPoint.y - onePoint.y, 2));
const sortByCost = (a: SearchNode, b: SearchNode) => (a.g + a.node.h) - (b.g + b.node.h);
export const aStar = (nodes: Node[]) => {
    const startNode = nodes.find(node => node.name === 'AA')!;
    const finishNode = nodes.find(node => node.name === 'ZZ')!;

    // Set g and h costs
    nodes.forEach(node => {
        node.h = getDistance(node, finishNode);
    })
    
    const visited: SearchNode[] = [];
    let priority: SearchNode[] = [{
        prev: undefined,
        node: startNode,
        g: 0
    }];

    
    while(true) {
        const lookUp = priority.shift();
        if(!lookUp) {
            return null
        }

        if(lookUp.node.name === finishNode.name) {
            return lookUp
        }
        lookUp.node.neighbours.forEach(n => {
            const prio = priority.find(pn => pn.node.getKey() === n.getKey());
            const g = getDistance(lookUp.node, n) + lookUp.g;
            if(prio) {
                if(prio.g > g) {
                    prio.prev = lookUp;
                    prio.g = g;
                }
            } else if(!visited.some(vn => vn.node.getKey() === n.getKey())) {
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
