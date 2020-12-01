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
const mainRoutine = 'A,A,B,C,C,A,B,C,A,B'.split('').map(char => char.charCodeAt(0));
mainRoutine.push('10');
const functionA = 'L,12,L,12,R,12'.split('').map(char => char.charCodeAt(0));
functionA.push('10');
const functionB = 'L,8,L,8,R,12,L,8,L,8'.split('').map(char => char.charCodeAt(0));
functionB.push('10');
const functionC = 'L,10,R,8,R,12'.split('').map(char => char.charCodeAt(0));
functionC.push('10');

const newInput = require('../utils').readArray('./day-17/input.txt');
newInput[0] = '2';
vacuumProgram = makeProgram(newInput)
console.log({
	mainRoutine,
	functionA,
	functionB,
	functionC
})
let test = vacuumProgram.run(...mainRoutine, ...functionA, ...functionB, ...functionC, 'n'.charCodeAt(0).toString(), '10');
while(vacuumProgram.state === 'ON') {
	let output = vacuumProgram.run();
	console.log({
		output
	})

}


const htmlDoc = htmlMap(Object.values(theMap));

require('fs').writeFileSync('./day-17/map.html', htmlDoc);