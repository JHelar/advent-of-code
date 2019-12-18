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
		const results = inputNums.map((num) => {
			const base = basePattern[basePointer];
			if(baseIterationCount >= iteration) {
				basePointer = (basePointer + 1) % basePattern.length;
				baseIterationCount = 0;
			} else {
				baseIterationCount++;
			}
            return (num * base);
		})
		Array(repeat).fill(0).reduce(())
        result.push(Math.abs() % 10)
		console.log({
			iteration
		})
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
input = Array(10000).fill(input).join('');
let inputNums = input.split('');
const phaseCount = 100;

console.log({
	inputLength: input.length,
})
// Make all base patterns
for(let phase = 0; phase < phaseCount; phase++) {
	inputNums = getPhase(inputNums);
}

console.log({
    result: inputNums.slice(offset, offset + 8)
})