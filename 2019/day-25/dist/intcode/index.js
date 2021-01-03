"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.IntProgram = void 0;
const OP_CODES = {
    HALT: 99,
    ADD: 1,
    MUL: 2,
    INPUT: 3,
    OUTPUT: 4,
    JMP_TRUE: 5,
    JMP_FALSE: 6,
    LE: 7,
    EQ: 8,
    ADJ: 9
};
const OP_CODE_PARAMS = {
    [OP_CODES.ADD]: 3,
    [OP_CODES.MUL]: 3,
    [OP_CODES.INPUT]: 1,
    [OP_CODES.OUTPUT]: 1,
    [OP_CODES.HALT]: 0,
    [OP_CODES.JMP_TRUE]: 2,
    [OP_CODES.JMP_FALSE]: 2,
    [OP_CODES.LE]: 3,
    [OP_CODES.EQ]: 3,
    [OP_CODES.ADJ]: 1
};
class Operator {
    program;
    constructor(program) {
        this.program = program;
    }
    add() {
        const one = this.program.getValueForParam(0, this.program.cursor + 1n);
        const two = this.program.getValueForParam(1, this.program.cursor + 2n);
        const result = one + two;
        this.program.setValueForParam(2, result, this.program.cursor + 3n);
        this.program.advanceCursor();
    }
    mul() {
        const one = this.program.getValueForParam(0, this.program.cursor + 1n);
        const two = this.program.getValueForParam(1, this.program.cursor + 2n);
        const result = one * two;
        this.program.setValueForParam(2, result, this.program.cursor + 3n);
        this.program.advanceCursor();
    }
    input(value) {
        this.program.setValueForParam(0, value, this.program.cursor + 1n);
        this.program.advanceCursor();
    }
    output() {
        const value = this.program.getValueForParam(0, this.program.cursor + 1n);
        this.program.advanceCursor();
        return value;
    }
    jumpIfTrue() {
        const value = this.program.getValueForParam(0, this.program.cursor + 1n);
        if (value !== 0n) {
            this.program.cursor = this.program.getValueForParam(1, this.program.cursor + 2n);
        }
        else {
            this.program.advanceCursor();
        }
    }
    jumpIfFalse() {
        const value = this.program.getValueForParam(0, this.program.cursor + 1n);
        if (value === 0n) {
            this.program.cursor = this.program.getValueForParam(1, this.program.cursor + 2n);
        }
        else {
            this.program.advanceCursor();
        }
    }
    lesserThan() {
        const one = this.program.getValueForParam(0, this.program.cursor + 1n);
        const two = this.program.getValueForParam(1, this.program.cursor + 2n);
        if (one < two) {
            this.program.setValueForParam(2, 1n, this.program.cursor + 3n);
        }
        else {
            this.program.setValueForParam(2, 0n, this.program.cursor + 3n);
        }
        this.program.advanceCursor();
    }
    equals() {
        const one = this.program.getValueForParam(0, this.program.cursor + 1n);
        const two = this.program.getValueForParam(1, this.program.cursor + 2n);
        if (one === two) {
            this.program.setValueForParam(2, 1n, this.program.cursor + 3n);
        }
        else {
            this.program.setValueForParam(2, 0n, this.program.cursor + 3n);
        }
        this.program.advanceCursor();
    }
    adjust() {
        const value = this.program.getValueForParam(0, this.program.cursor + 1n);
        this.program.relativeBase += value;
        this.program.advanceCursor();
    }
}
class IntProgram {
    memory;
    originInput;
    cursor;
    relativeBase;
    running;
    opCode;
    paramModes;
    operator;
    constructor(input) {
        this.originInput = input;
        this.memory = {};
        this.reset();
        this.cursor = 0n;
        this.relativeBase = 0n;
        this.running = true;
        this.opCode = -1;
        this.paramModes = [];
        this.operator = new Operator(this);
    }
    getValueAt(address) {
        if (address < 0)
            throw new Error(`Tried to get value from invalid address: ${address}, opcode: ${this.opCode}, cursorAt: ${this.cursor}, paramModes: ${this.paramModes}`);
        if (!(address.toString() in this.memory)) {
            this.memory[address.toString()] = 0n;
            return 0n;
        }
        return this.memory[address.toString()];
    }
    getValueFromPointerAt(address) {
        const pointerAddress = this.getValueAt(address);
        return this.getValueAt(pointerAddress);
    }
    getValueRelativeFromPointerAt(address) {
        const relativeAddress = this.getValueAt(address);
        return this.getValueAt(this.relativeBase + relativeAddress);
    }
    setValueAt(value, address) {
        this.memory[address.toString()] = value;
    }
    setValueFromPointerAt(value, address) {
        const saveAtAddress = this.getValueAt(address);
        this.setValueAt(value, saveAtAddress);
    }
    setValueRelativeFromPointerAt(value, address) {
        const relativeAddress = this.getValueAt(address);
        this.setValueAt(value, this.relativeBase + relativeAddress);
    }
    getValueForParam(paramNo, address) {
        const mode = this.paramModes[paramNo] || 0;
        if (mode === 0) {
            return this.getValueFromPointerAt(address);
        }
        else if (mode === 2) {
            return this.getValueRelativeFromPointerAt(address);
        }
        return this.getValueAt(address);
    }
    setValueForParam(paramNo, value, address) {
        const mode = this.paramModes[paramNo] || 0;
        if (mode === 0) {
            return this.setValueFromPointerAt(value, address);
        }
        else if (mode === 2) {
            return this.setValueRelativeFromPointerAt(value, address);
        }
        return this.setValueAt(value, address);
    }
    advanceCursor() {
        this.cursor = this.cursor + 1n + BigInt(this.paramModes.length);
    }
    advanceProgram() {
        const code = this.getValueAt(this.cursor).toString();
        this.opCode = Number(code.substr(code.length - 2));
        this.paramModes = code.padStart(5, '0')
            .substr(0, 3)
            .split('')
            .map(Number)
            .reverse()
            .slice(0, OP_CODE_PARAMS[this.opCode]);
    }
    reset() {
        this.initializeMemory();
        this.cursor = 0n;
        this.relativeBase = 0n;
        this.running = true;
        this.opCode = -1;
        this.paramModes = [];
    }
    initializeMemory() {
        this.memory = {};
        this.originInput.split(',').map(i => BigInt(i)).forEach((i, index) => {
            this.memory[index.toString()] = i;
        });
    }
    exec(input, output = []) {
        this.advanceProgram();
        let result = Infinity;
        this.running = true;
        switch (this.opCode) {
            case OP_CODES.ADD:
                this.operator.add();
                break;
            case OP_CODES.MUL:
                this.operator.mul();
                break;
            case OP_CODES.JMP_TRUE:
                this.operator.jumpIfTrue();
                break;
            case OP_CODES.JMP_FALSE:
                this.operator.jumpIfFalse();
                break;
            case OP_CODES.LE:
                this.operator.lesserThan();
                break;
            case OP_CODES.EQ:
                this.operator.equals();
                break;
            case OP_CODES.ADJ:
                this.operator.adjust();
                break;
            case OP_CODES.INPUT:
                if (input.length) {
                    this.operator.input(input.shift());
                }
                else {
                    this.running = false;
                }
                break;
            case OP_CODES.OUTPUT:
                result = this.operator.output();
                output.push(result);
                break;
            case OP_CODES.HALT:
                this.running = false;
                break;
            default:
                this.running = false;
                break;
        }
        return result;
    }
}
exports.IntProgram = IntProgram;
