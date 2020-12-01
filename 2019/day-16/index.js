let input = require('../utils').getStringFromFile('./day-16/input.txt');

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
            return sum + (num * base);
		}, 0)) % 10)
		baseIterationCount = 1;
		basePointer = 0;
    }

    return result;
}

const getPhaseP2 = origin => {
	const inputNums = [...origin];
	inputNums[inputNums.length - 1] = origin[origin.length - 1];
	for(let positionConsidered = origin.length - 2; positionConsidered >= 0; positionConsidered--) {
		inputNums[positionConsidered] = (origin[positionConsidered] + inputNums[positionConsidered + 1]) % 10;
	}
	return inputNums;
}
// Get message offset
const offset = input.slice(0, 7) | 0;
const phaseCount = 100;
// Repeat input X
let inputNums = input.split('').map(n => n |0);
inputNums = Array(10000).fill(0).reduce((acc) => acc.concat(inputNums),[]);

const fromEnd = inputNums.length - offset;
const toDo = fromEnd * 2;

inputNums = inputNums.splice(inputNums.length - toDo, toDo);

console.log({
	inputLength: input.length,
})
// Make all base patterns
for(let phase = 0; phase < phaseCount; phase++) {
	inputNums = getPhaseP2(inputNums);
}

console.log({
    result: inputNums.slice(fromEnd, fromEnd + 8).join('')
})