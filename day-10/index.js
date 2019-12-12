const parseMap = require('./parse-map');
const getAstroidCount = require('./get-astroid-count');
const { rotate, machineGun } = require('./vaporize');
const mapString = require('../utils').getStringFromFile('./day-10/test.txt');

const { astroidMap } = parseMap(mapString);

const lazor = astroidMap.map(point => ({
    length: getAstroidCount(point, astroidMap).length,
    ...point
})).sort((a, b) => b.length - a.length)[0];

const result = machineGun(lazor, astroidMap);
console.log(result.vaporized.length)
// const c = contains({ x: 1, y: 0 }, { x: 3, y: 4 }, { x: 2, y: 2 });

// console.log({
//     c
// })

require('fs').writeFileSync('map.json', JSON.stringify(astroidMap))