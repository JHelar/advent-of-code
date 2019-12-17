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

const getPhase = (input, patterns) => {
    const inputNums = input.split('').map(n => n |0);
    let result = [];
    for(let iteration = 0; iteration < inputNums.length; iteration++) {
        const basePattern = patterns[iteration];
        
        result.push(inputNums.reduce((sum, num, index) => {
            return sum + (num * basePattern[index]);
        }, 0).toString())
    }

    return result.map(n => n.slice(n.length - 1)).join('');
}
const phaseCount = 100;
// Make all base patterns
const patterns = Array(input.length).fill(0).map((n, iteration) => getBasePattern(input.length, iteration));

for(let phase = 0; phase < phaseCount; phase++) {
    console.log({
        phase
    })
    input = getPhase(input, patterns)
}

console.log({
    result: input.slice(0, 8)
})