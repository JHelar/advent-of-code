let input = require('../utils').getStringFromFile('./day-16/test.txt');

const getBasePattern = (inputLength, iteration) => {
    const basePattern = [0, 1, 0, -1];
    const inputTimeBigger = Math.ceil((inputLength + 1) / basePattern.length);
    return Array(inputTimeBigger).fill(0)
        .reduce((pattern) => pattern.concat(basePattern), [])
        .reduce((pattern, num) => {
            return pattern.concat(Array(iteration + 1).fill(num))
        }, []).splice(1, inputLength + 1)
}

const getPhase = (inputNums, repeat) => {
	let result = [];
	let basePointer = 1;
	let baseIterationCount = 0;

	const basePattern = [0, 1, 0, -1];

    for(let iteration = 0; iteration < inputNums.length; iteration++) {
		result.push(Math.abs(inputNums.reduce((sum, num) => {
			const base = basePattern[basePointer];
			if(baseIterationCount >= iteration) {
				basePointer = (basePointer + 1) % basePattern.length;
				baseIterationCount = 0;
			} else {
				baseIterationCount++;
			}
            return sum + ((num * base) * repeat);
		}, 0)) % 10)
		baseIterationCount = 1;
		basePointer = 0;
    }

    return result;
}
// Get message offset
const offset = input.slice(0, 7) | 0;
console.log({
	offset
})
// Repeat input X
let inputNums = input.split('');
const phaseCount = 100;

console.log({
	inputLength: input.length,
})
// Make all base patterns
for(let phase = 0; phase < phaseCount; phase++) {
	inputNums = getPhase(inputNums, 10000);
}

console.log({
    result: inputNums.slice(offset, offset + 8)
})