const readline = require('readline');

const STATE = {
    NEXT: 1,
    EXIT: 99
}

const PARAMETER_MODE = {
    POSITION: "0",
    IMMEDIATE: "1",
    RELATIVE: "2"
}
let debugParam = false;
let relativeBase = 0;
const getParameterAddress = (intcode, pointer, mode) => {
    switch(mode) {
        case PARAMETER_MODE.POSITION:
            return intcode[pointer];
        case PARAMETER_MODE.IMMEDIATE:
            return pointer;
        case PARAMETER_MODE.RELATIVE:
            return sanitizeParam(intcode[pointer]) + relativeBase;
    }
};
const getParameterAddresses = (intcode, pointer, modes) => modes.map((mode, index) => getParameterAddress(intcode, pointer + index + 1, mode));
const makeOperation = operation => async (intcode, pointer, modes) => await operation(intcode, pointer, getParameterAddresses(intcode, pointer, modes))

const sanitizeParam = param => typeof param === 'bigint' ? param : param | 0

const addOperation = makeOperation((intcode, pointer, params) => {
    const [paramAddress1, paramAddress2, outputAddress] = params;

    const param1 = sanitizeParam(intcode[paramAddress1]);
    const param2 = sanitizeParam(intcode[paramAddress2]);

    let output = param1 + param2;
    // If output is 16 digits redo as bigint
    if(typeof output !== 'bigint' && output.toString().length >= 16) {
        output = BigInt(param1) * BigInt(param2);
    }

    intcode[outputAddress] = output;

    return Promise.resolve({
        pointer: pointer + 4
    });
});
const multiplyOperation = makeOperation((intcode, pointer, params) => {
    const [paramAddress1, paramAddress2, outputAddress] = params;

    const param1 = sanitizeParam(intcode[paramAddress1]);
    const param2 = sanitizeParam(intcode[paramAddress2]);

    let output = param1 * param2;
    // If output is 16 digits redo as bigint
    if(typeof output !== 'bigint' && output.toString().length >= 16) {
        output = BigInt(param1) * BigInt(param2);
    }
    intcode[outputAddress] = output;

    return Promise.resolve({
        pointer: pointer + 4
    });
});
const inputOperation = makeOperation((intcode, pointer, params) => {
    const [inputAddress] = params;
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
});
const outputOperation = makeOperation((intcode, pointer, params) => {
    const [outputAddress] = params;
    const output = intcode[outputAddress];
    process.stdout.write(output + '\n')
    return Promise.resolve({
        pointer: pointer + 2,
        output
    });
});
const jumpIfTrueOperation = makeOperation((intcode, pointer, params) => {
    const [ paramAddress1, paramAddress2 ] = params;
    const checkValue = sanitizeParam(intcode[paramAddress1]);
    let gotoValue = pointer + 3;

    if(checkValue !== 0) {
        gotoValue = sanitizeParam(intcode[paramAddress2]);
    }

    return Promise.resolve({
        pointer: gotoValue
    })
});

const jumpIfFalseOperation = makeOperation((intcode, pointer, params) => {
    const [ paramAddress1, paramAddress2 ] = params;
    const checkValue = sanitizeParam(intcode[paramAddress1]);
    let gotoValue = pointer + 3;

    if(checkValue === 0) {
        gotoValue = sanitizeParam(intcode[paramAddress2]);
    }

    return Promise.resolve({
        pointer: gotoValue
    })
});

const lessThanOperation = makeOperation((intcode, pointer, params) => {
    const [ paramAddress1, paramAddress2, outputAddress ] = params;
    const oneValue = sanitizeParam(intcode[paramAddress1]);
    const anotherValue = sanitizeParam(intcode[paramAddress2]);
    const outputValue = oneValue < anotherValue ? 1 : 0;

    intcode[outputAddress] = outputValue;

    return Promise.resolve({
        pointer: pointer + 4
    })
});

const equalsOperation = makeOperation((intcode, pointer, params) => {
    const [ paramAddress1, paramAddress2, outputAddress ] = params;
    const oneValue = sanitizeParam(intcode[paramAddress1]);
    const anotherValue = sanitizeParam(intcode[paramAddress2]);
    const outputValue = oneValue == anotherValue ? 1 : 0;

    intcode[outputAddress] = outputValue;

    return Promise.resolve({
        pointer: pointer + 4
    })
});

const adjustRelativeBaseOperation = makeOperation((intcode, pointer, params) => {
    const [ paramAddress1 ] = params;
    const adjustValue = sanitizeParam(intcode[paramAddress1]);
    relativeBase = relativeBase + adjustValue;

    return Promise.resolve({
        pointer: pointer + 2
    })
})

const OP = {
    "01": addOperation,
    "02": multiplyOperation,
    "03": inputOperation,
    "04": outputOperation,
    "05": jumpIfTrueOperation,
    "06": jumpIfFalseOperation,
    "07": lessThanOperation,
    "08": equalsOperation,
    "09": adjustRelativeBaseOperation
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
    let output = 0;
    let current_state = STATE.NEXT;
    let pointer = 0;

    while (current_state !== STATE.EXIT) {
        const { operation, modes, opcode } = getInstructions(intcode[pointer]);
        
        if (operation) {
            const result = await operation(intcode, pointer, modes);
            if (result.output !== undefined) {
                output = result.output
            }
            pointer = result.pointer;
        } else {
            current_state = STATE.EXIT;
        }
    }
    return output;
}

module.exports = readIntcode;