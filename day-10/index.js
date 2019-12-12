const parseMap = require('./parse-map');
const getAstroidCount = require('./get-astroid-count');
const vaporize = require('./vaporize');
const mapString = require('../utils').getStringFromFile('./day-10/input.txt');

const { astroidMap } = parseMap(mapString);

const lazor = astroidMap.map(point => ({
    length: getAstroidCount(point, astroidMap).length,
    ...point
})).sort((a, b) => b.length - a.length)[0];

const angleMap = vaporize(lazor, astroidMap);
const killOrder = [];
for(let lazorPointer = 0; lazorPointer < angleMap.length; lazorPointer++) {
    const astroid = angleMap[lazorPointer].splice(0, 1);
    if(astroid.length) {
        killOrder.push(astroid[0]);
    }
}
const twoHundrethKill = killOrder[199];
const killCode = twoHundrethKill.x * 100 + twoHundrethKill.y;
console.log({
    lazor,
    killed: killOrder[199],
    killCode
})
// console.log(result)
// const c = contains({ x: 1, y: 0 }, { x: 3, y: 4 }, { x: 2, y: 2 });

// console.log({
//     c
// })

require('fs').writeFileSync('map.json', JSON.stringify(killOrder))