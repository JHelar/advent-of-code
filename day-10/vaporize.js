const { contains, getLength } = require('./line');
const isNotPoint = myPoint => point => point.name !== myPoint.name
const maybeCheat = (lazor, astroidMap) => astroidMap.map(target => ({ distance: getLength(lazor, target), ...target })).sort((a, b) => a.distance - b.distance)
const getTargets = (lazor, toPoint, astroidMap) => astroidMap.filter(point => contains(lazor, toPoint, point)).map(target => ({ distance: getLength(lazor, target), ...target }));
const getClosestTarget = (targets) => targets.sort((a, b) => a.distance - b.distance)[0]
const rotate = (lazor, astroidMap, mapWidth, mapHeight) => {
    let targetX = lazor.x;
    let targetY = 0;

    let dirX = 1;
    let dirY = 0;

    let lookUpMap = [...astroidMap];
    let steps = mapWidth * mapHeight;

    let vaporized = [];

    for(let step = 0; step < steps; step++) {
        const target = { x: targetX, y: targetY }
        const targets = getTargets(lazor, target, lookUpMap);
       
        if(targets.length) {
            const vaporize = getClosestTarget(targets);
            vaporized.push(vaporize);

            lookUpMap = lookUpMap.filter(isNotPoint(vaporize));
        }
        targetX = targetX + (dirX * 1);
        targetY = targetY + (dirY * 1);

        if(targetX === mapWidth && targetY === 0) {
            dirX = 0;
            dirY = 1;
        } else if(targetX === mapWidth && targetY === mapHeight) {
            dirX = -1;
            dirY = 0;
        } else if(targetX === 0 && targetY === mapHeight) {
            dirX = 0;
            dirY = -1;
        } else if(targetX === 0 && targetY === 0) {
            dirX = 1;
            dirY = 0;
        }
    }

    return {
        astroidMap: lookUpMap,
        vaporized
    }
}

const machineGun = (lazor, astroidMap, mapWidth, mapHeight) => {
    let lookUpMap = [...astroidMap];
    let rotations = 0;
    let vaporized = [];
    let shoot = true;

    while(shoot){
        const rotationResult = rotate(lazor, lookUpMap, mapWidth, mapHeight);
        if(rotationResult.vaporized.length) {
            vaporized = vaporized.concat(rotationResult.vaporized);
            lookUpMap = rotationResult.astroidMap;
            rotations++;
        } else {
            break;
        }
    }

    return {
        rotations,
        vaporized
    }
}

module.exports = {
    rotate,
    machineGun,
    maybeCheat
}