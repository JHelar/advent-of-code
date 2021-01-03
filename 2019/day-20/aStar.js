"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.aStarWithLevels = exports.aStar = void 0;
const node_1 = require("./node");
const getDistance = (onePoint, anotherPoint) => Math.sqrt(Math.pow(anotherPoint.x - onePoint.x, 2) + Math.pow(anotherPoint.y - onePoint.y, 2) + Math.pow(anotherPoint.level - onePoint.level, 2));
const sortByCost = (a, b) => (a.g + a.node.h) - (b.g + b.node.h);
const aStar = (nodes) => {
    const startNode = nodes.find(node => node.name === 'AA');
    const finishNode = nodes.find(node => node.name === 'ZZ');
    // Set g and h costs
    nodes.forEach(node => {
        node.h = getDistance(node, finishNode);
    });
    const visited = [];
    let priority = [{
            prev: undefined,
            node: startNode,
            g: 0,
            distance: 0
        }];
    while (true) {
        const lookUp = priority.shift();
        if (!lookUp) {
            return null;
        }
        if (lookUp.node.name === finishNode.name) {
            return lookUp;
        }
        lookUp.node.neighbours.forEach(n => {
            const prio = priority.find(pn => pn.node.getKey() === n.getKey());
            const g = getDistance(lookUp.node, n) + lookUp.g;
            if (prio) {
                if (prio.g > g) {
                    prio.prev = lookUp;
                    prio.g = g;
                }
            }
            else if (!visited.some(vn => vn.node.getKey() === n.getKey())) {
                priority.push({
                    node: n,
                    prev: lookUp,
                    g,
                    distance: lookUp.distance + 1
                });
            }
        });
        priority.sort(sortByCost);
        visited.push(lookUp);
    }
};
exports.aStar = aStar;
const isInsidePortal = ({ x, y }, mapWidth, mapHeight) => y > 3 && y < mapHeight - 3 &&
    x > 3 && x < mapWidth - 3;
const isPortalOpen = (currentLevel, node) => {
    if (node instanceof node_1.Portal) {
        if (node.name === 'AA' || node.name === 'ZZ') {
            return currentLevel === 0;
        }
        return true;
    }
    return true;
};
const aStarWithLevels = (nodes, mapWidth, mapHeight) => {
    const startNode = nodes.find(node => node.name === 'AA');
    const finishNode = nodes.find(node => node.name === 'ZZ');
    const visited = [];
    let toVisit = [];
    let travel = [{
            prev: undefined,
            node: startNode,
            g: 0,
            distance: 0,
            level: 0
        }];
    while (toVisit.length) {
        const lookUp = toVisit.shift();
        if (!lookUp) {
            return null;
        }
        visited.push(lookUp);
        if (lookUp.node.name === finishNode.name && lookUp.level === 0) {
            console.log(lookUp.distance);
            return lookUp;
        }
        for (const n of lookUp.node.neighbours) {
            if (n.name === startNode.name)
                continue;
            if (n.name === finishNode.name && lookUp.level !== 0)
                continue;
            const nextLevel = (lookUp.node instanceof node_1.Portal ? isInsidePortal(lookUp.node, mapWidth, mapHeight) ? lookUp.level + 1 : lookUp.level - 1 : lookUp.level);
            if (nextLevel < 0)
                continue;
            if (visited.some(vn => vn.node.getKey() === n.getKey()))
                continue;
            if (!toVisit.some(vn => vn.node.getKey() === n.getKey())) {
                toVisit.push({
                    distance: lookUp.distance + 1,
                    g: 0,
                    level: nextLevel,
                    node: n,
                    prev: lookUp
                });
            }
        }
        // if(toVisit.length) {
        //     let curMin: SearchNodeWithLevel | undefined = undefined
        //     for (const v of toVisit) {
        //         if(curMin === undefined || v.distance < curMin!.distance) {
        //             curMin = v
        //         }
        //     }
        //     curMin && travel.push(curMin)
        //     toVisit = toVisit.filter(t => t.node.getKey() !== curMin!.node.getKey())
        // }
        // lookUp.node.neighbours.forEach(n => { 
        //     if(!visited.some(vn => vn.node.getKey() === n.getKey())) {
        //         priority.push({
        //             node: n,
        //             prev: lookUp,
        //             level,
        //             g: 0,
        //             distance: lookUp.distance + 1
        //         })
        //     }
        // })
    }
};
exports.aStarWithLevels = aStarWithLevels;
