const STATE = {
    NEXT: 1,
    EXIT: 99
}

const makeOperation = operation => (intcode, start_pos, end_pos, result_pos) => intcode[result_pos] = operation(intcode[start_pos] | 0, intcode[end_pos] | 0).toString();

const addOperation = makeOperation((oneValue, anotherValue) => oneValue + anotherValue);
const multiplyOperation = makeOperation((oneValue, anotherValue) => oneValue * anotherValue);

const OP = {
    "1": addOperation,
    "2": multiplyOperation    
}

const readIntcode = (noun, verb, intcode) => {
    intcode[1] = noun.toString();
    intcode[2] = verb.toString();

    let current_state = STATE.NEXT;
    let step = 0;

    while(current_state !== STATE.EXIT) {
        const operation = OP[intcode[step]];
        if(operation) {
            operation(intcode, intcode[step + 1], intcode[step + 2], intcode[step + 3]);
            current_state = STATE.NEXT;
            step += 4;
        } else {
            current_state = STATE.EXIT;
        }
    }
    return intcode;
}

module.exports = readIntcode;