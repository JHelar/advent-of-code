const { contains } = require('./line');

const inLineOfSight = (fromPoint, toPoint, map) => {
    const others = map.filter(isNotPoint(toPoint));
    return !others.some(point => contains(fromPoint, toPoint, point));
}

const isNotPoint = myPoint => point => point.name !== myPoint.name

const getAstroidCount = (point, map) => {
    let originMap = [...map].filter(isNotPoint(point));

    let lookUpMap = [...originMap];
    const canSee = [];
    while(lookUpMap.length) {
        let lookUp = lookUpMap.splice(0, 1)[0];
        if(inLineOfSight(point, lookUp, originMap)) {
            canSee.push(lookUp);
        }
    }
    // console.log({
    //     point,
    //     canSee
    // })
    return canSee;
}

module.exports = getAstroidCount;