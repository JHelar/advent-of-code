const { Readable, Writable } = require('stream');
const makeReadProgram = require('./read-intcode');
const getPermutations = require('./get-permutations');

const intcode = require('../utils').readArray('./day-7/input.txt');

const getProgram = (phaseSetting) => {
    const outputStream = new Readable();
    outputStream._read = () => { };

    const inputStream = new Readable();
    inputStream._read = () => { }; // redundant? see update below

    const runProgram = makeReadProgram(phaseSetting, inputStream, outputStream)

    return {
        phaseSetting,
        outputStream,
        inputStream,
        run: () => runProgram([...intcode])
    }
}

const getProgramOutput = phaseSettings => {
    const programs = phaseSettings.map(getProgram);
    const runners = () => Promise.all(programs.map(program => program.run()))
    let programPointer = 0;
    const handleData = data => {
        // Pass to next program
        // console.log(`Got ${data} from ${programs[programPointer].phaseSetting}`)
        programPointer = (programPointer + 1) % programs.length;
        const program = programs[programPointer];
        const inputStream = program.inputStream;
        // console.log(`Sending a ${data} to ${program.phaseSetting}`);
        currentOutput = data;
        if (!inputStream.destroyed) {
            inputStream.push(`${data}\n`)
        }
    }
    
    programs.forEach(({ outputStream }) => outputStream.on('data', handleData));
    
    // Start the program chain
    // console.log(`Sending a ${0} to ${programs[0].phaseSetting}`);
    programs[0].inputStream.push('0\n')
    
    return runners()
        .then(outputs => Math.max(...outputs))
}

Promise.all(getPermutations([5, 6, 7, 8, 9])
                        .map(getProgramOutput))
                        .then(outputs => Math.max(...outputs))
                        .then(bestOutput => console.log({ bestOutput }))