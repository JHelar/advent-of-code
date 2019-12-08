const readIntcode = require('./read-intcode');

const intcode = require('../utils').readArray('./day-2/input.txt');

const FIND_SUM = 19690720;
let noun = 0;
let verb = 0;

while(true) {
    const output = readIntcode(noun, verb, [...intcode])[0] | 0;
    if(output === FIND_SUM) {
        console.log(`Found it: noun:${noun}, verb:${verb}, sum:${100 * noun + verb}!`);
        break;
    };

    if(verb > 99) {
        console.log('Could not find it!');
        break;
    };

    noun = (noun + 1) % 100;
    if(noun === 0) {
        verb++;
    }
};