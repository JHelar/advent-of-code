import { Console } from 'console'
import { relative } from 'path'
import { readFileWithSeparator } from '../utils'

const NOP = 'nop'
const ACC = 'acc'
const JMP = 'jmp'

const COMMANDS = {
    [NOP]: (program: Program, instruction: Instruction) => Object.assign<Program, Pick<Program, 'cursor'>>(program, {
        cursor: program.cursor + 1
    }),
    [ACC]: (program: Program, instruction: Instruction) => Object.assign<Program, Pick<Program, 'accumulator' | 'cursor'>>(program, {
        accumulator: program.accumulator + instruction.amountCount * instruction.amountNegator,
        cursor: program.cursor + 1
    }),
    [JMP]: (program: Program, instruction: Instruction) => Object.assign<Program, Pick<Program, 'cursor'>>(program, {
        cursor: program.cursor + instruction.amountCount * instruction.amountNegator
    }),
}


interface Program {
    cursor: number,
    accumulator: number
}

interface Instruction {
    command: keyof typeof COMMANDS,
    originCommand: keyof typeof COMMANDS,
    amountNegator: number,
    amountCount: number,
    callCount: number,
    tampered: boolean
}
const runProgram = (instructions: Instruction[]) => {
    let program: Program = {
        accumulator: 0,
        cursor: 0
    }

    const runInstruction = (instruction: Instruction) => {
        COMMANDS[instruction.command](program, instruction)
        instruction.callCount++
    }

    let instruction = instructions[0]
    while(program.cursor < instructions.length) {
        runInstruction(instruction)

        instruction = instructions[program.cursor]
        if(instruction && instruction.callCount > 0) return -1
    }
    return program.accumulator
}

const parseInstruction = (iString: string) => {
    const [command, amount] = iString.split(' ')
    const amountNegator = amount[0] === '+' ? 1 : -1
    const amountCount = parseInt(amount.slice(1))

    const instruction: Instruction = {
        command: command as keyof typeof COMMANDS,
        originCommand: command as keyof typeof COMMANDS,
        amountNegator,
        amountCount,
        callCount: 0,
        tampered: false
    }

    return instruction
}

export default () => {
    let instructions = readFileWithSeparator('day-8/input.txt', '\n').map(parseInstruction)
    
    let programResult = -1
    while(true) {
        programResult = runProgram(instructions)
        if(programResult === -1) {
            let didChange = false
            instructions = instructions.map((instruction) => {
                if((instruction.originCommand === JMP || instruction.originCommand === NOP) && !instruction.tampered && !didChange) {
                    didChange = true
                    return {
                        ...instruction,
                        command: instruction.originCommand === JMP ? NOP : JMP,
                        callCount: 0,
                        tampered: true
                    }
                } else if (instruction.tampered) {
                    return {
                        ...instruction,
                        command: instruction.originCommand,
                        callCount: 0
                    }
                }
                return {
                    ...instruction,
                    callCount: 0
                }
            })
        }
        if (programResult > 0) {
            return programResult
        }
    }
}