const makeProgram = require('./read-incode');
const getMap = require('./get-map');
const htmlMap = require('./get-html-map');
const navigate = require('./navigate-map');

const input = require('../utils').readArray('./day-17/input.txt');
let vacuumProgram = makeProgram(input);

const theMap = getMap(vacuumProgram);
const movements = navigate(theMap);

console.log({
    movements
})
const mainRoutine = 'A,A,B,C,C,A,B,C,A,B';
const functionA = 'L,12,L,12,R,12';
const functionB = 'L,8,L,8,R,12,L,8,L,8';
const functionC = 'L,10,R,8,R,12';

input[0] = 2;
vacuumProgram = makeProgram(input)

const htmlDoc = htmlMap(Object.values(theMap));

require('fs').writeFileSync('./day-17/map.html', htmlDoc);