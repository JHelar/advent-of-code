const readline = require('readline');

const STATE = {
    NEXT: 1,
    EXIT: 99
}

const PARAMETER_MODE = {
    POSITION: "0",
    IMMEDIATE: "1"
}

const getParameterAddress = (intcode, pointer, mode) => mode === PARAMETER_MODE.POSITION ? intcode[pointer] : pointer;
const getParameterAddresses = (intcode, pointer, modes) => modes.map((mode, index) => getParameterAddress(intcode, pointer + index + 1, mode));
const makeOperation = operation => async (intcode, pointer, modes) => await operation(intcode, pointer, getParameterAddresses(intcode, pointer, modes))

let isInitialized = false;

const addOperation = makeOperation((intcode, pointer, params) => {
    const [paramAddress1, paramAddress2, outputAddress] = params;

    const param1 = intcode[paramAddress1] | 0;
    const param2 = intcode[paramAddress2] | 0;
    const output = param1 + param2;

    intcode[outputAddress] = output;

    return Promise.resolve({
        pointer: pointer + 4
    });
});
const multiplyOperation = makeOperation((intcode, pointer, params) => {
    const [paramAddress1, paramAddress2, outputAddress] = params;

    const param1 = intcode[paramAddress1] | 0;
    const param2 = intcode[paramAddress2] | 0;
    const output = param1 * param2;

    intcode[outputAddress] = output;

    return Promise.resolve({
        pointer: pointer + 4
    });
});
const inputOperation = makeOperation((intcode, pointer, params) => {
    const [inputAddress] = params;
    if(isInitialized) {
        const rl = readline.createInterface({
            input: process.stdin,
            output: process.stdout,
        });
    
        return new Promise(resolve => rl.question('Input: ', ans => {
            rl.close();
            intcode[inputAddress] = ans;
            resolve({
                pointer: pointer + 2,
            });
        }))
    } else {
        intcode[inputAddress] = process.argv[2] | 0;
        isInitialized = true;
        return Promise.resolve({
            pointer: pointer + 2,
        })
    }
});
const outputOperation = makeOperation((intcode, pointer, params) => {
    const [outputAddress] = params;
    const outputValue = intcode[outputAddress];

    process.stdout.write(outputValue)

    return Promise.resolve({
        pointer: pointer + 2,
        output: outputValue
    });
});
const jumpIfTrueOperation = makeOperation((intcode, pointer, params) => {
    const [ paramAddress1, paramAddress2 ] = params;
    const checkValue = intcode[paramAddress1] | 0;
    let gotoValue = pointer + 3;

    if(checkValue !== 0) {
        gotoValue = intcode[paramAddress2] | 0;
    }

    return Promise.resolve({
        pointer: gotoValue
    })
});

const jumpIfFalseOperation = makeOperation((intcode, pointer, params) => {
    const [ paramAddress1, paramAddress2 ] = params;
    const checkValue = intcode[paramAddress1] | 0;
    let gotoValue = pointer + 3;

    if(checkValue === 0) {
        gotoValue = intcode[paramAddress2] | 0;
    }

    return Promise.resolve({
        pointer: gotoValue
    })
});

const lessThanOperation = makeOperation((intcode, pointer, params) => {
    const [ paramAddress1, paramAddress2, outputAddress ] = params;
    const oneValue = intcode[paramAddress1] | 0;
    const anotherValue = intcode[paramAddress2] | 0;
    const outputValue = oneValue < anotherValue ? 1 : 0;

    intcode[outputAddress] = outputValue;

    return Promise.resolve({
        pointer: pointer + 4
    })
});

const equalsOperation = makeOperation((intcode, pointer, params) => {
    const [ paramAddress1, paramAddress2, outputAddress ] = params;
    const oneValue = intcode[paramAddress1] | 0;
    const anotherValue = intcode[paramAddress2] | 0;
    const outputValue = oneValue == anotherValue ? 1 : 0;

    intcode[outputAddress] = outputValue;

    return Promise.resolve({
        pointer: pointer + 4
    })
});

const OP = {
    "01": addOperation,
    "02": multiplyOperation,
    "03": inputOperation,
    "04": outputOperation,
    "05": jumpIfTrueOperation,
    "06": jumpIfFalseOperation,
    "07": lessThanOperation,
    "08": equalsOperation
}

const getInstructions = instructionString => {
    instructionString = instructionString.toString().padStart(4, '0');
    const opcode = instructionString.slice(instructionString.length - 2);

    const operation = OP[opcode] || false;

    const modes = instructionString.slice(0, instructionString.length - 2).padStart(3, '0').split('').reverse();

    return {
        operation,
        modes,
        opcode: opcode | 0
    }
}

const readIntcode = async (intcode) => {
    let current_state = STATE.NEXT;
    let pointer = 0;
    let output = 0;

    while (current_state !== STATE.EXIT) {
        const { operation, modes, opcode } = getInstructions(intcode[pointer]);
        if (operation) {
            const result = await operation(intcode, pointer, modes);
            if (result.output !== undefined) {
                output = result.output;
            }
            pointer = result.pointer;
        } else {
            current_state = STATE.EXIT;
        }
    }
    return output;
}

module.exports = readIntcode;