import { runInNewContext } from "vm"

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
}

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
}

class Operator {
    program: IntProgram

    constructor(program: IntProgram) {
        this.program = program
    }

    add() {
        const one = this.program.getValueForParam(0, this.program.cursor + 1n)
        const two = this.program.getValueForParam(1, this.program.cursor + 2n)

        const result = one + two
        this.program.setValueForParam(2, result, this.program.cursor + 3n)
        this.program.advanceCursor()
    }

    mul() {
        const one = this.program.getValueForParam(0, this.program.cursor + 1n)
        const two = this.program.getValueForParam(1, this.program.cursor + 2n)

        const result = one * two
        this.program.setValueForParam(2, result, this.program.cursor + 3n)
        this.program.advanceCursor()
    }

    input(value: bigint) {
        this.program.setValueForParam(0, value, this.program.cursor + 1n)
        this.program.advanceCursor()
    }

    output() {
        const value = this.program.getValueForParam(0, this.program.cursor + 1n)
        this.program.advanceCursor()

        return value
    }

    jumpIfTrue() {
        const value = this.program.getValueForParam(0, this.program.cursor + 1n)
        if(value !== 0n) {
            this.program.cursor = this.program.getValueForParam(1, this.program.cursor + 2n)
        } else {
            this.program.advanceCursor()
        }
    }

    jumpIfFalse() {
        const value = this.program.getValueForParam(0, this.program.cursor + 1n)
        if(value === 0n) {
            this.program.cursor = this.program.getValueForParam(1, this.program.cursor + 2n)
        } else {
            this.program.advanceCursor()
        }
    }

    lesserThan() {
        const one = this.program.getValueForParam(0, this.program.cursor + 1n)
        const two = this.program.getValueForParam(1, this.program.cursor + 2n)

        if(one < two) {
            this.program.setValueForParam(2, 1n, this.program.cursor + 3n)
        } else {
            this.program.setValueForParam(2, 0n, this.program.cursor + 3n)
        }
        this.program.advanceCursor()
    }

    equals() {
        const one = this.program.getValueForParam(0, this.program.cursor + 1n)
        const two = this.program.getValueForParam(1, this.program.cursor + 2n)

        if(one === two) {
            this.program.setValueForParam(2, 1n, this.program.cursor + 3n)
        } else {
            this.program.setValueForParam(2, 0n, this.program.cursor + 3n)
        }
        this.program.advanceCursor()
    }

    adjust() {
        const value = this.program.getValueForParam(0, this.program.cursor + 1n)
        this.program.relativeBase += value
        this.program.advanceCursor()
    }
}

export class IntProgram {
    memory: Map<bigint, bigint>
    originInput: string
    cursor: bigint
    relativeBase: bigint
    running: boolean
    opCode: number
    paramModes: number[]
    operator: Operator

    constructor(input: string) {
        this.originInput = input
        this.memory = new Map()
        this.reset()
        this.cursor = 0n
        this.relativeBase = 0n
        this.running = true
        this.opCode = -1
        this.paramModes = []
        this.operator = new Operator(this)
    }

    getValueAt(address: bigint){
        if(address < 0) throw new Error(`Tried to get value from invalid address: ${address}, opcode: ${this.opCode}, cursorAt: ${this.cursor}, paramModes: ${this.paramModes}`)
        if(!this.memory.has(address)) {
            this.memory.set(address, 0n)
            return 0n
        }
        return this.memory.get(address)!
    }

    getValueFromPointerAt(address: bigint) {
        const pointerAddress = this.getValueAt(address)
        return this.getValueAt(pointerAddress)
    }

    getValueRelativeFromPointerAt(address: bigint) {
        const relativeAddress = this.getValueAt(address)
        return this.getValueAt(this.relativeBase + relativeAddress)
    }

    setValueAt(value: bigint, address: bigint) {
        this.memory.set(address, value)
    }

    setValueFromPointerAt(value: bigint, address: bigint) {
        const saveAtAddress = this.getValueAt(address)
        this.setValueAt(value, saveAtAddress)
    }

    setValueRelativeFromPointerAt(value: bigint, address: bigint) {
        const relativeAddress = this.getValueAt(address)
        this.setValueAt(value, this.relativeBase + relativeAddress)
    }

    getValueForParam(paramNo: number, address: bigint){
        const mode = this.paramModes[paramNo] || 0
        if(mode === 0) {
            return this.getValueFromPointerAt(address)
        } else if(mode === 2) {
            return this.getValueRelativeFromPointerAt(address)
        }
        return this.getValueAt(address)
    }

    setValueForParam(paramNo: number, value: bigint, address: bigint) {
        const mode = this.paramModes[paramNo] || 0
        if(mode === 0) {
            return this.setValueFromPointerAt(value, address)
        } else if(mode === 2) {
            return this.setValueRelativeFromPointerAt(value, address)
        }
        return this.setValueAt(value, address)
    }

    advanceCursor() {
        this.cursor = this.cursor + 1n + BigInt(this.paramModes.length)
    }

    advanceProgram() {
        const code = this.getValueAt(this.cursor).toString()
        this.opCode = Number(code.substr(code.length - 2))
        this.paramModes = code.padStart(5, '0')
            .substr(0, 3)
            .split('')
            .map(Number)
            .reverse()
            .slice(0, OP_CODE_PARAMS[this.opCode])
    }

    reset() {
        this.initializeMemory()
        this.cursor = 0n
        this.relativeBase = 0n
        this.running = true
        this.opCode = -1
        this.paramModes = []
    }

    initializeMemory() {
        this.memory = new Map()
        this.originInput.split(',').map(i => BigInt(i)).forEach((i, index) => {
            this.memory.set(BigInt(index), i)
        })
    }

    exec(input: bigint[], output: bigint[] = []) {
        this.advanceProgram()
        let result: number | bigint = Infinity
        this.running = true
        switch(this.opCode) {
            case OP_CODES.ADD:
                this.operator.add()
                break;
            case OP_CODES.MUL:
                this.operator.mul()
                break;
            case OP_CODES.JMP_TRUE:
                this.operator.jumpIfTrue()
                break
            case OP_CODES.JMP_FALSE:
                this.operator.jumpIfFalse()
                break
            case OP_CODES.LE:
                this.operator.lesserThan()
                break
            case OP_CODES.EQ:
                this.operator.equals()
                break
            case OP_CODES.ADJ:
                this.operator.adjust()
                break
            case OP_CODES.INPUT:
                if(input.length) {
                    this.operator.input(input.shift()!)
                } else {
                    this.running = false
                }
                break
            case OP_CODES.OUTPUT:
                result = this.operator.output()
                output.push(result)
                break
            case OP_CODES.HALT:
                this.running = false
                break;
            default:
                this.running = false
                break
        }
        return result
    }
}